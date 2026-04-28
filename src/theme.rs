use gpui::{Action, App, SharedString};
use gpui_component::{ActiveTheme as _, Theme, ThemeMode, ThemeRegistry};
use serde::{Deserialize, Serialize};
use std::path::Path;

const STATE_FILE: &str = "target/gpui-starter-state.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct State {
    theme: SharedString,
    mode: ThemeMode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: "Default Light".into(),
            mode: ThemeMode::Light,
        }
    }
}

pub fn init(cx: &mut App) {
    let state = load_state();
    apply_state(&state, cx);
    watch_theme_dir(state.clone(), cx);

    cx.observe_global::<Theme>(|cx| {
        persist_state(cx);
    })
    .detach();

    cx.on_action(|switch: &SwitchTheme, cx| {
        let theme_name = switch.0.clone();
        if let Some(theme_config) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme_config);
        }
        cx.refresh_windows();
        persist_state(cx);
    });

    cx.on_action(|switch: &SwitchThemeMode, cx| {
        Theme::change(switch.0, None, cx);
        cx.refresh_windows();
        persist_state(cx);
    });
}

fn watch_theme_dir(state: State, cx: &mut App) {
    let theme_dir = Path::new("./themes");
    if !theme_dir.exists() {
        return;
    }

    if let Err(err) = ThemeRegistry::watch_dir(theme_dir.to_path_buf(), cx, move |cx| {
        apply_state(&state, cx);
        cx.refresh_windows();
    }) {
        eprintln!("failed to watch themes directory: {err}");
    }
}

fn load_state() -> State {
    std::fs::read_to_string(STATE_FILE)
        .ok()
        .and_then(|json| serde_json::from_str::<State>(&json).ok())
        .unwrap_or_default()
}

fn apply_state(state: &State, cx: &mut App) {
    Theme::change(state.mode, None, cx);

    if let Some(theme_config) = ThemeRegistry::global(cx)
        .themes()
        .get(&state.theme)
        .cloned()
    {
        Theme::global_mut(cx).apply_config(&theme_config);
    }
}

fn persist_state(cx: &App) {
    let state = State {
        theme: cx.theme().theme_name().clone(),
        mode: cx.theme().mode,
    };

    let path = Path::new(STATE_FILE);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    if let Ok(json) = serde_json::to_string_pretty(&state) {
        let _ = std::fs::write(path, json);
    }
}

#[derive(Action, Clone, PartialEq)]
#[action(namespace = themes, no_json)]
pub struct SwitchTheme(pub SharedString);

#[derive(Action, Clone, PartialEq)]
#[action(namespace = themes, no_json)]
pub struct SwitchThemeMode(pub ThemeMode);
