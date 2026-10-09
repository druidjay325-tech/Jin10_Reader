use crate::hotkeys::Registry;
use crate::{
    config::{AppConfig, Shortcuts},
    error::{AppError, AppResult},
    state::AppState,
};
use tauri::{Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub fn parse_shortcuts(config: &Shortcuts) -> AppResult<[Shortcut; 2]> {
    let toggle: Shortcut = config
        .toggle_window
        .parse()
        .map_err(|_| AppError::invalid("显示窗口快捷键格式无效，例如 Ctrl+Alt+J"))?;
    let refresh: Shortcut = config
        .refresh
        .parse()
        .map_err(|_| AppError::invalid("刷新快捷键格式无效，例如 Ctrl+Alt+R"))?;
    if toggle.id() == refresh.id() {
        return Err(AppError::invalid("显示窗口与刷新不能使用同一快捷键"));
    }
    Ok([toggle, refresh])
}

struct NativeRegistry<'a>(&'a tauri::AppHandle);
impl crate::hotkeys::Registry for NativeRegistry<'_> {
    fn register(&self, key: Shortcut) -> AppResult<()> {
        self.0.global_shortcut().register(key).map_err(|error| {
            let detail = error.to_string();
            let occupied = detail.to_lowercase().contains("already") || detail.contains("1409");
            AppError::new(
                if occupied {
                    "shortcut_conflict"
                } else {
                    "shortcut_registration"
                },
                &if occupied {
                    "该组合已被其他应用占用".into()
                } else {
                    format!("系统注册失败：{detail}")
                },
                false,
            )
        })
    }
    fn unregister(&self, key: Shortcut) -> AppResult<()> {
        self.0
            .global_shortcut()
            .unregister(key)
            .map_err(|_| AppError::new("shortcut_registration", "无法移除旧组合", false))
    }
    fn contains(&self, key: Shortcut) -> bool {
        self.0.global_shortcut().is_registered(key)
    }
}

fn reconcile(
    app: &tauri::AppHandle,
    previous: Option<&Shortcuts>,
    config: &Shortcuts,
) -> AppResult<()> {
    let state = app.state::<AppState>();
    let before = state
        .shortcuts
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone();
    let registry = NativeRegistry(app);
    match crate::hotkeys::apply(&registry, &before, previous, config) {
        Ok(result) => {
            *state.shortcuts.lock().unwrap_or_else(|e| e.into_inner()) = result.bindings;
            *state
                .desktop_warning
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = if result.warnings.is_empty() {
                None
            } else {
                Some(result.warnings.join("；"))
            };
            Ok(())
        }
        Err(error) => {
            let mut actual = before;
            if let Ok(keys) = parse_shortcuts(config) {
                for (index, role) in ["toggle", "refresh"].into_iter().enumerate() {
                    actual
                        .entry(keys[index].id())
                        .or_insert((keys[index], role));
                }
            }
            actual.retain(|_, (key, _)| registry.contains(*key));
            *state.shortcuts.lock().unwrap_or_else(|e| e.into_inner()) = actual;
            Err(error)
        }
    }
}

pub fn register_shortcuts(app: &tauri::AppHandle, config: &Shortcuts) -> AppResult<()> {
    reconcile(app, None, config)
}

pub fn initialize_preferences(app: &tauri::AppHandle, config: &AppConfig) -> AppResult<()> {
    register_shortcuts(app, &config.shortcuts)?;
    if let Err(error) = set_autostart(app, config.auto_start) {
        let state = app.state::<AppState>();
        let mut warning = state
            .desktop_warning
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        *warning = Some(match warning.take() {
            Some(existing) => format!("{existing}；{}", error.message),
            None => error.message.clone(),
        });
    }
    Ok(())
}

pub fn rollback_result(error: AppError, restored: bool) -> AppResult<()> {
    if restored {
        Err(error)
    } else {
        Err(AppError::new(
            "partial_rollback",
            "系统设置修改失败，部分回滚未成功；配置未保存，请检查快捷键与启动项",
            false,
        ))
    }
}

