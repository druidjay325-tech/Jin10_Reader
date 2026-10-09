fn main() {
    // 显式登记应用命令，让 capabilities 对自定义 IPC 同样生效。
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "get_quotes",
            "get_flash",
            "get_news",
            "get_calendar",
            "search_flash",
            "search_news",
            "get_news_detail",
            "health",
            "get_config",
            "save_config",
            "test_mcp",
            "restart_mcp",
            "open_external",
            "quit_app",
            "open_settings",
            "reset_window_position",
            "toggle_maximize",
            "set_window_size",
            "set_window_opacity",
            "set_mini_mode",
            "set_theme",
        ]),
    ))
    .expect("application permission manifest must compile");
}
