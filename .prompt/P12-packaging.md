# P12 — Native Installer Packaging

## Goal

Build a cross-platform installer and package generation pipeline for Aura's pure Rust + GPUI native applications.

## Non-negotiable Constraint

Aura applications must remain pure Rust + GPUI native apps. Do not convert `aura-gallery`, `aura-docs`, or future Aura apps to Tauri. Do not introduce WebView, HTML/CSS/DOM, browser runtime, or frontend build systems as application runtime dependencies.

## Naming Decision

The internal packaging module is named `aura-packager`, not `aura-installer`.

- `aura-packager` is a Rust library for packaging domain logic.
- `xtask` is the command entrypoint: `cargo xtask package ...`.
- `packaging/` stores static platform resources and packager configuration.

## Technical Direction

- `crates/aura-packager`: app metadata, package formats, checksums, output manifests, validation helpers.
- `xtask`: build orchestration, app/format selection, future cargo-packager/RPM backend invocation.
- `packaging/`: Packager config, icons, Linux desktop/metainfo, macOS entitlements, Windows installer resources.
- Primary backend: `cargo-packager` for app/dmg/deb/AppImage/NSIS/MSI/Pacman where practical.
- RPM backend: `cargo-generate-rpm` or `nfpm` as a supplemental path.

## Required Package Formats

- Linux: AppImage, deb, rpm, portable tar.gz.
- macOS: app, dmg.
- Windows: NSIS exe, MSI.

## Current Implementation Baseline

- `docs/packaging-installer-technical-plan.md` is the source technical plan.
- Initial `crates/aura-packager` crate exists.
- Initial `xtask package validate/build/package` command exists.
- `packaging/` contains initial app metadata and platform integration skeletons.

## Next Work

1. Replace placeholder app icons with real PNG/ICNS/ICO assets.
2. Integrate `cargo-packager` backend invocation from `xtask`.
3. Add RPM generation backend.
4. Generate checksums and `package-manifest.json` for produced artifacts.
5. Add Linux AppImage/deb/rpm smoke checks.
6. Add CI matrix for Linux/macOS/Windows packaging.
