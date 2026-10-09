use crate::{
    config::{ConfigPatch, McpConfig, PublicConfig, WindowBounds, normalize_endpoint},
    error::{AppError, AppResult},
    mcp,
    state::{AppState, Health},
};
use serde_json::{Value, json};
use std::sync::Arc;
use tauri::{Emitter, Manager, State};

fn require_window(window: &tauri::WebviewWindow, label: &str) -> AppResult<()> {
    if window.label() != label {
        return Err(AppError::new("forbidden", "该窗口无权执行此操作", false));
    }
    Ok(())
}
fn bounded_input(value: &str, label: &str, maximum: usize) -> AppResult<()> {
    if value.trim().is_empty() || value.len() > maximum || value.chars().any(char::is_control) {
        return Err(AppError::invalid(&format!("{label}为空、过长或含控制字符")));
    }
    Ok(())
}
fn validate_cursor(cursor: Option<&str>) -> AppResult<()> {
    if let Some(value) = cursor {
        bounded_input(value, "分页游标", 4096)?;
    }
    Ok(())
}
pub async fn emit_health(app: &tauri::AppHandle) {
    let health = app.state::<AppState>().health().await;
    let _ = app.emit("connection-state", health);
}
async fn client(app: &tauri::AppHandle, state: &AppState) -> AppResult<Arc<mcp::McpClient>> {
    let result = state.client(false).await;
    emit_health(app).await;
    result
}
async fn finish<T>(
    app: &tauri::AppHandle,
    state: &AppState,
    client: &Arc<mcp::McpClient>,
    result: AppResult<T>,
) -> AppResult<T> {
    let result = state.finish(client, result).await;
    emit_health(app).await;
    result
}

