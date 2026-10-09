use crate::{
    config::{ConfigStore, WindowBounds},
    error::{AppError, AppResult},
    mcp::{self, McpClient},
};
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex as StdMutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub process: &'static str,
    pub state: String,
    pub configured: bool,
    pub last_success_at: Option<u64>,
    pub last_error: Option<AppError>,
    pub config_warning: Option<String>,
    pub shortcut_status: ShortcutStatus,
    pub material: crate::acrylic::MaterialStatus,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutStatus {
    pub toggle_window: bool,
    pub refresh: bool,
}
struct Connection {
    client: Option<Arc<McpClient>>,
    state: String,
    last_success_at: Option<u64>,
    last_error: Option<AppError>,
    retry_at: Option<Instant>,
    failures: u32,
}
pub struct AppState {
    pub config: Arc<ConfigStore>,
    connection: Mutex<Connection>,
    connecting: Mutex<()>,
    generation: AtomicU64,
    pub quitting: AtomicBool,
    pub bounds_generation: AtomicU64,
    pub snap_generation: AtomicU64,
    pub mini: AtomicBool,
    pub expanded: AtomicBool,
    pub restore_bounds: StdMutex<Option<WindowBounds>>,
    pub shortcuts: StdMutex<HashMap<u32, (tauri_plugin_global_shortcut::Shortcut, &'static str)>>,
    pub desktop_warning: StdMutex<Option<String>>,
    pub material_status: StdMutex<crate::acrylic::MaterialStatus>,
}
impl AppState {
    pub fn new(config: Arc<ConfigStore>) -> Self {
        let configured = !config.snapshot().mcp.bearer_token.is_empty();
        Self {
            config,
            connection: Mutex::new(Connection {
                client: None,
                state: if configured {
                    "offline"
                } else {
                    "unconfigured"
                }
                .into(),
                last_success_at: None,
                last_error: None,
                retry_at: None,
                failures: 0,
            }),
            connecting: Mutex::new(()),
            generation: AtomicU64::new(0),
            quitting: AtomicBool::new(false),
            bounds_generation: AtomicU64::new(0),
            snap_generation: AtomicU64::new(0),
            mini: AtomicBool::new(false),
            expanded: AtomicBool::new(false),
            restore_bounds: StdMutex::new(None),
            shortcuts: StdMutex::new(HashMap::new()),
            desktop_warning: StdMutex::new(None),
            material_status: StdMutex::new(crate::acrylic::MaterialStatus::default()),
        }
    }
    pub async fn health(&self) -> Health {
        let public = self.config.public();
        let shortcut_status = {
            let bindings = self.shortcuts.lock().unwrap_or_else(|e| e.into_inner());
            let active = |value: &str, role| {
                value
                    .parse::<tauri_plugin_global_shortcut::Shortcut>()
                    .ok()
                    .is_some_and(|key| {
                        bindings
                            .get(&key.id())
                            .is_some_and(|(_, action)| *action == role)
                    })
            };
            ShortcutStatus {
                toggle_window: active(&public.shortcuts.toggle_window, "toggle"),
                refresh: active(&public.shortcuts.refresh, "refresh"),
            }
        };
        let conn = self.connection.lock().await;
        Health {
            process: "ok",
            state: conn.state.clone(),
            configured: public.mcp.has_token,
            last_success_at: conn.last_success_at,
            last_error: conn.last_error.clone(),
            config_warning: public.config_warning.or_else(|| {
                self.desktop_warning
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .clone()
            }),
            shortcut_status,
            material: self
                .material_status
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone(),
        }
    }
    pub async fn invalidate(&self) {
        self.generation.fetch_add(1, Ordering::SeqCst);
        let configured = !self.config.snapshot().mcp.bearer_token.is_empty();
        let mut conn = self.connection.lock().await;
        if let Some(client) = conn.client.take() {
            client.cancellation_token().cancel();
        }
        conn.state = if configured {
            "offline"
        } else {
            "unconfigured"
        }
        .into();
        conn.last_error = None;
        conn.retry_at = None;
        conn.failures = 0;
        conn.last_success_at = None;
    }
    pub async fn client(&self, force: bool) -> AppResult<Arc<McpClient>> {
        // 只序列化握手，健康读取和业务请求不争用该锁。
        let _connecting = self.connecting.lock().await;
        let generation = self.generation.load(Ordering::SeqCst);
        let cfg = self.config.snapshot().mcp;
        if cfg.bearer_token.is_empty() {
            return Err(AppError::unconfigured());
        }
        {
            let mut conn = self.connection.lock().await;
            if !force {
                if matches!(conn.state.as_str(), "auth_failed" | "rate_limited") {
                    return Err(conn.last_error.clone().unwrap_or_else(AppError::auth));
                }
                if let Some(client) = &conn.client
                    && !client.is_closed()
                {
                    return Ok(client.clone());
                }
                if conn.retry_at.is_some_and(|time| time > Instant::now()) {
                    return Err(conn.last_error.clone().unwrap_or_else(AppError::network));
                }
            }
            if force && let Some(old) = conn.client.take() {
                old.cancellation_token().cancel();
            }
            conn.state = "connecting".into();
        }
        let result = mcp::connect(&cfg).await;
        let mut conn = self.connection.lock().await;
        if generation != self.generation.load(Ordering::SeqCst) {
            if let Ok(client) = result {
                client.cancellation_token().cancel();
            }
            return Err(AppError::stale());
        }
        match result {
            Ok(client) => {
                let client = Arc::new(client);
                if let Some(old) = conn.client.replace(client.clone()) {
                    old.cancellation_token().cancel();
                }
                conn.state = "connected".into();
                conn.last_error = None;
                conn.retry_at = None;
                conn.failures = 0;
                Ok(client)
            }
            Err(error) => {
                record_error(&mut conn, &error);
                Err(error)
            }
        }
    }
    pub async fn finish<T>(&self, client: &Arc<McpClient>, result: AppResult<T>) -> AppResult<T> {
        let mut conn = self.connection.lock().await;
        if !conn
            .client
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, client))
        {
            return Err(AppError::stale());
        }
        match &result {
            Ok(_) => {
                conn.last_success_at = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64,
                );
                if !matches!(conn.state.as_str(), "rate_limited" | "auth_failed") {
                    conn.state = "connected".into();
                    conn.last_error = None;
                }
            }
            Err(error) => {
                if error.retryable
                    && let Some(old) = conn.client.take()
                {
                    old.cancellation_token().cancel();
                }
                record_error(&mut conn, error);
            }
        }
        result
    }
    pub async fn shutdown(&self) {
        self.quitting.store(true, Ordering::SeqCst);
        self.invalidate().await;
    }
}
fn record_error(conn: &mut Connection, error: &AppError) {
    if matches!(conn.state.as_str(), "auth_failed" | "rate_limited")
        && !matches!(error.code.as_str(), "auth_failed" | "rate_limited")
    {
        return;
    }
    conn.state = match error.code.as_str() {
        "auth_failed" => "auth_failed",
        "rate_limited" => "rate_limited",
        "unconfigured" => "unconfigured",
        "offline" | "timeout" => "offline",
        _ => "error",
    }
    .into();
    conn.last_error = Some(error.clone());
    if error.retryable {
        conn.failures = conn.failures.saturating_add(1);
        conn.retry_at =
            Some(Instant::now() + Duration::from_secs((1u64 << conn.failures.min(6)).min(60)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn disconnected() -> Connection {
        Connection {
            client: None,
            state: "offline".into(),
            last_success_at: None,
            last_error: None,
            retry_at: None,
            failures: 0,
        }
    }
    #[test]
    fn authentication_and_quota_pause_survive_late_parallel_errors() {
        for error in [AppError::auth(), AppError::quota()] {
            let mut conn = disconnected();
            record_error(&mut conn, &error);
            record_error(&mut conn, &AppError::new("upstream", "业务失败", false));
            assert_eq!(conn.state, error.code);
            assert_eq!(conn.last_error.unwrap().code, error.code);
            assert!(conn.retry_at.is_none());
        }
    }
    #[test]
    fn only_connection_errors_schedule_bounded_backoff() {
        let mut conn = disconnected();
        record_error(&mut conn, &AppError::new("upstream", "业务失败", false));
        assert!(conn.retry_at.is_none());
        for _ in 0..20 {
            record_error(&mut conn, &AppError::network());
        }
        let remaining = conn
            .retry_at
            .unwrap()
            .saturating_duration_since(Instant::now());
        assert!(remaining <= Duration::from_secs(60));
        assert!(remaining > Duration::from_secs(59));
    }
}
