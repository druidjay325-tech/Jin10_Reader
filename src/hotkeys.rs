use crate::{
    config::Shortcuts,
    desktop::parse_shortcuts,
    error::{AppError, AppResult},
};
use std::collections::HashMap;
use tauri_plugin_global_shortcut::{Code, Shortcut};

pub type Bindings = HashMap<u32, (Shortcut, &'static str)>;
pub trait Registry {
    fn register(&self, shortcut: Shortcut) -> AppResult<()>;
    fn unregister(&self, shortcut: Shortcut) -> AppResult<()>;
    fn contains(&self, shortcut: Shortcut) -> bool;
}
pub struct AppliedBindings {
    pub bindings: Bindings,
    pub warnings: Vec<String>,
}

fn with_context(role: &str, shortcut: Shortcut, error: AppError) -> AppError {
    let label = if role == "toggle" {
        "显示/隐藏"
    } else {
        "刷新"
    };
    AppError::new(
        &error.code,
        &format!("{label}快捷键 {shortcut}：{}", error.message),
        false,
    )
}
fn register_checked(registry: &impl Registry, shortcut: Shortcut) -> AppResult<()> {
    if cfg!(windows) && shortcut.key == Code::F12 {
        return Err(AppError::invalid(
            "Windows 保留 F12 用于调试，请选择其他主键",
        ));
    }
    registry.register(shortcut)
}

pub fn restore(
    registry: &impl Registry,
    before: &Bindings,
    attempted: &Bindings,
) -> (Bindings, bool) {
    let mut ok = true;
    for (id, (shortcut, _)) in attempted {
        if !before.contains_key(id) && registry.contains(*shortcut) {
            ok &= registry.unregister(*shortcut).is_ok();
        }
    }
    for (shortcut, _) in before.values() {
        if !registry.contains(*shortcut) {
            ok &= registry.register(*shortcut).is_ok();
        }
    }
    let mut actual = attempted.clone();
    actual.extend(before.clone());
    actual.retain(|_, (shortcut, _)| registry.contains(*shortcut));
    (actual, ok)
}

/// 新改动必须成功；原先不可用且未改动的组合只产生警告，不牵连有效组合。
pub fn apply(
    registry: &impl Registry,
    before: &Bindings,
    previous: Option<&Shortcuts>,
    desired: &Shortcuts,
) -> AppResult<AppliedBindings> {
    let keys = parse_shortcuts(desired)?;
    let required = previous
        .map(|old| {
            [
                old.toggle_window != desired.toggle_window,
                old.refresh != desired.refresh,
            ]
        })
        .unwrap_or([false, false]);
    let mut next = Bindings::new();
    let mut warnings = Vec::new();
    for (index, role) in ["toggle", "refresh"].into_iter().enumerate() {
        let shortcut = keys[index];
        if !registry.contains(shortcut)
            && let Err(error) = register_checked(registry, shortcut)
        {
            let error = with_context(role, shortcut, error);
            if required[index] {
                let (_, restored) = restore(registry, before, &next);
                return Err(if restored {
                    error
                } else {
                    AppError::new(
                        "partial_rollback",
                        "快捷键修改失败，部分回滚未成功，请检查系统状态",
                        false,
                    )
                });
            }
            warnings.push(error.message);
            continue;
        }
        next.insert(shortcut.id(), (shortcut, role));
    }
    for (id, (shortcut, _)) in before {
        if !next.contains_key(id)
            && let Err(error) = registry.unregister(*shortcut)
        {
            let (_, restored) = restore(registry, before, &next);
            return Err(if restored {
                error
            } else {
                AppError::new(
                    "partial_rollback",
                    "无法移除旧快捷键，部分回滚未成功，请重启后重试",
                    false,
                )
            });
        }
    }
    Ok(AppliedBindings {
        bindings: next,
        warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, collections::HashSet};
    struct Fake {
        owned: RefCell<HashSet<u32>>,
        busy: HashSet<u32>,
    }
    impl Registry for Fake {
        fn register(&self, s: Shortcut) -> AppResult<()> {
            if self.busy.contains(&s.id()) {
                Err(AppError::new("shortcut_conflict", "该组合已被占用", false))
            } else {
                self.owned.borrow_mut().insert(s.id());
                Ok(())
            }
        }
        fn unregister(&self, s: Shortcut) -> AppResult<()> {
            self.owned.borrow_mut().remove(&s.id());
            Ok(())
        }
        fn contains(&self, s: Shortcut) -> bool {
            self.owned.borrow().contains(&s.id())
        }
    }
    fn config(toggle: &str, refresh: &str) -> Shortcuts {
        Shortcuts {
            toggle_window: toggle.into(),
            refresh: refresh.into(),
        }
    }
    #[test]
    fn unchanged_conflict_does_not_block_a_valid_change() {
        let old = config("Ctrl+Alt+J", "Ctrl+Alt+R");
        let next = config("Ctrl+Alt+K", "Ctrl+Alt+R");
        let [a, b] = parse_shortcuts(&old).unwrap();
        let registry = Fake {
            owned: RefCell::new([a.id()].into()),
            busy: [b.id()].into(),
        };
        let before = [(a.id(), (a, "toggle"))].into();
        let result = apply(&registry, &before, Some(&old), &next).unwrap();
        assert_eq!(result.bindings.len(), 1);
        assert!(result.warnings[0].contains("刷新快捷键"));
        assert!(!registry.contains(a));
        assert!(registry.contains(parse_shortcuts(&next).unwrap()[0]));
    }
    #[test]
    fn changed_conflict_restores_existing_keys_and_actions() {
        let old = config("Ctrl+Alt+J", "Ctrl+Alt+R");
        let next = config("Ctrl+Alt+R", "Ctrl+Alt+K");
        let [a, b] = parse_shortcuts(&old).unwrap();
        let c = parse_shortcuts(&next).unwrap()[1];
        let registry = Fake {
            owned: RefCell::new([a.id(), b.id()].into()),
            busy: [c.id()].into(),
        };
        let before = [(a.id(), (a, "toggle")), (b.id(), (b, "refresh"))].into();
        let error = apply(&registry, &before, Some(&old), &next).err().unwrap();
        assert!(error.message.contains("刷新快捷键"));
        assert!(registry.contains(a) && registry.contains(b));
        assert_eq!(before[&b.id()].1, "refresh");
    }
    #[test]
    fn startup_keeps_other_available_binding_when_one_conflicts() {
        let desired = config("Ctrl+Alt+J", "Ctrl+Alt+R");
        let [a, b] = parse_shortcuts(&desired).unwrap();
        let registry = Fake {
            owned: RefCell::new(HashSet::new()),
            busy: [a.id()].into(),
        };
        let result = apply(&registry, &Bindings::new(), None, &desired).unwrap();
        assert_eq!(result.bindings.len(), 1);
        assert!(registry.contains(b));
        assert!(result.warnings[0].contains("显示/隐藏"));
    }
}
