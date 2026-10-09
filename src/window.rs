use crate::{
    config::WindowBounds,
    error::{AppError, AppResult},
    state::AppState,
};
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Emitter, LogicalSize, Manager, PhysicalPosition};

pub fn native_error(_: impl std::fmt::Display) -> AppError {
    AppError::new("window", "窗口操作失败，请重试", false)
}

pub fn show(window: &tauri::WebviewWindow) -> AppResult<()> {
    window.unminimize().map_err(native_error)?;
    window.show().map_err(native_error)?;
    window.set_focus().map_err(native_error)
}
pub fn toggle_main(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) && !window.is_minimized().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = show(&window);
        }
    }
}

#[derive(Clone)]
pub struct WorkArea {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale: f64,
}

// Distances are logical pixels; positions and window sizes remain physical pixels.
pub fn snap_position(position: (i32, i32), size: (u32, u32), area: &WorkArea) -> (i32, i32) {
    fn axis(value: i32, start: i32, extent: u32, size: u32, scale: f64) -> i32 {
        let gap = (12.0 * scale).round() as i64;
        let threshold = (28.0 * scale).round() as i64;
        let low = i64::from(start);
        let high = low + i64::from(extent) - i64::from(size);
        let value = i64::from(value);
        if high - low < 2 * gap {
            return value as i32;
        }
        let left = (value - low).abs();
        let right = (value - high).abs();
        let target = if left <= threshold && left <= right {
            low + gap
        } else if right <= threshold {
            high - gap
        } else {
            value
        };
        target.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
    }
    if !area.scale.is_finite() || area.scale <= 0.0 {
        return position;
    }
    (
        axis(position.0, area.x, area.width, size.0, area.scale),
        axis(position.1, area.y, area.height, size.1, area.scale),
    )
}

fn dragging_button_down() -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
            GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON,
        };
        unsafe {
            GetAsyncKeyState(i32::from(VK_LBUTTON)) < 0
                || GetAsyncKeyState(i32::from(VK_RBUTTON)) < 0
        }
    }
    #[cfg(not(windows))]
    {
        false
    }
}

fn schedule_edge_snap(window: &tauri::Window) {
    if window.label() != "main" {
        return;
    }
    let generation = window
        .state::<AppState>()
        .snap_generation
        .fetch_add(1, Ordering::SeqCst)
        + 1;
    let app = window.app_handle().clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        loop {
            let state = app.state::<AppState>();
            if state.snap_generation.load(Ordering::SeqCst) != generation
                || state.quitting.load(Ordering::SeqCst)
                || state.expanded.load(Ordering::SeqCst)
            {
                return;
            }
            if !dragging_button_down() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        let Some(window) = app.get_webview_window("main") else {
            return;
        };
        if !window.is_visible().unwrap_or(false)
            || window.is_minimized().unwrap_or(true)
            || window.is_maximized().unwrap_or(true)
        {
            return;
        }
        let (Ok(position), Ok(size), Ok(Some(monitor))) = (
            window.outer_position(),
            window.outer_size(),
            window.current_monitor(),
        ) else {
            return;
        };
        let work = monitor.work_area();
        let next = snap_position(
            (position.x, position.y),
            (size.width, size.height),
            &WorkArea {
                x: work.position.x,
                y: work.position.y,
                width: work.size.width,
                height: work.size.height,
                scale: monitor.scale_factor(),
            },
        );
        // Recheck after native queries, so an old pending snap cannot overwrite a newer move.
        if app
            .state::<AppState>()
            .snap_generation
            .load(Ordering::SeqCst)
            == generation
            && next != (position.x, position.y)
        {
            let _ = window.set_position(PhysicalPosition::new(next.0, next.1));
        }
    });
}

pub fn clamp_physical_position(
    position: (i32, i32),
    size: (u32, u32),
    area: &WorkArea,
) -> (i32, i32) {
    let max_x = area
        .x
        .saturating_add(area.width.saturating_sub(size.0) as i32);
    let max_y = area
        .y
        .saturating_add(area.height.saturating_sub(size.1) as i32);
    (
        position.0.clamp(area.x, max_x.max(area.x)),
        position.1.clamp(area.y, max_y.max(area.y)),
    )
}

