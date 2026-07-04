# Liora Updater

Safe GitHub Release updater planning library.

## What it provides

The updater crate helps applications check GitHub releases, select platform-appropriate assets, verify checksums, and produce install plans. It does not silently escalate privileges or force app-specific hardcoded behavior.

## Quick example

```rust
use liora_updater::{AssetSelector, Platform, UpdateRequest};

let platform = Platform::current().unwrap_or(Platform::LinuxX64);
let _request = UpdateRequest::new(
    "liora-gallery",
    "v0.2.3",
    platform,
    std::env::temp_dir().join("liora-update-cache"),
);
let _selector = AssetSelector::for_platform(platform).matching_prefix("liora-gallery");
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-updater` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## GPUI dependency note

Liora is developed against the official `zed-industries/zed` GPUI source. When building an application, follow the repository README for the exact GPUI git revision and patching guidance. Do not use unofficial GPUI forks unless the application owner explicitly accepts that risk.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

