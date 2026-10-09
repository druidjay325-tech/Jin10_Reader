use jin10_reader::{
    commands::safe_external_url,
    config::{AppConfig, ConfigPatch, ConfigStore, WindowBounds},
    desktop::parse_shortcuts,
    error::AppError,
    mcp,
    state::AppState,
    window::{WorkArea, clamp_bounds},
};
use serde_json::json;
use std::{
    fs,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);
struct Sandbox(PathBuf);
impl Sandbox {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "jin10-reader-offline-{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn config(&self) -> PathBuf {
        self.0.join("settings.json")
    }
}
impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn patch(value: serde_json::Value) -> ConfigPatch {
    serde_json::from_value(value).unwrap()
}

#[test]
fn public_config_never_serializes_a_secret_or_unknown_fields() {
    let mut config = AppConfig::default();
    config.mcp.bearer_token = "synthetic-private-test-value".into();
    config.extra.insert(
        "privateExtension".into(),
        json!("synthetic-private-test-value"),
    );
    let public = serde_json::to_value(config.public(None)).unwrap();
    assert_eq!(public["mcp"]["hasToken"], true);
    assert!(!public.to_string().contains("synthetic-private-test-value"));
    assert!(public["mcp"].get("bearerToken").is_none());
    assert!(public.get("privateExtension").is_none());
}

#[test]
fn blank_token_preserves_and_clear_is_explicit() {
    let mut config = AppConfig::default();
    config.mcp.bearer_token = "synthetic-private-test-value".into();
    config.reminders.push(json!({"id": 1}));
    let next = config
        .patched(patch(json!({"theme":"light","mcp":{"bearerToken":""}})))
        .unwrap();
    assert_eq!(next.mcp.bearer_token, config.mcp.bearer_token);
    assert_eq!(next.reminders, config.reminders);
    let next = next
        .patched(patch(json!({"mcp":{"clearToken":true}})))
        .unwrap();
    assert!(next.mcp.bearer_token.is_empty());
    assert!(
        config
            .patched(patch(
                json!({"mcp":{"serverUrl":"https://different.example/mcp"}})
            ))
            .is_err()
    );
    assert!(
        config
            .patched(patch(
                json!({"mcp":{"clearToken":true,"bearerToken":"fixture"}})
            ))
            .is_err()
    );
    assert!(serde_json::from_value::<ConfigPatch>(json!({"windowBounds":{}})).is_err());
    assert!(serde_json::from_value::<ConfigPatch>(json!({"reminders":[]})).is_err());
}

#[test]
fn config_updates_preserve_other_fields_and_have_a_previous_backup() {
    let sandbox = Sandbox::new();
    let store = ConfigStore::open(sandbox.config());
    store
        .update(
            |old| {
                let mut next = old.clone();
                next.reminders.push(json!({"id":1}));
                next.extra.insert("futureField".into(), json!(7));
                Ok(next)
            },
            |_, _| Ok(()),
        )
        .unwrap();
    store
        .save_bounds(WindowBounds {
            x: 800,
            y: 200,
            width: 450,
            height: 700,
        })
        .unwrap();
    store
        .update(
            |old| old.patched(patch(json!({"theme":"light"}))),
            |_, _| Ok(()),
        )
        .unwrap();
    let restored = ConfigStore::open(sandbox.config());
    let snapshot = restored.snapshot();
    assert_eq!(snapshot.theme, "light");
    assert_eq!(snapshot.window_bounds.x, 800);
    assert_eq!(snapshot.reminders.len(), 1);
    assert_eq!(snapshot.extra["futureField"], 7);
    let backup: AppConfig =
        serde_json::from_str(&fs::read_to_string(sandbox.0.join("settings.json.bak")).unwrap())
            .unwrap();
    assert_eq!(backup.theme, "dark");
    assert_eq!(backup.window_bounds.x, 800);
}

#[test]
fn concurrent_window_and_settings_updates_do_not_lose_fields() {
    let sandbox = Sandbox::new();
    let store = Arc::new(ConfigStore::open(sandbox.config()));
    let a = store.clone();
    let first = std::thread::spawn(move || {
        for x in 0..20 {
            a.save_bounds(WindowBounds {
                x,
                ..Default::default()
            })
            .unwrap();
        }
    });
    let b = store.clone();
    let second = std::thread::spawn(move || {
        for _ in 0..20 {
            b.update(
                |old| old.patched(patch(json!({"theme":"light","minimizeOnStart":true}))),
                |_, _| Ok(()),
            )
            .unwrap();
        }
    });
    first.join().unwrap();
    second.join().unwrap();
    let config = ConfigStore::open(sandbox.config()).snapshot();
    assert_eq!(config.window_bounds.x, 19);
    assert_eq!(config.theme, "light");
    assert!(config.minimize_on_start);
}