#[tauri::command]
pub async fn get_quotes(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    codes: Vec<String>,
) -> AppResult<Vec<Value>> {
    if codes.is_empty() || codes.len() > 12 {
        return Err(AppError::invalid("每次可查询 1 到 12 个品种"));
    }
    for code in &codes {
        bounded_input(code, "品种代码", 32)?;
    }
    let client = client(&app, &state).await?;
    let result = mcp::bounded(mcp::CALL_BUDGET, mcp::get_quotes(&client, &codes)).await;
    finish(&app, &state, &client, result).await
}
#[tauri::command]
pub async fn get_flash(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    cursor: Option<String>,
) -> AppResult<Value> {
    validate_cursor(cursor.as_deref())?;
    let client = client(&app, &state).await?;
    let result = mcp::list_flash(&client, cursor.as_deref()).await;
    finish(&app, &state, &client, result).await
}
#[tauri::command]
pub async fn get_news(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    cursor: Option<String>,
) -> AppResult<Value> {
    validate_cursor(cursor.as_deref())?;
    let client = client(&app, &state).await?;
    let result = mcp::list_news(&client, cursor.as_deref()).await;
    finish(&app, &state, &client, result).await
}
#[tauri::command]
pub async fn get_calendar(app: tauri::AppHandle, state: State<'_, AppState>) -> AppResult<Value> {
    let client = client(&app, &state).await?;
    let result = mcp::list_calendar(&client).await;
    finish(&app, &state, &client, result).await
}
#[tauri::command]
pub async fn search_flash(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    keyword: String,
    cursor: Option<String>,
) -> AppResult<Value> {
    bounded_input(&keyword, "搜索词", 200)?;
    validate_cursor(cursor.as_deref())?;
    let client = client(&app, &state).await?;
    let result = mcp::search_flash(&client, keyword.trim(), cursor.as_deref()).await;
    finish(&app, &state, &client, result).await
}
#[tauri::command]
pub async fn search_news(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    keyword: String,
    cursor: Option<String>,
    limit: Option<u32>,
) -> AppResult<Value> {
    bounded_input(&keyword, "搜索词", 200)?;
    validate_cursor(cursor.as_deref())?;
    if limit.is_some_and(|value| value == 0 || value > 100) {
        return Err(AppError::invalid("每页数量应为 1 到 100"));
    }
    let client = client(&app, &state).await?;
    let result = mcp::search_news(&client, keyword.trim(), cursor.as_deref(), limit).await;
    finish(&app, &state, &client, result).await
}
#[tauri::command]
pub async fn get_news_detail(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> AppResult<Value> {
    bounded_input(&id, "资讯 ID", 200)?;
    let client = client(&app, &state).await?;
    let result = mcp::get_news(&client, &id).await;
    finish(&app, &state, &client, result).await
}
#[tauri::command]
pub async fn health(app: tauri::AppHandle, state: State<'_, AppState>) -> AppResult<Health> {
    crate::acrylic::refresh(&app).await;
    Ok(state.health().await)
}
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> AppResult<PublicConfig> {
    Ok(state.config.public())
}

#[tauri::command]
pub async fn save_config(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    config: ConfigPatch,
) -> AppResult<Value> {
    require_window(&window, "settings")?;
    let store = state.config.clone();
    let before = store.snapshot().mcp;
    let native_app = app.clone();
    let updated = tauri::async_runtime::spawn_blocking(move || {
        let mut snapshot = None;
        store.update(
            |old| old.patched(config),
            |old, next| {
                if let Some(before) = snapshot.take() {
                    return crate::desktop::restore_snapshot(&native_app, before);
                }
                let before = crate::desktop::snapshot(&native_app, old, next)?;
                if let Err(error) = crate::desktop::apply_preferences(&native_app, old, next) {
                    let restored = crate::desktop::restore_snapshot(&native_app, before).is_ok();
                    return Err(crate::desktop::rollback_result(error, restored).unwrap_err());
                }
                snapshot = Some(before);
                Ok(())
            },
        )
    })
    .await
    .map_err(|_| AppError::storage("保存任务意外中断，原配置已保留"))??;
    let changed = before != updated.mcp;
    if changed {
        state.invalidate().await;
    }
    let public = state.config.public();
    if changed {
        let _ = app.emit("data-source-changed", &public);
    }
    let _ = app.emit("config-updated", &public);
    emit_health(&app).await;
    Ok(json!({"saved": true, "connectionChanged": changed, "config": public}))
}
#[tauri::command]
pub async fn set_theme(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    theme: String,
) -> AppResult<PublicConfig> {
    require_window(&window, "main")?;
    let store = state.config.clone();
    let native_app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        store.update(
            |old| {
                old.patched(ConfigPatch {
                    theme: Some(theme),
                    ..Default::default()
                })
            },
            |old, next| crate::desktop::apply_preferences(&native_app, old, next),
        )
    })
    .await
    .map_err(|_| AppError::storage("保存主题任务意外中断"))??;
    let config = state.config.public();
    let _ = app.emit("config-updated", &config);
    Ok(config)
}
#[tauri::command]
pub async fn test_mcp(
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    server_url: String,
    bearer_token: String,
) -> AppResult<Value> {
    require_window(&window, "settings")?;
    let endpoint = normalize_endpoint(&server_url)?;
    let stored = state.config.snapshot().mcp;
    let token = if bearer_token.trim().is_empty() && stored.server_url == endpoint {
        stored.bearer_token
    } else {
        bearer_token.trim().to_owned()
    };
    let client = mcp::connect(&McpConfig {
        server_url: endpoint,
        bearer_token: token,
    })
    .await?;
    let result = mcp::list_tools(&client).await;
    client.cancellation_token().cancel();
    Ok(json!({"ok": true, "tools": result?}))
}
#[tauri::command]
pub async fn restart_mcp(app: tauri::AppHandle) -> AppResult<Health> {
    restart_mcp_impl(&app).await
}
pub async fn restart_mcp_impl(app: &tauri::AppHandle) -> AppResult<Health> {
    let state = app.state::<AppState>();
    let result = state.client(true).await;
    emit_health(app).await;
    result?;
    let _ = app.emit("mcp-ready", ());
    Ok(state.health().await)
}