// 坐标使用物理像素、尺寸使用逻辑像素，以便跨 DPI 屏幕恢复后仍可读。
pub fn clamp_bounds(bounds: &WindowBounds, areas: &[WorkArea]) -> WindowBounds {
    let Some(area) = areas
        .iter()
        .find(|area| {
            bounds.x >= area.x
                && bounds.y >= area.y
                && (bounds.x as i64) < area.x as i64 + area.width as i64
                && (bounds.y as i64) < area.y as i64 + area.height as i64
        })
        .or_else(|| areas.first())
    else {
        return bounds.clone();
    };
    let width = bounds
        .width
        .clamp(320, ((area.width as f64 / area.scale) as u32).max(320));
    let height = bounds
        .height
        .clamp(420, ((area.height as f64 / area.scale) as u32).max(420));
    let max_x = area.x.saturating_add(
        area.width
            .saturating_sub((width as f64 * area.scale) as u32) as i32,
    );
    let max_y = area.y.saturating_add(
        area.height
            .saturating_sub((height as f64 * area.scale) as u32) as i32,
    );
    WindowBounds {
        x: bounds.x.clamp(area.x, max_x.max(area.x)),
        y: bounds.y.clamp(area.y, max_y.max(area.y)),
        width,
        height,
    }
}
fn areas(window: &tauri::WebviewWindow) -> Vec<WorkArea> {
    window
        .available_monitors()
        .unwrap_or_default()
        .iter()
        .map(|monitor| {
            let area = monitor.work_area();
            WorkArea {
                x: area.position.x,
                y: area.position.y,
                width: area.size.width,
                height: area.size.height,
                scale: monitor.scale_factor(),
            }
        })
        .collect()
}
pub fn apply_bounds(window: &tauri::WebviewWindow, bounds: &WindowBounds) -> AppResult<()> {
    let bounds = clamp_bounds(bounds, &areas(window));
    window
        .set_position(PhysicalPosition::new(bounds.x, bounds.y))
        .map_err(native_error)?;
    window
        .set_size(LogicalSize::new(bounds.width, bounds.height))
        .map_err(native_error)
}
pub fn current_bounds(window: &tauri::Window) -> Option<WindowBounds> {
    let scale = window.scale_factor().ok()?;
    let position = window.outer_position().ok()?;
    let size = window.inner_size().ok()?;
    Some(WindowBounds {
        x: position.x,
        y: position.y,
        width: (size.width as f64 / scale).round() as u32,
        height: (size.height as f64 / scale).round() as u32,
    })
}
pub fn apply_window_config(app: &tauri::App) -> AppResult<()> {
    let config = app.state::<AppState>().config.snapshot();
    if let Some(window) = app.get_webview_window("main") {
        apply_bounds(&window, &config.window_bounds)?;
        apply_appearance(
            app.handle(),
            &config.theme,
            config.opacity,
            config.glass_luminosity,
        )?;
        if config.minimize_on_start {
            window.hide().map_err(native_error)?;
        } else {
            show(&window)?;
        }
    }
    Ok(())
}
pub fn save_window_position(window: &tauri::Window) {
    if window.label() != "main" {
        return;
    }
    let state = window.state::<AppState>();
    if state.mini.load(Ordering::SeqCst)
        || state.expanded.load(Ordering::SeqCst)
        || window.is_minimized().unwrap_or(false)
    {
        return;
    }
    let Some(bounds) = current_bounds(window) else {
        return;
    };
    if bounds.width < 320 || bounds.height < 420 {
        return;
    }
    let generation = state.bounds_generation.fetch_add(1, Ordering::SeqCst) + 1;
    let app = window.app_handle().clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(350)).await;
        let state = app.state::<AppState>();
        if state.bounds_generation.load(Ordering::SeqCst) != generation
            || state.mini.load(Ordering::SeqCst)
            || state.expanded.load(Ordering::SeqCst)
        {
            return;
        }
        let store = state.config.clone();
        let _ = tauri::async_runtime::spawn_blocking(move || store.save_bounds(bounds)).await;
    });
}
pub fn open_settings(app: &AppHandle) -> AppResult<()> {
    let window = if let Some(window) = app.get_webview_window("settings") {
        window
    } else {
        tauri::WebviewWindowBuilder::new(
            app,
            "settings",
            tauri::WebviewUrl::App("settings.html".into()),
        )
        .title("Jin10 Reader · 设置")
        .inner_size(460.0, 680.0)
        .resizable(true)
        .min_inner_size(360.0, 480.0)
        .maximizable(false)
        .minimizable(false)
        .decorations(false)
        .transparent(true)
        .build()
        .map_err(native_error)?
    };
    let config = app.state::<AppState>().config.snapshot();
    set_window_appearance(
        &window,
        &config.theme,
        config.opacity,
        config.glass_luminosity,
    )?;
    show(&window)?;
    let _ = window.emit("settings-opened", ());
    Ok(())
}
pub fn handle_window_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    match event {
        tauri::WindowEvent::Moved(_) => {
            schedule_edge_snap(window);
            save_window_position(window);
        }
        tauri::WindowEvent::Resized(_) => save_window_position(window),
        tauri::WindowEvent::CloseRequested { api, .. } => {
            if window.state::<AppState>().quitting.load(Ordering::SeqCst) {
                return;
            }
            save_window_position(window);
            api.prevent_close();
            let _ = window.hide();
        }
        _ => {}
    }
}

