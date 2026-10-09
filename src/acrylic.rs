use crate::{
    error::{AppError, AppResult},
    state::AppState,
};
use serde::Serialize;
use tauri::Manager;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialStatus {
    pub engine: String,
    pub luminosity: Option<f64>,
    pub state: Option<i32>,
    pub warning: Option<String>,
}
impl Default for MaterialStatus {
    fn default() -> Self {
        Self {
            engine: "solid".into(),
            luminosity: None,
            state: None,
            warning: None,
        }
    }
}

#[cfg(windows)]
mod native {
    use super::*;
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use std::{cell::RefCell, collections::HashMap, sync::OnceLock};
    use windows_sys::Win32::Foundation::FreeLibrary;
    use windows_sys::Win32::System::{LibraryLoader::*, Threading::GetCurrentThreadId};
    static UI_THREAD: OnceLock<u32> = OnceLock::new();
    type Create = unsafe extern "system" fn(isize, *mut usize) -> i32;
    type Set = unsafe extern "system" fn(usize, f32, f32) -> i32;
    type Info = unsafe extern "system" fn(usize, *mut u8, i32) -> i32;
    type Close = unsafe extern "system" fn(usize);
    struct Api {
        _module: usize,
        create: Create,
        set: Set,
        info: Info,
        close: Close,
    }
    #[derive(Default)]
    struct Session {
        api: Option<Api>,
        contexts: HashMap<String, usize>,
    }
    thread_local! {static SESSION:RefCell<Session>=RefCell::new(Session::default());}
    pub fn initialize_ui() {
        let _ = UI_THREAD.set(unsafe { GetCurrentThreadId() });
    }
    fn on_ui() -> bool {
        UI_THREAD
            .get()
            .is_some_and(|id| *id == unsafe { GetCurrentThreadId() })
    }
    fn failure() -> String {
        "可调毛玻璃运行环境不可用，已回退系统预设。请安装 Windows App Runtime 2.5.1（x64）后重启应用。".into()
    }
    fn load(window: &tauri::WebviewWindow) -> Result<Api, String> {
        #[cfg(debug_assertions)]
        if std::env::var_os("JIN10_READER_DISABLE_ACRYLIC").is_some() {
            return Err(failure());
        }
        let base = window
            .app_handle()
            .path()
            .resource_dir()
            .map_err(|_| failure())?
            .join("resources/acrylic");
        #[cfg(debug_assertions)]
        let base = {
            // Cargo/Tauri may leave an older resource copy beside the debug EXE.
            let source =
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/acrylic");
            if source.join("official-acrylic.dll").is_file() {
                source
            } else {
                base
            }
        };
        let path = base
            .join("official-acrylic.dll")
            .to_string_lossy()
            .replace('/', "\\");
        let path = path.strip_prefix("\\\\?\\").unwrap_or(&path);
        let wide: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
        let module = unsafe {
            LoadLibraryExW(
                wide.as_ptr(),
                std::ptr::null_mut(),
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
            )
        };
        if module.is_null() {
            return Err(failure());
        }
        let find =
            |name: &[u8]| unsafe { GetProcAddress(module, name.as_ptr()) }.ok_or_else(failure);
        let api = (|| {
            Ok::<_, String>(Api {
                _module: module as usize,
                create: unsafe {
                    std::mem::transmute::<unsafe extern "system" fn() -> isize, Create>(find(
                        b"OfficialCreate\0",
                    )?)
                },
                set: unsafe {
                    std::mem::transmute::<unsafe extern "system" fn() -> isize, Set>(find(
                        b"OfficialSet\0",
                    )?)
                },
                info: unsafe {
                    std::mem::transmute::<unsafe extern "system" fn() -> isize, Info>(find(
                        b"OfficialInfo\0",
                    )?)
                },
                close: unsafe {
                    std::mem::transmute::<unsafe extern "system" fn() -> isize, Close>(find(
                        b"OfficialClose\0",
                    )?)
                },
            })
        })();
        if api.is_err() {
            unsafe { FreeLibrary(module) };
        }
        api
    }
    fn apply_ui(window: &tauri::WebviewWindow, glass: bool, luminosity: f64) -> AppResult<()> {
        let status = SESSION.with(|cell| {
            let mut session = cell.borrow_mut();
            if !glass || !session.contexts.contains_key(window.label()) {
                window
                    .set_effects(None::<tauri::utils::config::WindowEffectsConfig>)
                    .map_err(crate::window::native_error)?;
            }
            if !glass {
                if let Some(ctx) = session.contexts.remove(window.label())
                    && let Some(api) = &session.api
                {
                    unsafe { (api.close)(ctx) };
                }
                return Ok::<_, AppError>(MaterialStatus::default());
            }
            let outcome = (|| {
                if session.api.is_none() {
                    session.api = Some(load(window)?);
                }
                let api = session.api.as_ref().ok_or_else(failure)?;
                let ctx = if let Some(ctx) = session.contexts.get(window.label()) {
                    *ctx
                } else {
                    let handle = window.window_handle().map_err(|_| failure())?;
                    let hwnd = match handle.as_raw() {
                        RawWindowHandle::Win32(value) => value.hwnd.get(),
                        _ => return Err(failure()),
                    };
                    let mut ctx = 0;
                    let code = unsafe { (api.create)(hwnd, &mut ctx) };
                    if code < 0 || ctx == 0 {
                        return Err(failure());
                    }
                    session.contexts.insert(window.label().to_owned(), ctx);
                    ctx
                };
                let api = session.api.as_ref().ok_or_else(failure)?;
                if unsafe { (api.set)(ctx, 0.0, luminosity as f32) } < 0 {
                    return Err(failure());
                }
                let mut bytes = [0u8; 1024];
                if unsafe { (api.info)(ctx, bytes.as_mut_ptr(), 1024) } < 0 {
                    return Err(failure());
                }
                let end = bytes
                    .iter()
                    .position(|value| *value == 0)
                    .ok_or_else(failure)?;
                let value: serde_json::Value =
                    serde_json::from_slice(&bytes[..end]).map_err(|_| failure())?;
                let state = value["state"].as_i64().unwrap_or(1) as i32;
                Ok(MaterialStatus {
                    engine: "official".into(),
                    luminosity: value["luminosityOpacity"].as_f64(),
                    state: Some(state),
                    warning: (state != 0).then(|| {
                        "系统策略暂停了毛玻璃效果，当前显示实色；亮度层设置已保留。".into()
                    }),
                })
            })();
            match outcome {
                Ok(status) => Ok(status),
                Err(warning) => {
                    if let Some(ctx) = session.contexts.remove(window.label())
                        && let Some(api) = &session.api
                    {
                        unsafe { (api.close)(ctx) };
                    }
                    window
                        .set_effects(
                            tauri::window::EffectsBuilder::new()
                                .effect(tauri::window::Effect::Acrylic)
                                .build(),
                        )
                        .map_err(crate::window::native_error)?;
                    Ok(MaterialStatus {
                        engine: "preset".into(),
                        luminosity: None,
                        state: None,
                        warning: Some(warning),
                    })
                }
            }
        })?;
        if window.label() == "main" {
            *window
                .state::<AppState>()
                .material_status
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = status;
        }
        Ok(())
    }
    pub fn apply(window: &tauri::WebviewWindow, glass: bool, luminosity: f64) -> AppResult<()> {
        if on_ui() {
            return apply_ui(window, glass, luminosity);
        }
        let (send, receive) = std::sync::mpsc::channel();
        let handle = window.clone();
        window
            .run_on_main_thread(move || {
                let _ = send.send(apply_ui(&handle, glass, luminosity));
            })
            .map_err(crate::window::native_error)?;
        receive
            .recv_timeout(std::time::Duration::from_secs(10))
            .map_err(|_| AppError::new("appearance", "外观应用超时，请重启后重试", false))?
    }
    pub fn shutdown() {
        SESSION.with(|cell| {
            let mut session = cell.borrow_mut();
            let contexts = std::mem::take(&mut session.contexts);
            if let Some(api) = &session.api {
                for ctx in contexts.into_values() {
                    unsafe { (api.close)(ctx) };
                }
            }
        });
    }
    pub fn refresh_ui(app: &tauri::AppHandle) {
        SESSION.with(|cell| {
            let session = cell.borrow();
            let (Some(api), Some(ctx)) = (&session.api, session.contexts.get("main")) else {
                return;
            };
            let mut bytes = [0u8; 1024];
            if unsafe { (api.info)(*ctx, bytes.as_mut_ptr(), 1024) } < 0 {
                return;
            }
            let Some(end) = bytes.iter().position(|value| *value == 0) else {
                return;
            };
            let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes[..end]) else {
                return;
            };
            let mut status = app
                .state::<AppState>()
                .material_status
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone();
            status.luminosity = value["luminosityOpacity"].as_f64();
            status.state = value["state"].as_i64().map(|v| v as i32);
            status.warning = (status.state != Some(0))
                .then(|| "系统策略暂停了毛玻璃效果，当前显示实色；亮度层设置已保留。".into());
            *app.state::<AppState>()
                .material_status
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = status;
        });
    }
}

pub async fn refresh(app: &tauri::AppHandle) {
    #[cfg(windows)]
    {
        let (send, receive) = tokio::sync::oneshot::channel();
        let handle = app.clone();
        if app
            .run_on_main_thread(move || {
                native::refresh_ui(&handle);
                let _ = send.send(());
            })
            .is_ok()
        {
            let _ = tokio::time::timeout(std::time::Duration::from_secs(2), receive).await;
        }
    }
    #[cfg(not(windows))]
    let _ = app;
}

pub fn initialize_ui() {
    #[cfg(windows)]
    native::initialize_ui();
}
pub fn shutdown() {
    #[cfg(windows)]
    native::shutdown();
}
pub fn apply(window: &tauri::WebviewWindow, glass: bool, luminosity: f64) -> AppResult<()> {
    if !luminosity.is_finite() || !(0.0..=1.0).contains(&luminosity) {
        return Err(AppError::invalid(
            "毛玻璃亮度层不透明度应在 0% 到 100% 之间",
        ));
    }
    #[cfg(windows)]
    {
        native::apply(window, glass, luminosity)
    }
    #[cfg(not(windows))]
    {
        let _ = (window, glass, luminosity);
        Ok(())
    }
}