#[test]
fn corrupt_and_future_config_are_preserved_instead_of_silently_overwritten() {
    for content in ["{broken", "{\"schemaVersion\":999}"] {
        let sandbox = Sandbox::new();
        fs::write(sandbox.config(), content).unwrap();
        let store = ConfigStore::open(sandbox.config());
        assert!(store.public().config_warning.is_some());
        assert!(store.public().config_read_only);
        assert!(store.save_bounds(WindowBounds::default()).is_err());
        assert_eq!(fs::read_to_string(sandbox.config()).unwrap(), content);
    }
}

#[test]
fn recovered_backup_can_be_saved_and_partial_shortcuts_preserve_the_other_binding() {
    let sandbox = Sandbox::new();
    let mut config = AppConfig::default();
    config.shortcuts.toggle_window = "Ctrl+Alt+K".into();
    fs::write(
        sandbox.0.join("settings.json.bak"),
        serde_json::to_vec(&config).unwrap(),
    )
    .unwrap();
    let store = ConfigStore::open(sandbox.config());
    assert!(store.public().config_warning.is_some());
    assert!(!store.public().config_read_only);
    store
        .update(
            |old| old.patched(patch(json!({"shortcuts":{"refresh":"Ctrl+Alt+Y"}}))),
            |_, _| Ok(()),
        )
        .unwrap();
    assert!(sandbox.config().exists());
    assert_eq!(store.snapshot().shortcuts.toggle_window, "Ctrl+Alt+K");
    assert_eq!(store.snapshot().shortcuts.refresh, "Ctrl+Alt+Y");
    assert!(store.public().config_warning.is_none());
}

#[test]
fn interrupted_temporary_file_does_not_replace_the_valid_config() {
    let sandbox = Sandbox::new();
    let store = ConfigStore::open(sandbox.config());
    store
        .update(
            |old| old.patched(patch(json!({"theme":"light"}))),
            |_, _| Ok(()),
        )
        .unwrap();
    fs::write(sandbox.0.join("settings.json.tmp"), "{partial").unwrap();
    assert_eq!(
        ConfigStore::open(sandbox.config()).snapshot().theme,
        "light"
    );
}

#[test]
fn missing_primary_and_broken_backup_are_read_only_and_preserved() {
    let sandbox = Sandbox::new();
    let backup = sandbox.0.join("settings.json.bak");
    fs::write(&backup, "{broken-backup").unwrap();
    let store = ConfigStore::open(sandbox.config());
    assert!(store.public().config_read_only);
    assert!(store.public().config_warning.is_some());
    assert!(store.save_bounds(WindowBounds::default()).is_err());
    assert_eq!(fs::read_to_string(&backup).unwrap(), "{broken-backup");
    assert!(!sandbox.config().exists());
}

#[test]
fn missing_primary_and_unreadable_backup_are_not_treated_as_first_start() {
    let sandbox = Sandbox::new();
    let backup = sandbox.0.join("settings.json.bak");
    fs::create_dir(&backup).unwrap();
    let store = ConfigStore::open(sandbox.config());
    assert!(store.public().config_read_only);
    assert!(store.save_bounds(WindowBounds::default()).is_err());
    assert!(backup.is_dir());
    assert!(!sandbox.config().exists());
}

#[test]
fn mini_size_is_clamped_using_actual_pixels_at_higher_dpi() {
    let area = WorkArea {
        x: 1920,
        y: 0,
        width: 1920,
        height: 1040,
        scale: 1.5,
    };
    let position = jin10_reader::window::clamp_physical_position((3300, 900), (750, 390), &area);
    assert_eq!(position, (3090, 650));
    let negative_area = WorkArea { x: -1920, ..area };
    assert_eq!(
        jin10_reader::window::clamp_physical_position((-100, -40), (500, 260), &negative_area),
        (-500, 0)
    );
}

