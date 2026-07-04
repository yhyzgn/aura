# Liora Tray

Cross-platform system tray facade for Liora applications.

## What it provides

It wraps tray menu state, close-to-tray behavior, tray commands, icon events, and tray control-center integration while keeping application-specific icons owned by the app.

## Quick example

```rust
use liora_tray::TrayConfig;

let _config = TrayConfig::new("my-native-app")
    .tooltip("My Native App");
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-tray` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## GPUI dependency note

Liora is developed against the official `zed-industries/zed` GPUI source. When building an application, follow the repository README for the exact GPUI git revision and patching guidance. Do not use unofficial GPUI forks unless the application owner explicitly accepts that risk.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

