use tauri::{
    Emitter,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
pub fn init(app: &tauri::App) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "显示 / 隐藏", true, None::<&str>)?;
    let refresh = MenuItem::with_id(app, "refresh", "刷新数据", true, None::<&str>)?;
    let reconnect = MenuItem::with_id(app, "reconnect", "重新连接", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&show, &refresh, &reconnect, &settings, &separator, &quit],
    )?;
    let mut tray = TrayIconBuilder::new()
        .tooltip("Jin10 Reader")
        .menu(&menu)
        .show_menu_on_left_click(false);
    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }
    tray.on_menu_event(|app, event| match event.id.as_ref() {
        "show" => crate::window::toggle_main(app),
        "refresh" => {
            let _ = app.emit_to("main", "refresh-all", ());
        }
        "reconnect" => {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                let _ = crate::commands::restart_mcp_impl(&app).await;
            });
        }
        "settings" => {
            let app = app.clone();
            tauri::async_runtime::spawn_blocking(move || crate::window::open_settings(&app));
        }
        "quit" => {
            let app = app.clone();
            tauri::async_runtime::spawn(crate::commands::quit_impl(app));
        }
        _ => {}
    })
    .on_tray_icon_event(|tray, event| {
        if matches!(
            event,
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            }
        ) {
            crate::window::toggle_main(tray.app_handle());
        }
    })
    .build(app)?;
    Ok(())
}