#[test]
fn failed_native_preference_is_not_persisted() {
    let sandbox = Sandbox::new();
    let store = ConfigStore::open(sandbox.config());
    assert!(
        store
            .update(
                |old| old.patched(patch(json!({"autoStart":true}))),
                |_, _| Err(AppError::invalid("测试用系统失败"))
            )
            .is_err()
    );
    assert!(!store.snapshot().auto_start);
    assert!(!sandbox.config().exists());
}

#[test]
fn filesystem_failure_rolls_native_preferences_back() {
    let sandbox = Sandbox::new();
    let blocked = sandbox.0.join("file-instead-of-directory");
    fs::write(&blocked, "test").unwrap();
    let store = ConfigStore::open(blocked.join("settings.json"));
    // 独立构造新的目录后移走它，以模拟加载后介质失效。
    let writable = sandbox.0.join("writable");
    fs::create_dir(&writable).unwrap();
    let store2 = ConfigStore::open(writable.join("settings.json"));
    fs::remove_dir(&writable).unwrap();
    fs::write(&writable, "test").unwrap();
    let effects = std::sync::Mutex::new(Vec::new());
    let result = store2.update(
        |old| old.patched(patch(json!({"theme":"light"}))),
        |_, next| {
            effects.lock().unwrap().push(next.theme.clone());
            Ok(())
        },
    );
    assert!(result.is_err());
    assert_eq!(*effects.lock().unwrap(), ["light", "dark"]);
    assert_eq!(store2.snapshot().theme, "dark");
    assert!(store.public().config_warning.is_some());
}

#[test]
fn unsafe_links_endpoints_and_shortcuts_are_rejected() {
    for url in [
        "javascript:alert(1)",
        "file:///C:/private",
        "data:text/html,test",
        "https://user:secret@example.com",
        "https://example.com\n",
    ] {
        assert!(safe_external_url(url).is_err(), "危险链接未被拒绝");
    }
    assert!(safe_external_url("https://example.com/article?q=test#part").is_ok());
    for url in [
        "http://example.com/mcp",
        "https://example.com/mcp?token=x",
        "https://user@example.com/mcp",
    ] {
        assert!(jin10_reader::config::normalize_endpoint(url).is_err());
    }
    let mut shortcuts = jin10_reader::config::Shortcuts::default();
    shortcuts.refresh = shortcuts.toggle_window.clone();
    assert!(parse_shortcuts(&shortcuts).is_err());
    shortcuts.refresh = "not a shortcut".into();
    assert!(parse_shortcuts(&shortcuts).is_err());
}