pub fn handle_shortcut(
    app: &tauri::AppHandle,
    shortcut: &Shortcut,
    event: tauri_plugin_global_shortcut::ShortcutEvent,
) {
    if event.state() != ShortcutState::Pressed {
        return;
    }
    let action = app
        .state::<AppState>()
        .shortcuts
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&shortcut.id())
        .map(|(_, action)| *action);
    match action {
        Some("toggle") => crate::window::toggle_main(app),
        Some("refresh") => {
            let _ = app.emit_to("main", "refresh-all", ());
        }
        _ => {}
    }
}

pub fn set_autostart(app: &tauri::AppHandle, enabled: bool) -> AppResult<()> {
    let manager = app.autolaunch();
    let previous = manager
        .is_enabled()
        .map_err(|_| AppError::new("autostart", "无法读取系统启动项状态", false))?;
    if previous != enabled {
        let result = if enabled {
            manager.enable()
        } else {
            manager.disable()
        };
        if result.is_err() {
            return Err(AppError::new(
                "autostart",
                "系统启动项修改失败，配置未保存",
                false,
            ));
        }
    }
    if manager.is_enabled().ok() != Some(enabled) {
        return Err(AppError::new(
            "autostart",
            "系统启动项状态与期望不一致，配置未保存",
            false,
        ));
    }
    Ok(())
}

pub struct DesktopSnapshot {
    bindings: crate::hotkeys::Bindings,
    warning: Option<String>,
    auto_start: Option<bool>,
    appearance: Option<(String, f64, f64)>,
}
pub fn snapshot(
    app: &tauri::AppHandle,
    old: &AppConfig,
    next: &AppConfig,
) -> AppResult<DesktopSnapshot> {
    let state = app.state::<AppState>();
    Ok(DesktopSnapshot {
        bindings: state
            .shortcuts
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone(),
        warning: state
            .desktop_warning
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone(),
        auto_start: if old.auto_start != next.auto_start {
            Some(
                app.autolaunch()
                    .is_enabled()
                    .map_err(|_| AppError::new("autostart", "无法读取原系统启动项状态", false))?,
            )
        } else {
            None
        },
        appearance: (old.opacity != next.opacity
            || old.theme != next.theme
            || old.glass_luminosity != next.glass_luminosity)
            .then_some((old.theme.clone(), old.opacity, old.glass_luminosity)),
    })
}
pub fn restore_snapshot(app: &tauri::AppHandle, before: DesktopSnapshot) -> AppResult<()> {
    let state = app.state::<AppState>();
    let current = state
        .shortcuts
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone();
    let (actual, mut restored) =
        crate::hotkeys::restore(&NativeRegistry(app), &before.bindings, &current);
    *state.shortcuts.lock().unwrap_or_else(|e| e.into_inner()) = actual;
    if let Some(enabled) = before.auto_start {
        restored &= set_autostart(app, enabled).is_ok();
    }
    if let Some((theme, opacity, luminosity)) = before.appearance {
        restored &= crate::window::apply_appearance(app, &theme, opacity, luminosity).is_ok();
    }
    *state
        .desktop_warning
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = if restored {
        before.warning
    } else {
        Some("部分系统设置未能恢复，请检查快捷键和启动项".into())
    };
    if restored {
        Ok(())
    } else {
        Err(AppError::new(
            "partial_rollback",
            "部分系统设置未能恢复",
            false,
        ))
    }
}
pub fn apply_preferences(
    app: &tauri::AppHandle,
    old: &AppConfig,
    next: &AppConfig,
) -> AppResult<()> {
    if old.shortcuts != next.shortcuts {
        reconcile(app, Some(&old.shortcuts), &next.shortcuts)?;
    }
    if old.auto_start != next.auto_start {
        set_autostart(app, next.auto_start)?;
    }
    if old.opacity != next.opacity
        || old.theme != next.theme
        || old.glass_luminosity != next.glass_luminosity
    {
        crate::window::apply_appearance(app, &next.theme, next.opacity, next.glass_luminosity)?;
    }
    Ok(())
}
