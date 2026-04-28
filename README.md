# GPUI App Template

A small starter for building desktop apps with GPUI and `gpui-component`.

## Run

```bash
cargo run
```

## Reuse

Copy this folder when starting a new GPUI app, then update:

- `Cargo.toml`: package name, version, and dependencies
- `src/main.rs`: `APP_TITLE`
- `src/screens/home.rs`: first screen UI
- `src/app_menus.rs`: app-specific menu actions

## Layout

```txt
src/
├── main.rs              # GPUI application bootstrap
├── app.rs               # Root application view
├── app_menus.rs         # Native and in-window menu setup
├── theme.rs             # Theme mode switching and persistence
├── ui.rs                # Reusable UI module root
├── ui/
│   └── app_title_bar.rs # Client-side title bar
├── screens.rs           # Screen module root
└── screens/
    └── home.rs          # Starter content screen
```