pub fn safe_external_url(url: &str) -> AppResult<String> {
    if url.len() > 4096 || url.chars().any(char::is_control) {
        return Err(AppError::invalid("链接格式无效"));
    }
    let parsed = reqwest::Url::parse(url).map_err(|_| AppError::invalid("链接格式无效"))?;
    if !matches!(parsed.scheme(), "http" | "https")
        || parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
    {
        return Err(AppError::invalid(
            "仅允许打开不含用户凭据的 HTTP 或 HTTPS 网页",
        ));
    }
    Ok(parsed.to_string())
}
#[tauri::command]
pub async fn open_external(app: tauri::AppHandle, url: String) -> AppResult<()> {
    use tauri_plugin_opener::OpenerExt;
    let url = safe_external_url(&url)?;
    tauri::async_runtime::spawn_blocking(move || {
        app.opener()
            .open_url(url, None::<&str>)
            .map_err(|_| AppError::new("open_url", "无法打开浏览器，请检查系统默认浏览器", false))
    })
    .await
    .map_err(|_| AppError::new("open_url", "打开链接任务中断", false))?
}
#[tauri::command]
pub async fn quit_app(app: tauri::AppHandle) {
    quit_impl(app).await;
}
pub async fn quit_impl(app: tauri::AppHandle) {
    let state = app.state::<AppState>();
    if let Some(window) = app.get_webview_window("main")
        && !state.mini.load(std::sync::atomic::Ordering::SeqCst)
        && !state.expanded.load(std::sync::atomic::Ordering::SeqCst)
        && !window.is_minimized().unwrap_or(false)
        && let Some(bounds) = crate::window::current_bounds(&window.as_ref().window())
    {
        let store = state.config.clone();
        let _ = tauri::async_runtime::spawn_blocking(move || store.save_bounds(bounds)).await;
    }
    state.shutdown().await;
    app.exit(0);
}
#[tauri::command]
pub async fn open_settings(app: tauri::AppHandle) -> AppResult<()> {
    crate::window::open_settings(&app)
}
#[tauri::command]
pub async fn reset_window_position(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> AppResult<()> {
    if state.mini.load(std::sync::atomic::Ordering::SeqCst)
        || state.expanded.load(std::sync::atomic::Ordering::SeqCst)
    {
        return Err(AppError::invalid("请先还原主窗口后再重置位置"));
    }
    let bounds = WindowBounds::default();
    if let Some(window) = app.get_webview_window("main") {
        crate::window::apply_bounds(&window, &bounds)?;
        crate::window::show(&window)?;
    }
    let store = state.config.clone();
    tauri::async_runtime::spawn_blocking(move || store.save_bounds(bounds))
        .await
        .map_err(|_| AppError::storage("窗口位置保存任务中断"))?
}
#[tauri::command]
pub fn toggle_maximize(window: tauri::WebviewWindow) -> AppResult<()> {
    require_window(&window, "main")?;
    crate::window::toggle_maximize(&window)
}
#[tauri::command]
pub fn set_mini_mode(window: tauri::WebviewWindow, enabled: bool) -> AppResult<()> {
    require_window(&window, "main")?;
    crate::window::set_mini(&window, enabled)
}
#[tauri::command]
pub fn set_window_size(window: tauri::WebviewWindow, width: u32, height: u32) -> AppResult<()> {
    require_window(&window, "main")?;
    if !(320..=5000).contains(&width) || !(240..=5000).contains(&height) {
        return Err(AppError::invalid("窗口尺寸超出可用范围"));
    }
    window
        .set_size(tauri::LogicalSize::new(width, height))
        .map_err(crate::window::native_error)
}
#[tauri::command]
pub async fn set_window_opacity(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    state: State<'_, AppState>,
    opacity: f64,
) -> AppResult<()> {
    require_window(&window, "settings")?;
    if state.config.snapshot().theme != "glass" {
        return Err(AppError::invalid(
            "墨色和浅色始终完全不透明；请切换到毛玻璃模式后调整",
        ));
    }
    save_config(
        app,
        window,
        state,
        ConfigPatch {
            opacity: Some(opacity),
            ..Default::default()
        },
    )
    .await
    .map(|_| ())
}
