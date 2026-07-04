# Liora Packager

Packaging metadata and artifact helpers for native GPUI applications.

## What it provides

This crate backs Liora's xtask packaging flow and release artifact validation. It models app metadata, cargo-packager configuration, checksums, package manifests, and layout validation.

## Quick example

```rust
use liora_packager::validate_packaging_layout;

let report = validate_packaging_layout(std::env::current_dir().unwrap());
assert!(report.is_ok(), "{report}");
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-packager` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## GPUI dependency note

Liora is developed against the official `zed-industries/zed` GPUI source. When building an application, follow the repository README for the exact GPUI git revision and patching guidance. Do not use unofficial GPUI forks unless the application owner explicitly accepts that risk.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

