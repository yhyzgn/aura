# Liora Components

Native GPUI component library for desktop Rust applications.

## What it provides

It provides reusable controls such as Button, Input, Select, Table, Grid, Shell, Sidebar, TitleBar, charts, overlays, menus, text primitives, and virtualized data views. Components are native Rust + GPUI, not WebView/Tauri/HTML runtime widgets.

## Quick example

```rust
use liora_components::{Button, Input, init_liora};

fn setup(cx: &mut gpui::App) {
    init_liora(cx);
    let _button = Button::new("Save").primary();
    let _input = Input::new().placeholder("Search");
}
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-components` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## GPUI dependency note

Liora is developed against the official `zed-industries/zed` GPUI source. When building an application, follow the repository README for the exact GPUI git revision and patching guidance. Do not use unofficial GPUI forks unless the application owner explicitly accepts that risk.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