pub fn set_mini(window: &tauri::WebviewWindow, enabled: bool) -> AppResult<()> {
    let state = window.state::<AppState>();
    if state.mini.load(Ordering::SeqCst) == enabled {
        return Ok(());
    }
    if enabled {
        if state.expanded.load(Ordering::SeqCst) {
            return Err(AppError::invalid("请先还原窗口再切换迷你模式"));
        }
        let bounds =
            current_bounds(&window.as_ref().window()).ok_or_else(|| native_error("bounds"))?;
        let monitor = window.current_monitor().map_err(native_error)?;
        *state
            .restore_bounds
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(bounds.clone());
        state.mini.store(true, Ordering::SeqCst);
        state.bounds_generation.fetch_add(1, Ordering::SeqCst);
        let result = (|| -> AppResult<()> {
            window
                .set_min_size(Some(LogicalSize::new(360, 240)))
                .map_err(native_error)?;
            window
                .set_size(LogicalSize::new(500, 260))
                .map_err(native_error)?;
            if let Some(monitor) = monitor {
                let area = monitor.work_area();
                let size = window.outer_size().map_err(native_error)?;
                let position = window.outer_position().map_err(native_error)?;
                let visible = clamp_physical_position(
                    (position.x, position.y),
                    (size.width, size.height),
                    &WorkArea {
                        x: area.position.x,
                        y: area.position.y,
                        width: area.size.width,
                        height: area.size.height,
                        scale: monitor.scale_factor(),
                    },
                );
                window
                    .set_position(PhysicalPosition::new(visible.0, visible.1))
                    .map_err(native_error)?;
            }
            Ok(())
        })();
        if let Err(error) = result {
            let _ = window.set_min_size(Some(LogicalSize::new(320, 420)));
            let _ = apply_bounds(window, &bounds);
            state.mini.store(false, Ordering::SeqCst);
            *state
                .restore_bounds
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = None;
            return Err(error);
        }
    } else {
        let bounds = state
            .restore_bounds
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .unwrap_or_else(|| state.config.snapshot().window_bounds);
        window
            .set_min_size(Some(LogicalSize::new(320, 420)))
            .map_err(native_error)?;
        apply_bounds(window, &bounds)?;
        state.mini.store(false, Ordering::SeqCst);
        *state
            .restore_bounds
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = None;
    }
    Ok(())
}

pub fn toggle_maximize(window: &tauri::WebviewWindow) -> AppResult<()> {
    let state = window.state::<AppState>();
    if state.mini.load(Ordering::SeqCst) {
        return Err(AppError::invalid("请先退出迷你模式"));
    }
    if state.expanded.load(Ordering::SeqCst) {
        let bounds = state
            .restore_bounds
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take()
            .unwrap_or_else(|| state.config.snapshot().window_bounds);
        apply_bounds(window, &bounds)?;
        state.expanded.store(false, Ordering::SeqCst);
    } else {
        let monitor = window
            .current_monitor()
            .map_err(native_error)?
            .or(window.primary_monitor().map_err(native_error)?)
            .ok_or_else(|| native_error("monitor"))?;
        let bounds =
            current_bounds(&window.as_ref().window()).ok_or_else(|| native_error("bounds"))?;
        *state
            .restore_bounds
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(bounds);
        state.expanded.store(true, Ordering::SeqCst);
        state.bounds_generation.fetch_add(1, Ordering::SeqCst);
        let area = monitor.work_area();
        window.set_position(area.position).map_err(native_error)?;
        window.set_size(area.size).map_err(native_error)?;
    }
    Ok(())
}

pub fn apply_appearance(
    app: &AppHandle,
    theme: &str,
    opacity: f64,
    luminosity: f64,
) -> AppResult<()> {
    for label in ["main", "settings"] {
        if let Some(window) = app.get_webview_window(label) {
            set_window_appearance(&window, theme, opacity, luminosity)?;
        }
    }
    Ok(())
}

pub fn set_window_appearance(
    window: &tauri::WebviewWindow,
    theme: &str,
    opacity: f64,
    luminosity: f64,
) -> AppResult<()> {
    if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
        return Err(AppError::invalid("毛玻璃不透明度应在 0% 到 100% 之间"));
    }
    clear_legacy_alpha(window)?;
    window
        .set_theme(match theme {
            "light" => Some(tauri::Theme::Light),
            "system" => None,
            _ => Some(tauri::Theme::Dark),
        })
        .map_err(native_error)?;
    crate::acrylic::apply(window, theme == "glass", luminosity)?;
    Ok(())
}

// Remove the former whole-window alpha: only the glass surface can be translucent.
#[cfg(windows)]
fn clear_legacy_alpha(window: &tauri::WebviewWindow) -> AppResult<()> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows_sys::Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            GWL_EXSTYLE, GetWindowLongPtrW, SetWindowLongPtrW, WS_EX_LAYERED,
        },
    };
    let handle = window.window_handle().map_err(native_error)?;
    let hwnd = match handle.as_raw() {
        RawWindowHandle::Win32(handle) => handle.hwnd.get() as HWND,
        _ => return Err(native_error("Win32")),
    };
    let ex = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) };
    if (ex as u32) & WS_EX_LAYERED != 0 {
        unsafe { SetWindowLongPtrW(hwnd, GWL_EXSTYLE, (ex as u32 & !WS_EX_LAYERED) as isize) };
        if (unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) } as u32) & WS_EX_LAYERED != 0 {
            return Err(native_error("opacity"));
        }
    }
    Ok(())
}
#[cfg(not(windows))]
fn clear_legacy_alpha(_window: &tauri::WebviewWindow) -> AppResult<()> {
    Ok(())
}
