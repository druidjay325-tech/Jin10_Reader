use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};

pub const NAMESPACE: &str = "jin10-reader";

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppConfig {
    pub schema_version: u32,
    pub auto_start: bool,
    pub minimize_on_start: bool,
    pub theme: String,
    pub opacity: f64,
    pub glass_luminosity: f64,
    pub shortcuts: Shortcuts,
    pub window_bounds: WindowBounds,
    pub reminders: Vec<Value>,
    pub mcp: McpConfig,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Shortcuts {
    pub toggle_window: String,
    pub refresh: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

// 故意不实现 Debug，避免诊断输出携带凭据。
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct McpConfig {
    pub server_url: String,
    pub bearer_token: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            schema_version: 1,
            auto_start: false,
            minimize_on_start: false,
            theme: "dark".into(),
            opacity: 0.95,
            glass_luminosity: 0.0,
            shortcuts: Shortcuts::default(),
            window_bounds: WindowBounds::default(),
            reminders: vec![],
            mcp: McpConfig::default(),
            extra: Map::new(),
        }
    }
}
impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            toggle_window: "Ctrl+Alt+J".into(),
            refresh: "Ctrl+Alt+R".into(),
        }
    }
}
impl Default for WindowBounds {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 400,
            height: 600,
        }
    }
}
impl Default for McpConfig {
    fn default() -> Self {
        Self {
            server_url: "https://mcp.jin10.com/mcp".into(),
            bearer_token: String::new(),
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicConfig {
    pub auto_start: bool,
    pub minimize_on_start: bool,
    pub theme: String,
    pub opacity: f64,
    pub glass_luminosity: f64,
    pub shortcuts: Shortcuts,
    pub window_bounds: WindowBounds,
    pub mcp: PublicMcpConfig,
    pub config_warning: Option<String>,
    pub config_read_only: bool,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicMcpConfig {
    pub server_url: String,
    pub has_token: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigPatch {
    pub auto_start: Option<bool>,
    pub minimize_on_start: Option<bool>,
    pub theme: Option<String>,
    pub opacity: Option<f64>,
    pub glass_luminosity: Option<f64>,
    pub shortcuts: Option<ShortcutsPatch>,
    pub mcp: Option<McpPatch>,
}
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShortcutsPatch {
    pub toggle_window: Option<String>,
    pub refresh: Option<String>,
}
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct McpPatch {
    pub server_url: Option<String>,
    pub bearer_token: Option<String>,
    #[serde(default)]
    pub clear_token: bool,
}

impl AppConfig {
    pub fn public(&self, warning: Option<String>) -> PublicConfig {
        PublicConfig {
            auto_start: self.auto_start,
            minimize_on_start: self.minimize_on_start,
            theme: self.theme.clone(),
            opacity: self.opacity,
            glass_luminosity: self.glass_luminosity,
            shortcuts: self.shortcuts.clone(),
            window_bounds: self.window_bounds.clone(),
            mcp: PublicMcpConfig {
                server_url: self.mcp.server_url.clone(),
                has_token: !self.mcp.bearer_token.is_empty(),
            },
            config_warning: warning,
            config_read_only: false,
        }
    }
    pub fn patched(&self, patch: ConfigPatch) -> AppResult<Self> {
        let mut next = self.clone();
        if let Some(value) = patch.auto_start {
            next.auto_start = value;
        }
        if let Some(value) = patch.minimize_on_start {
            next.minimize_on_start = value;
        }
        if let Some(value) = patch.theme {
            next.theme = value;
        }
        if let Some(value) = patch.opacity {
            next.opacity = value;
        }
        if let Some(value) = patch.glass_luminosity {
            next.glass_luminosity = value;
        }
        if let Some(value) = patch.shortcuts {
            if let Some(toggle) = value.toggle_window {
                next.shortcuts.toggle_window = toggle;
            }
            if let Some(refresh) = value.refresh {
                next.shortcuts.refresh = refresh;
            }
        }
        if let Some(mcp) = patch.mcp {
            if let Some(url) = mcp.server_url {
                next.mcp.server_url = normalize_endpoint(&url)?;
            }
            let token = mcp.bearer_token.unwrap_or_default().trim().to_owned();
            if mcp.clear_token && !token.is_empty() {
                return Err(AppError::invalid("不能同时清除凭据和写入新凭据"));
            }
            if mcp.clear_token {
                next.mcp.bearer_token.clear();
            } else if !token.is_empty() {
                validate_token(&token)?;
                next.mcp.bearer_token = token;
            } else if next.mcp.server_url != self.mcp.server_url
                && !self.mcp.bearer_token.is_empty()
            {
                return Err(AppError::invalid(
                    "更换服务地址时，请提供新凭据或明确清除已保存凭据",
                ));
            }
        }
        next.validate()?;
        Ok(next)
    }
    pub fn validate(&self) -> AppResult<()> {
        if self.schema_version != 1 {
            return Err(AppError::invalid(
                "配置版本不受支持，请使用与配置匹配的应用版本",
            ));
        }
        if !["dark", "light", "system", "glass"].contains(&self.theme.as_str()) {
            return Err(AppError::invalid("主题必须为墨色、浅色、跟随系统或毛玻璃"));
        }
        if !self.opacity.is_finite() || !(0.0..=1.0).contains(&self.opacity) {
            return Err(AppError::invalid("毛玻璃不透明度应在 0% 到 100% 之间"));
        }
        if !self.glass_luminosity.is_finite() || !(0.0..=1.0).contains(&self.glass_luminosity) {
            return Err(AppError::invalid(
                "毛玻璃亮度层不透明度应在 0% 到 100% 之间",
            ));
        }
        normalize_endpoint(&self.mcp.server_url)?;
        validate_token(&self.mcp.bearer_token)?;
        crate::desktop::parse_shortcuts(&self.shortcuts)?;
        Ok(())
    }
}

pub fn normalize_endpoint(input: &str) -> AppResult<String> {
    let url = reqwest::Url::parse(input.trim())
        .map_err(|_| AppError::invalid("请输入有效的 HTTPS 服务地址"))?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(AppError::invalid(
            "服务地址必须为 HTTPS，且不能包含用户名、密码、查询参数或片段",
        ));
    }
    Ok(url.to_string())
}
pub fn validate_token(token: &str) -> AppResult<()> {
    if token.len() > 8192 || token.chars().any(char::is_control) {
        return Err(AppError::invalid("凭据格式无效，请检查是否包含换行"));
    }
    Ok(())
}
pub fn data_dir() -> AppResult<PathBuf> {
    dirs::config_dir()
        .map(|path| path.join(NAMESPACE))
        .ok_or_else(|| AppError::storage("无法确定用户配置目录"))
}

struct StoredConfig {
    value: AppConfig,
    warning: Option<String>,
    read_only: bool,
}
pub struct ConfigStore {
    path: PathBuf,
    inner: Mutex<StoredConfig>,
}
impl ConfigStore {
    pub fn apply_current(&self, apply: impl FnOnce(&AppConfig) -> AppResult<()>) -> AppResult<()> {
        let stored = self.inner.lock().unwrap_or_else(|error| error.into_inner());
        if stored.read_only {
            return Ok(());
        }
        apply(&stored.value)
    }
    pub fn open(path: PathBuf) -> Self {
        let mut warning = None;
        let mut read_only = false;
        let value = match fs::read_to_string(&path) {
            Ok(text) => match serde_json::from_str::<AppConfig>(&text)
                .ok()
                .filter(|cfg| cfg.validate().is_ok())
            {
                Some(cfg) => cfg,
                None => {
                    warning = Some("配置文件损坏或版本不兼容。原文件已保留，当前使用临时默认值；请关闭应用后从 settings.json.bak 恢复，或备份后移走损坏文件。".into());
                    read_only = true;
                    AppConfig::default()
                }
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if path.parent().is_some_and(Path::is_file) {
                    warning = Some(
                        "配置目录被同名文件占用，保存已暂停；请保留原文件并修复目录后重启。".into(),
                    );
                    read_only = true;
                    AppConfig::default()
                } else {
                    match fs::read_to_string(backup_path(&path)) {
                        Ok(text) => match serde_json::from_str::<AppConfig>(&text)
                            .ok()
                            .filter(|cfg| cfg.validate().is_ok())
                        {
                            Some(cfg) => {
                                warning = Some(
                                    "主配置缺失，已读取上一次备份；下次保存将重建主配置。".into(),
                                );
                                cfg
                            }
                            None => {
                                warning = Some("主配置缺失且备份损坏或版本不兼容。备份已保留，保存已暂停；请先备份并修复配置。".into());
                                read_only = true;
                                AppConfig::default()
                            }
                        },
                        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                            AppConfig::default()
                        }
                        Err(_) => {
                            warning = Some("主配置缺失且备份无法读取。备份未改动，保存已暂停；请检查文件权限。".into());
                            read_only = true;
                            AppConfig::default()
                        }
                    }
                }
            }
            Err(_) => {
                warning = Some(
                    "无法读取配置。原文件未改动，保存已暂停；请检查当前用户的文件权限。".into(),
                );
                read_only = true;
                AppConfig::default()
            }
        };
        Self {
            path,
            inner: Mutex::new(StoredConfig {
                value,
                warning,
                read_only,
            }),
        }
    }
    pub fn snapshot(&self) -> AppConfig {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .value
            .clone()
    }
    pub fn public(&self) -> PublicConfig {
        let stored = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let mut config = stored.value.public(stored.warning.clone());
        config.config_read_only = stored.read_only;
        config
    }
    // 设置、窗口记忆共用此锁，写入时只修改各自拥有的字段。
    pub fn update<F, A>(&self, mutate: F, mut apply: A) -> AppResult<AppConfig>
    where
        F: FnOnce(&AppConfig) -> AppResult<AppConfig>,
        A: FnMut(&AppConfig, &AppConfig) -> AppResult<()>,
    {
        let mut stored = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if stored.read_only {
            return Err(AppError::storage(
                stored.warning.as_deref().unwrap_or("配置当前只读"),
            ));
        }
        let next = mutate(&stored.value)?;
        next.validate()?;
        apply(&stored.value, &next)?;
        if atomic_write(&self.path, &next).is_err() {
            let rolled_back = apply(&next, &stored.value).is_ok();
            return Err(AppError::storage(if rolled_back {
                "配置保存失败，系统设置已回滚，原配置仍有效"
            } else {
                "配置保存失败，部分系统设置未能回滚，请重新检查自启与快捷键"
            }));
        }
        stored.value = next.clone();
        stored.warning = None;
        Ok(next)
    }
    pub fn save_bounds(&self, bounds: WindowBounds) -> AppResult<()> {
        self.update(
            |old| {
                let mut next = old.clone();
                next.window_bounds = bounds;
                Ok(next)
            },
            |_, _| Ok(()),
        )
        .map(|_| ())
    }
}
fn backup_path(path: &Path) -> PathBuf {
    path.with_extension("json.bak")
}
fn atomic_write(path: &Path, value: &AppConfig) -> AppResult<()> {
    let parent = path
        .parent()
        .ok_or_else(|| AppError::storage("配置路径无效"))?;
    fs::create_dir_all(parent).map_err(|_| AppError::storage("无法创建配置目录"))?;
    let temporary = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(value).map_err(|_| AppError::storage("无法编码配置"))?;
    let mut file =
        fs::File::create(&temporary).map_err(|_| AppError::storage("无法创建临时配置"))?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|_| AppError::storage("无法同步临时配置"))?;
    drop(file);
    if path.exists() {
        let backup_temp = path.with_extension("json.bak.tmp");
        fs::copy(path, &backup_temp).map_err(|_| AppError::storage("无法备份原配置"))?;
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&backup_temp)
            .and_then(|f| f.sync_all())
            .map_err(|_| AppError::storage("无法同步配置备份"))?;
        replace_file(&backup_temp, &backup_path(path))?;
    }
    replace_file(&temporary, path)
}
#[cfg(windows)]
fn replace_file(from: &Path, to: &Path) -> AppResult<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };
    let from: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
    let to: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();
    if unsafe {
        MoveFileExW(
            from.as_ptr(),
            to.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    } == 0
    {
        return Err(AppError::storage("原子替换配置失败，原文件与备份已保留"));
    }
    Ok(())
}
#[cfg(not(windows))]
fn replace_file(from: &Path, to: &Path) -> AppResult<()> {
    fs::rename(from, to).map_err(|_| AppError::storage("原子替换配置失败"))?;
    if let Some(parent) = to.parent() {
        fs::File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|_| AppError::storage("配置目录同步失败"))?;
    }
    Ok(())
}