#[test]
fn response_envelopes_classify_business_errors_without_retry() {
    assert_eq!(
        mcp::parse_payload(r#"{"status":200,"data":{"list":[1],"next_offset":"cursor-2"}}"#)
            .unwrap()["next_offset"],
        "cursor-2"
    );
    assert_eq!(
        mcp::parse_payload(r#"{"status":429,"message":"quota exceeded"}"#)
            .unwrap_err()
            .code,
        "rate_limited"
    );
    assert_eq!(
        mcp::parse_payload(r#"{"status":401,"message":"invalid token"}"#)
            .unwrap_err()
            .code,
        "auth_failed"
    );
    assert!(!mcp::classify_message("invalid parameter", false).retryable);
    let legacy_quota = mcp::classify_message("-32000 今日已达上限", false);
    assert_eq!(legacy_quota.code, "rate_limited");
    assert!(!legacy_quota.retryable);
    assert_eq!(
        mcp::classify_message("invalid token", false).code,
        "auth_failed"
    );
    assert!(mcp::parse_payload("not JSON").is_err());
    assert!(
        !mcp::classify_message("secret-value authentication request", false)
            .message
            .contains("secret-value")
    );
}

#[test]
fn removed_monitor_is_clamped_and_negative_secondary_monitor_is_preserved() {
    let areas = [
        WorkArea {
            x: 0,
            y: 0,
            width: 1920,
            height: 1040,
            scale: 1.0,
        },
        WorkArea {
            x: -1920,
            y: 0,
            width: 1920,
            height: 1040,
            scale: 1.0,
        },
    ];
    let missing = clamp_bounds(
        &WindowBounds {
            x: 9000,
            y: 9000,
            ..Default::default()
        },
        &areas,
    );
    assert!(missing.x <= 1520 && missing.y <= 440);
    let secondary = WindowBounds {
        x: -1500,
        y: 100,
        ..Default::default()
    };
    assert_eq!(clamp_bounds(&secondary, &areas), secondary);
}

#[tokio::test]
async fn no_token_is_unconfigured_and_health_never_invents_a_success() {
    let sandbox = Sandbox::new();
    let state = AppState::new(Arc::new(ConfigStore::open(sandbox.config())));
    let health = state.health().await;
    assert_eq!(health.state, "unconfigured");
    assert!(health.last_success_at.is_none());
    assert!(matches!(state.client(false).await, Err(error) if error.code == "unconfigured"));
}

#[tokio::test]
async fn stalled_operation_has_a_budget_and_does_not_hold_unrelated_work() {
    let pending = mcp::bounded(
        std::time::Duration::from_millis(30),
        std::future::pending::<Result<(), AppError>>(),
    );
    let unrelated = async {
        tokio::time::sleep(std::time::Duration::from_millis(2)).await;
        7
    };
    let (result, usable) = tokio::join!(pending, unrelated);
    assert_eq!(result.unwrap_err().code, "timeout");
    assert_eq!(usable, 7);
}

#[test]
fn snapping_has_fixed_margins_corners_and_can_be_released() {
    use jin10_reader::window::{WorkArea, snap_position};
    let area = WorkArea {
        x: 0,
        y: 0,
        width: 1920,
        height: 1040,
        scale: 1.0,
    };
    assert_eq!(snap_position((5, 3), (400, 600), &area), (12, 12));
    assert_eq!(snap_position((1518, 436), (400, 600), &area), (1508, 428));
    assert_eq!(snap_position((100, 100), (400, 600), &area), (100, 100));
    assert_eq!(snap_position((12, 12), (400, 600), &area), (12, 12));
    assert_eq!(snap_position((80, 80), (400, 600), &area), (80, 80));
    assert_eq!(snap_position((-10, 100), (400, 600), &area), (12, 100));
    assert_eq!(snap_position((0, 0), (1920, 1040), &area), (0, 0));
}

#[test]
fn snapping_scales_with_dpi_and_negative_monitor_coordinates() {
    use jin10_reader::window::{WorkArea, snap_position};
    let area = WorkArea {
        x: -2560,
        y: -200,
        width: 2560,
        height: 1440,
        scale: 1.5,
    };
    assert_eq!(
        snap_position((-2550, -192), (600, 900), &area),
        (-2542, -182)
    );
    assert_eq!(snap_position((-601, 330), (600, 900), &area), (-618, 322));
    assert_eq!(
        snap_position((-2500, -100), (600, 900), &area),
        (-2500, -100)
    );
}

#[test]
fn glass_theme_round_trips_without_exposing_credentials() {
    use jin10_reader::config::{AppConfig, ConfigPatch};
    for opacity in [0.0, 0.65, 1.0] {
        let config = AppConfig::default()
            .patched(ConfigPatch {
                theme: Some("glass".into()),
                opacity: Some(opacity),
                glass_luminosity: Some(opacity),
                ..Default::default()
            })
            .unwrap();
        let recovered: AppConfig =
            serde_json::from_str(&serde_json::to_string(&config).unwrap()).unwrap();
        assert_eq!(recovered.theme, "glass");
        assert_eq!(recovered.opacity, opacity);
        assert_eq!(recovered.glass_luminosity, opacity);
        assert!(recovered.validate().is_ok());
        let public = serde_json::to_value(recovered.public(None)).unwrap();
        assert!(public["mcp"].get("bearerToken").is_none());
    }
    for opacity in [-0.01, 1.01, f64::NAN] {
        assert!(
            AppConfig::default()
                .patched(ConfigPatch {
                    opacity: Some(opacity),
                    ..Default::default()
                })
                .is_err()
        );
        assert!(
            AppConfig::default()
                .patched(ConfigPatch {
                    glass_luminosity: Some(opacity),
                    ..Default::default()
                })
                .is_err()
        );
    }
}

#[test]
fn older_settings_default_luminosity_to_zero_and_keep_private_fields() {
    use jin10_reader::config::{AppConfig, ConfigPatch};
    let old: AppConfig = serde_json::from_value(
        serde_json::json!({"theme":"glass","opacity":0.2,"collection":{"enabled":false}}),
    )
    .unwrap();
    assert_eq!(old.glass_luminosity, 0.0);
    let changed = old
        .patched(ConfigPatch {
            glass_luminosity: Some(0.25),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(changed.extra["collection"]["enabled"], false);
    assert_eq!(changed.public(None).glass_luminosity, 0.25);
}
