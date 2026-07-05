# Liora CodeEditor v2 Design

## Goal

CodeEditor v2 turns the old `Input + CodeBlock preview` composition into a real native editor foundation that can grow into enterprise editing scenarios without adopting a WebView runtime or coupling Liora to Zed's GPL editor crates.

The design is based on the official `zed-industries/zed` source at the GPUI revision used by this repository. Zed's editor is fast because it separates text storage, snapshots, display mapping, visible-row rendering, language services, and UI painting. Liora follows the same architectural direction while keeping its implementation independent and license-safe.

## Non-goals for the first landing

- Do not copy Zed `editor`, `language`, `multi_buffer`, `text`, or `rope` crate code.
- Do not add WebView, Monaco, Tauri runtime, HTML, CSS, or DOM dependencies.
- Do not hard-bind to a specific LSP server.
- Do not attempt to ship all Zed-grade features in one patch.

## Official Zed references inspected

- `crates/editor/src/editor.rs`: editor state, commands, diagnostics, completion, hover, scroll manager.
- `crates/editor/src/display_map.rs`: display coordinate mapping, inlays, folds, soft wrap, blocks.
- `crates/editor/src/element.rs`: visible-row layout and painting.
- `crates/language/src/buffer.rs`: buffer snapshots and language-aware text state.
- `crates/language/src/syntax_map.rs`: incremental Tree-sitter syntax layers.
- `crates/multi_buffer/src/multi_buffer.rs`: excerpts, anchors, diff/review-ready buffer composition.
- `crates/rope/src/rope.rs`: scalable text storage.

## License boundary

The official Zed editor-adjacent crates are GPL-3.0-or-later. Liora currently uses `LicenseRef-Liora`, so CodeEditor v2 must not directly copy or publish those GPL modules inside the SDK unless the project owner explicitly changes the licensing strategy.

## v2 internal layers

### CodeBuffer

Owns source text and line metadata. The first implementation uses a compact `String + line index` model. This is intentionally isolated so a future rope backend can replace it without changing the public `CodeEditor` API.

Responsibilities:

- UTF-8 safe offset clamping.
- offset ↔ point mapping.
- line start/end lookup.
- range replacement.
- selected-line range expansion for indent/outdent.

### CodeSelection

Tracks primary cursor and selected byte range separately from UI rendering.

Responsibilities:

- collapsed cursor range.
- reversed selections for shift-selection.
- preferred column for vertical movement.

Future expansion:

- multi-cursor selections.
- rectangular/column selection.
- selection history.

### CodeViewport

Stores editor viewport sizing decisions. Rendering uses GPUI's official `ListState` and `list` element for visible-row rendering, then overlays Liora's `VirtualScrollbar`.

Responsibilities:

- visible row count.
- default editor height.
- future soft-wrap and row-height policy.

### Provider stores

Diagnostics, completions, and hover data remain provider-driven. This keeps Liora independent from a concrete LSP process and allows enterprise apps to bridge local LSP, remote services, or custom validators.

## Current deliverable

- `CodeEditor` no longer stores an `Entity<Input>` editing core.
- Editing state is held by `CodeBuffer` and `CodeSelection`.
- Rows render through GPUI `list`, so large documents no longer require rendering every source line into a single element tree.
- Existing public builder methods remain available: `language`, `theme`, `line_numbers`, `tab_size`, `soft_tabs`, `rows`, `height`, `preview`, `diagnostics`, `diagnostics_provider`, `completion_provider`, `hover_provider`, `search_query`, `on_change`.
- Existing keybindings are expanded to editor-owned copy/paste/cut, navigation, selection, enter, indent, and outdent actions.

## Next phases

1. Replace `String + line index` with a rope-like backend or an internal piece table.
2. Add an explicit transaction log for undo/redo and grouped edits.
3. Add a display map layer for soft wrap, folds, inlays, code lens, and block widgets.
4. Add optional Tree-sitter feature gates for incremental syntax highlighting.
5. Add LSP bridge traits without hard-coding a server implementation.
6. Add true cursor hit-testing and mouse drag selection on virtualized rows.
7. Add minimap, bracket matching, inline completion, and diff gutters.

## Verification expectations

Every CodeEditor change should run at least:

```bash
cargo test -p liora-components code_editor
cargo check -p liora-gallery
cargo check -p liora-docs
```

For performance-sensitive changes, use gallery/docs examples with large generated buffers and verify scrolling remains smooth.
