#![windows_subsystem = "windows"]

use jin10_reader::{
    commands,
    config::{self, ConfigStore},
    desktop,
    state::AppState,
    tray, window,
};
use std::sync::Arc;
use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let data_dir = config::data_dir().expect("无法确定用户数据目录");
    #[cfg(debug_assertions)]
    let data_dir = std::env::var_os("JIN10_READER_TEST_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or(data_dir);
    let store = Arc::new(ConfigStore::open(data_dir.join("settings.json")));
    let log_dir = data_dir.join("logs");
    let file_writer = tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("jin10-reader")
        .filename_suffix("log")
        .max_log_files(7)
        .build(log_dir);
    // 固定过滤级别，不允许环境变量把请求头、服务响应等敏感调试信息写入日志。
    let registry = tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::new(
        "warn,jin10_reader=info,rmcp=off,reqwest=off,hyper=off",
    ));
    if let Ok(writer) = file_writer {
        let _ = registry
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_writer(writer),
            )
            .try_init();
    } else {
        let _ = registry.with(tracing_subscriber::fmt::layer()).try_init();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window::show(&window);
            }
        }))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(desktop::handle_shortcut)
                .build(),
        )
        .plugin(
            tauri_plugin_autostart::Builder::new()
                .app_name(
                    if cfg!(debug_assertions) && std::env::var_os("JIN10_READER_TEST_DIR").is_some()
                    {
                        "Jin10 Reader QA"
                    } else {
                        "Jin10 Reader"
                    },
                )
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new(store))
        .setup(|app| {
            jin10_reader::acrylic::initialize_ui();
            tray::init(app)?;
            window::apply_window_config(app)?;
            let handle = app.handle().clone();
            // 系统启动项与快捷键通过后台任务应用；失败展示在健康状态，不伪装成已生效。
            tauri::async_runtime::spawn(async move {
                let native = handle.clone();
                let result = tauri::async_runtime::spawn_blocking(move || {
                    let store = native.state::<AppState>().config.clone();
                    store.apply_current(|config| {
                        let result = desktop::initialize_preferences(&native, config);
                        if let Err(error) = &result {
                            *native
                                .state::<AppState>()
                                .desktop_warning
                                .lock()
                                .unwrap_or_else(|e| e.into_inner()) = Some(error.message.clone());
                        }
                        result
                    })
                })
                .await;
                if result.is_err() {
                    *handle
                        .state::<AppState>()
                        .desktop_warning
                        .lock()
                        .unwrap_or_else(|error| error.into_inner()) =
                        Some("系统偏好初始化任务中断，请重启应用后重试".into());
                }
                commands::emit_health(&handle).await;
            });
            // 握手由前端首次可见阅读或用户重连触发，未配置时没有后台请求。
            Ok(())
        })
        .on_window_event(window::handle_window_event)
        .invoke_handler(tauri::generate_handler![
            commands::get_quotes,
            commands::get_flash,
            commands::get_news,
            commands::get_calendar,
            commands::search_flash,
            commands::search_news,
            commands::get_news_detail,
            commands::health,
            commands::get_config,
            commands::save_config,
            commands::set_theme,
            commands::test_mcp,
            commands::restart_mcp,
            commands::open_external,
            commands::quit_app,
            commands::open_settings,
            commands::reset_window_position,
            commands::toggle_maximize,
            commands::set_mini_mode,
            commands::set_window_size,
            commands::set_window_opacity,
        ])
        .build(tauri::generate_context!())
        .expect("Jin10 Reader 启动失败")
        .run(|_, event| {
            if let tauri::RunEvent::Exit = event {
                jin10_reader::acrylic::shutdown();
            }
        });
}
