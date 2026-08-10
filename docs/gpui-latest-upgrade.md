# GPUI latest upgrade notes

This branch upgrades Liora from the previous official Zed GPUI revision
`2c346f60a76fe3f0367ef924927f50a6efdf5718` to official `zed-industries/zed`
`main` revision `492acd6c815cbe8c7366d54e6092341340afa6c7`.

## Upgrade source and boundary

- Source repository: `https://github.com/zed-industries/zed`.
- GPUI crate version remains `0.2.2`; the behavior change comes from the git revision.
- SDK crates remain app-agnostic. Gallery and Docs remain the only canonical dogfooding apps.
- No WebView, Tauri, HTML/CSS/DOM runtime path is introduced.

## Upstream feature disposition

| Upstream GPUI change | Liora disposition |
| --- | --- |
| `Render` / `RenderOnce` unified through `View`; `gpui::Component` replaced by `gpui::ViewElement` | Updated Liora stateless components and docs snippets to use `ViewElement` while keeping public component builder behavior unchanged. |
| `Window::paint_image(bounds, image_bounds, ...)` signature | Updated image/color-picker custom element painters to pass both visible and image bounds. |
| `Animation::repeat_synced()` and phase-locked rotate animations | `liora-components::motion::spin_icon_with_duration` now uses `repeat_synced()`, so `Spinner` inherits app-clock-synced rotation. |
| `accessibility_id(...)` builder | Added `liora_components::Accessible` and top-level `liora::Accessible` wrapper to attach stable platform-visible accessibility identifiers to any Liora/GPUI element. |
| WebGL backend for `gpui_web` | Lockfile includes the upstream backend dependency changes. Liora remains native GPUI; no web runtime is added. |
| `container_query(...)` | Available to applications directly through `gpui`; not wrapped in SDK yet because Liora layout components must not hide container sizing constraints without a dedicated design pass. |
| Improved `div` scroll axis handling | Inherited from upgraded GPUI. No Liora API change required. |
| System notification platform APIs | Available to final applications through GPUI `App`; not wrapped in Liora notification/toast APIs in this branch to avoid conflating in-window notification components with OS notification policy. |
| Window state/attention APIs (`is_resizable`, `is_minimizable`, `request_attention`, `run_embedded`) | Inherited from GPUI and available to app/window shell code; no SDK wrapper needed for current Liora component contracts. |
| Wayland anchored popups, input regions, layer-shell setters, outbound drags | Inherited at platform layer. Liora Popover/Dropdown still retain in-window fallback behavior because upstream anchored popups are not yet cross-platform complete. |

## Validation expected before merge

```bash
python scripts/context_bootstrap.py validate --root .
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test -p liora-components
cargo check -p liora-docs --bin check_snippets
git diff --check -- . ':(exclude).omx'
```
