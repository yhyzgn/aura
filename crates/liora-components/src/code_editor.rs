//! Code Editor module.
//!
//! This public module implements the Liora native code editor surface for editable snippets and diagnostics. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::{CodeLanguage, CodeTheme, VirtualScrollbar, code_block::highlighted_code_text};
use gpui::{
    App, Bounds, ClipboardItem, Context, Element, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, GlobalElementId, Hsla, InspectorElementId,
    IntoElement, KeyBinding, LayoutId, ListAlignment, ListState, MouseButton, MouseDownEvent,
    MouseMoveEvent, Pixels, Point, Render, SharedString, Style, UTF16Selection, Window, actions,
    div, list, prelude::*, px, relative,
};
use liora_core::{Config, code_font_family, code_font_weight};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::{ops::Range, sync::Arc, time::Duration};

/// Type alias for code editor change callback values used by the code editor API.
pub type CodeEditorChangeCallback = dyn Fn(&str, &mut Context<CodeEditor>) + 'static;
/// Type alias for code diagnostics provider values used by the code editor API.
pub type CodeDiagnosticsProvider = dyn Fn(&str) -> Vec<CodeDiagnostic> + 'static;
/// Type alias for code completion provider values used by the code editor API.
pub type CodeCompletionProvider = dyn Fn(&str) -> Vec<CodeCompletionItem> + 'static;
/// Type alias for hover/help provider values used by the code editor API.
pub type CodeHoverProvider = dyn Fn(&str) -> Option<CodeHover> + 'static;

actions!(
    code_editor,
    [
        #[doc = "Keyboard action that deletes the character before the caret."]
        CodeEditorBackspace,
        #[doc = "Keyboard action that deletes the character after the caret."]
        CodeEditorDelete,
        #[doc = "Keyboard action that moves the caret one character left."]
        CodeEditorLeft,
        #[doc = "Keyboard action that moves the caret one character right."]
        CodeEditorRight,
        #[doc = "Keyboard action that moves the caret one visual row up."]
        CodeEditorUp,
        #[doc = "Keyboard action that moves the caret one visual row down."]
        CodeEditorDown,
        #[doc = "Keyboard action that moves the caret to the current line start."]
        CodeEditorHome,
        #[doc = "Keyboard action that moves the caret to the current line end."]
        CodeEditorEnd,
        #[doc = "Keyboard action that selects the full editor buffer."]
        CodeEditorSelectAll,
        #[doc = "Keyboard action that copies the selected editor text."]
        CodeEditorCopy,
        #[doc = "Keyboard action that pastes clipboard text into the editor."]
        CodeEditorPaste,
        #[doc = "Keyboard action that cuts the selected editor text."]
        CodeEditorCut,
        #[doc = "Keyboard action that inserts a newline into the editor."]
        CodeEditorEnter,
        #[doc = "Keyboard action that extends selection one character left."]
        CodeEditorSelectLeft,
        #[doc = "Keyboard action that extends selection one character right."]
        CodeEditorSelectRight,
        #[doc = "Keyboard action that extends selection one visual row up."]
        CodeEditorSelectUp,
        #[doc = "Keyboard action that extends selection one visual row down."]
        CodeEditorSelectDown,
        #[doc = "Keyboard action that extends selection to the current line start."]
        CodeEditorSelectHome,
        #[doc = "Keyboard action that extends selection to the current line end."]
        CodeEditorSelectEnd,
        #[doc = "Keyboard action that indents the selected code editor lines."]
        CodeEditorIndent,
        #[doc = "Keyboard action that outdents the selected code editor lines."]
        CodeEditorOutdent,
        #[doc = "Keyboard action that restores the previous edit transaction."]
        CodeEditorUndo,
        #[doc = "Keyboard action that reapplies the next edit transaction."]
        CodeEditorRedo
    ]
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Options that control code diagnostic severity behavior.
pub enum CodeDiagnosticSeverity {
    /// Uses informational semantic color tokens.
    Info,
    /// Uses warning semantic color tokens.
    Warning,
    /// Reports a error failure.
    Error,
}

impl CodeDiagnosticSeverity {
    fn label(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    fn color(self, theme: &liora_theme::Theme) -> Hsla {
        match self {
            Self::Info => theme.info.base,
            Self::Warning => theme.warning.base,
            Self::Error => theme.danger.base,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Fluent native GPUI component for rendering Liora code diagnostic.
pub struct CodeDiagnostic {
    /// One-based source line for diagnostics.
    pub line: usize,
    /// One-based source column for diagnostics.
    pub column: usize,
    /// Diagnostic severity used to choose color and icon treatment.
    pub severity: CodeDiagnosticSeverity,
    /// User-facing message associated with this item.
    pub message: SharedString,
}

impl CodeDiagnostic {
    /// Creates `CodeDiagnostic` with default theme-driven styling and no optional callbacks attached.
    pub fn new(
        line: usize,
        column: usize,
        severity: CodeDiagnosticSeverity,
        message: impl Into<SharedString>,
    ) -> Self {
        Self {
            line: line.max(1),
            column: column.max(1),
            severity,
            message: message.into(),
        }
    }

    /// Applies the informational semantic visual variant.
    pub fn info(line: usize, column: usize, message: impl Into<SharedString>) -> Self {
        Self::new(line, column, CodeDiagnosticSeverity::Info, message)
    }

    /// Applies the warning semantic visual variant.
    pub fn warning(line: usize, column: usize, message: impl Into<SharedString>) -> Self {
        Self::new(line, column, CodeDiagnosticSeverity::Warning, message)
    }

    /// Sets the error value used by the component.
    pub fn error(line: usize, column: usize, message: impl Into<SharedString>) -> Self {
        Self::new(line, column, CodeDiagnosticSeverity::Error, message)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Completion candidate rendered by CodeEditor suggestion panels.
pub struct CodeCompletionItem {
    /// Text inserted or referenced by the host application.
    pub label: SharedString,
    /// Optional category such as keyword, function, snippet, or variable.
    pub kind: Option<SharedString>,
    /// Optional explanatory detail shown after the label.
    pub detail: Option<SharedString>,
}

impl CodeCompletionItem {
    /// Creates a completion candidate with a required label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            kind: None,
            detail: None,
        }
    }

    /// Adds a short kind label used for grouping or visual explanation.
    pub fn kind(mut self, kind: impl Into<SharedString>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    /// Adds human-readable detail text.
    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Hover/help content rendered below the editor without binding to a concrete LSP backend.
pub struct CodeHover {
    /// Title for the hover/help panel.
    pub title: SharedString,
    /// Body text shown under the title.
    pub description: SharedString,
}

impl CodeHover {
    /// Creates hover/help content from title and description text.
    pub fn new(title: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Zero-based point inside a [`CodeBuffer`].
pub struct CodePoint {
    /// Zero-based line index.
    pub row: usize,
    /// UTF-8 byte column inside the line.
    pub column: usize,
}

impl CodePoint {
    /// Creates a new point and lets the buffer clamp it when used.
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CodeLine {
    text: String,
    start_offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CodeBuffer {
    text: String,
    lines: Vec<CodeLine>,
}

impl CodeBuffer {
    fn new(value: impl Into<String>) -> Self {
        let mut this = Self {
            text: value.into(),
            lines: Vec::new(),
        };
        this.rebuild_lines();
        this
    }

    fn as_str(&self) -> &str {
        &self.text
    }

    fn len(&self) -> usize {
        self.text.len()
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn line_count(&self) -> usize {
        self.lines.len().max(1)
    }

    fn line(&self, row: usize) -> &str {
        self.lines
            .get(row)
            .map(|line| line.text.as_str())
            .unwrap_or("")
    }

    fn line_start(&self, row: usize) -> usize {
        self.lines
            .get(row)
            .map(|line| line.start_offset)
            .unwrap_or(self.text.len())
    }

    fn line_end(&self, row: usize) -> usize {
        let Some(line) = self.lines.get(row) else {
            return self.text.len();
        };
        line.start_offset + line.text.len()
    }

    fn set_text(&mut self, value: impl Into<String>) {
        self.text = value.into();
        self.rebuild_lines();
    }

    fn replace_range(&mut self, range: Range<usize>, replacement: &str) -> usize {
        let range = normalize_replace_range(&self.text, range);
        self.text.replace_range(range.clone(), replacement);
        let cursor = range.start + replacement.len();
        self.rebuild_lines();
        self.clamp_offset(cursor)
    }

    fn point_to_offset(&self, point: CodePoint) -> usize {
        let row = point.row.min(self.line_count().saturating_sub(1));
        let line = self.line(row);
        self.line_start(row) + clamp_to_char_boundary(line, point.column)
    }

    fn offset_to_point(&self, offset: usize) -> CodePoint {
        let offset = self.clamp_offset(offset);
        let row = match self
            .lines
            .binary_search_by(|line| line.start_offset.cmp(&offset))
        {
            Ok(row) => row,
            Err(row) => row.saturating_sub(1),
        };
        CodePoint::new(row, offset.saturating_sub(self.line_start(row)))
    }

    fn clamp_offset(&self, offset: usize) -> usize {
        clamp_to_char_boundary(&self.text, offset)
    }

    fn prev_char(&self, offset: usize) -> usize {
        let offset = self.clamp_offset(offset);
        if offset == 0 {
            return 0;
        }
        let mut previous = offset - 1;
        while previous > 0 && !self.text.is_char_boundary(previous) {
            previous -= 1;
        }
        previous
    }

    fn next_char(&self, offset: usize) -> usize {
        let offset = self.clamp_offset(offset);
        if offset >= self.text.len() {
            return self.text.len();
        }
        let mut next = offset + 1;
        while next < self.text.len() && !self.text.is_char_boundary(next) {
            next += 1;
        }
        next
    }

    fn line_start_at_offset(&self, offset: usize) -> usize {
        let point = self.offset_to_point(offset);
        self.line_start(point.row)
    }

    fn line_end_at_offset(&self, offset: usize) -> usize {
        let point = self.offset_to_point(offset);
        self.line_end(point.row)
    }

    fn selected_line_bounds(&self, selection: Range<usize>) -> Range<usize> {
        if self.is_empty() {
            return 0..0;
        }
        let start = self.line_start_at_offset(selection.start);
        let mut end_offset = selection.end.min(self.len());
        if end_offset > selection.start
            && end_offset > 0
            && self.text.as_bytes().get(end_offset - 1) == Some(&b'\n')
        {
            end_offset -= 1;
        }
        let end_point = self.offset_to_point(end_offset);
        start..self.line_end(end_point.row)
    }

    fn rebuild_lines(&mut self) {
        self.lines.clear();
        let mut start = 0;
        for segment in self.text.split_inclusive('\n') {
            let line_text = segment.strip_suffix('\n').unwrap_or(segment).to_string();
            self.lines.push(CodeLine {
                text: line_text,
                start_offset: start,
            });
            start += segment.len();
        }
        if self.text.is_empty() || self.text.ends_with('\n') {
            self.lines.push(CodeLine {
                text: String::new(),
                start_offset: self.text.len(),
            });
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CodeSelection {
    range: Range<usize>,
    reversed: bool,
    preferred_column: Option<usize>,
}

impl CodeSelection {
    fn new(cursor: usize) -> Self {
        Self {
            range: cursor..cursor,
            reversed: false,
            preferred_column: None,
        }
    }

    fn cursor(&self) -> usize {
        if self.reversed {
            self.range.start
        } else {
            self.range.end
        }
    }

    fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    fn set_cursor(&mut self, offset: usize) {
        self.range = offset..offset;
        self.reversed = false;
        self.preferred_column = None;
    }

    fn select_to(&mut self, offset: usize) {
        if self.reversed {
            self.range.start = offset;
        } else {
            self.range.end = offset;
        }
        if self.range.end < self.range.start {
            self.reversed = !self.reversed;
            self.range = self.range.end..self.range.start;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CodeDisplayMap {
    row_height: Pixels,
    gutter_width: Pixels,
    content_left_padding: Pixels,
    average_char_width: Pixels,
    viewport_padding: Pixels,
}

impl CodeDisplayMap {
    fn default_for(line_numbers: bool) -> Self {
        Self {
            row_height: px(24.0),
            gutter_width: if line_numbers { px(64.0) } else { px(0.0) },
            content_left_padding: px(14.0),
            average_char_width: px(8.0),
            viewport_padding: px(24.0),
        }
    }

    fn viewport_height(self, rows: usize) -> Pixels {
        px(self.row_height.as_f32() * rows.max(1) as f32 + self.viewport_padding.as_f32())
    }

    fn row_height(self) -> Pixels {
        self.row_height
    }

    fn gutter_width(self, line_numbers: bool) -> Pixels {
        if line_numbers {
            self.gutter_width
        } else {
            px(0.0)
        }
    }

    fn row_for_y(
        self,
        y: Pixels,
        scroll_row: usize,
        scroll_offset_in_row: Pixels,
        line_count: usize,
    ) -> usize {
        let max_row = line_count.saturating_sub(1);
        let row_delta = ((y + scroll_offset_in_row).as_f32() / self.row_height.as_f32())
            .floor()
            .max(0.0) as usize;
        scroll_row.saturating_add(row_delta).min(max_row)
    }

    fn column_for_x(self, x: Pixels, line_numbers: bool) -> usize {
        let content_x = (x.as_f32()
            - self.gutter_width(line_numbers).as_f32()
            - self.content_left_padding.as_f32())
        .max(0.0);
        (content_x / self.average_char_width.as_f32())
            .round()
            .max(0.0) as usize
    }

    fn offset_for_position(
        self,
        buffer: &CodeBuffer,
        position: Point<Pixels>,
        scroll_row: usize,
        scroll_offset_in_row: Pixels,
        line_numbers: bool,
    ) -> usize {
        let row = self.row_for_y(
            position.y,
            scroll_row,
            scroll_offset_in_row,
            buffer.line_count(),
        );
        let column = self.column_for_x(position.x, line_numbers);
        buffer.point_to_offset(CodePoint::new(row, column))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CodeTransaction {
    before_text: String,
    after_text: String,
    before_selection: CodeSelection,
    after_selection: CodeSelection,
}

impl CodeTransaction {
    fn new(before_text: String, before_selection: CodeSelection) -> Self {
        Self {
            before_text,
            after_text: String::new(),
            before_selection,
            after_selection: CodeSelection::new(0),
        }
    }

    fn finish(mut self, after_text: String, after_selection: CodeSelection) -> Self {
        self.after_text = after_text;
        self.after_selection = after_selection;
        self
    }

    fn changed(&self) -> bool {
        self.before_text != self.after_text || self.before_selection != self.after_selection
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CodeViewport {
    rows: usize,
}

impl CodeViewport {
    fn new(rows: usize) -> Self {
        Self { rows: rows.max(1) }
    }

    fn height(self, display: CodeDisplayMap) -> Pixels {
        display.viewport_height(self.rows)
    }
}

/// Native code editing surface with virtualized rows, line numbers, indentation metadata,
/// live syntax highlighting and pluggable diagnostics.
///
/// The v2 foundation follows Zed's public architecture lessons without copying or
/// depending on GPL editor crates: buffer state, selection state, viewport state,
/// and rendered rows are separated so the component can grow toward tree-sitter,
/// LSP, folding, and diff features while remaining pure Rust + native GPUI.
pub struct CodeEditor {
    buffer: CodeBuffer,
    selection: CodeSelection,
    focus_handle: FocusHandle,
    list_state: ListState,
    language: CodeLanguage,
    theme: CodeTheme,
    line_numbers: bool,
    tab_size: usize,
    soft_tabs: bool,
    viewport: CodeViewport,
    height: Option<Pixels>,
    editor_bounds: Option<Bounds<Pixels>>,
    cursor_visible: bool,
    blink_task: Option<gpui::Task<()>>,
    drag_selecting: bool,
    undo_stack: Vec<CodeTransaction>,
    redo_stack: Vec<CodeTransaction>,
    diagnostics: Vec<CodeDiagnostic>,
    diagnostics_provider: Option<Arc<CodeDiagnosticsProvider>>,
    completion_items: Vec<CodeCompletionItem>,
    completion_provider: Option<Arc<CodeCompletionProvider>>,
    hover: Option<CodeHover>,
    hover_provider: Option<Arc<CodeHoverProvider>>,
    search_query: Option<SharedString>,
    on_change: Option<Arc<CodeEditorChangeCallback>>,
}

impl CodeEditor {
    /// Creates `CodeEditor` initialized from the supplied value.
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let value = value.into();
        let buffer = CodeBuffer::new(value.to_string());
        let row_count = buffer.line_count();
        Self {
            selection: CodeSelection::new(buffer.len()),
            buffer,
            focus_handle: cx.focus_handle(),
            list_state: ListState::new(row_count, ListAlignment::Top, px(160.0)),
            language: CodeLanguage::PlainText,
            theme: CodeTheme::Auto,
            line_numbers: true,
            tab_size: 4,
            soft_tabs: true,
            viewport: CodeViewport::new(row_count.max(8).min(24)),
            height: None,
            editor_bounds: None,
            cursor_visible: true,
            blink_task: None,
            drag_selecting: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            diagnostics: Vec::new(),
            diagnostics_provider: None,
            completion_items: Vec::new(),
            completion_provider: None,
            hover: None,
            hover_provider: None,
            search_query: None,
            on_change: None,
        }
    }

    /// Creates a GPUI entity that owns this component state across render passes.
    pub fn entity(value: impl Into<SharedString>, cx: &mut App) -> Entity<Self> {
        let value = value.into();
        cx.new(|cx| Self::new(value, cx))
    }

    /// Returns the serialized value used by forms, configuration, or persistence.
    pub fn value(&self, _cx: &App) -> SharedString {
        SharedString::from(self.buffer.as_str().to_string())
    }

    /// Updates the stored value value and keeps the existing component identity.
    pub fn set_value(&mut self, value: impl Into<SharedString>, cx: &mut Context<Self>) {
        let value = value.into();
        if self.buffer.as_str() == value.as_ref() {
            return;
        }
        self.buffer.set_text(value.to_string());
        self.selection.set_cursor(self.buffer.len());
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.sync_list_state();
        self.handle_buffer_change(cx);
    }

    /// Returns the current zero-based caret point inside the editor buffer.
    pub fn cursor_point(&self) -> CodePoint {
        self.buffer.offset_to_point(self.selection.cursor())
    }

    /// Returns the current selected byte range.
    pub fn selected_range(&self) -> Range<usize> {
        self.selection.range.clone()
    }

    /// Updates the selected byte range and clamps it to valid UTF-8 boundaries.
    pub fn set_selection(&mut self, range: Range<usize>, cx: &mut Context<Self>) {
        let start = self.buffer.clamp_offset(range.start);
        let end = self.buffer.clamp_offset(range.end);
        self.selection.range = start.min(end)..start.max(end);
        self.selection.reversed = false;
        self.selection.preferred_column = None;
        self.reveal_cursor();
        self.reset_blink(cx);
    }

    /// Sets the language identifier used for code display.
    pub fn language(mut self, language: impl Into<CodeLanguage>) -> Self {
        self.language = language.into();
        self
    }

    /// Updates the stored language value and keeps the existing component identity.
    pub fn set_language(&mut self, language: impl Into<CodeLanguage>, cx: &mut Context<Self>) {
        let language = language.into();
        if self.language != language {
            self.language = language;
            cx.notify();
        }
    }

    /// Applies an explicit theme or theme mode.
    pub fn theme(mut self, theme: CodeTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the line numbers value used by the component.
    pub fn line_numbers(mut self, enabled: bool) -> Self {
        self.line_numbers = enabled;
        self
    }

    /// Sets the tab size value used by the component.
    pub fn tab_size(mut self, size: usize) -> Self {
        self.tab_size = size.max(1);
        self
    }

    /// Sets the soft tabs value used by the component.
    pub fn soft_tabs(mut self, enabled: bool) -> Self {
        self.soft_tabs = enabled;
        self
    }

    /// Sets the visible row count for editor-like controls.
    pub fn rows(mut self, rows: usize) -> Self {
        self.viewport = CodeViewport::new(rows);
        self
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Preserves source compatibility with earlier CodeEditor versions.
    ///
    /// CodeEditor v2 renders syntax highlighting inside the editable virtual rows,
    /// so there is no separate preview panel to toggle. This builder intentionally
    /// remains a no-op to avoid breaking existing applications while keeping the
    /// editing surface and the live visual result unified.
    pub fn preview(self, _preview: bool) -> Self {
        self
    }

    /// Sets the diagnostics value used by the component.
    pub fn diagnostics(mut self, diagnostics: impl IntoIterator<Item = CodeDiagnostic>) -> Self {
        self.diagnostics = diagnostics.into_iter().collect();
        self
    }

    /// Updates the stored diagnostics value and keeps the existing component identity.
    pub fn set_diagnostics(
        &mut self,
        diagnostics: impl IntoIterator<Item = CodeDiagnostic>,
        cx: &mut Context<Self>,
    ) {
        self.diagnostics = diagnostics.into_iter().collect();
        cx.notify();
    }

    /// Performs the diagnostics provider operation used by this component.
    pub fn diagnostics_provider(
        mut self,
        provider: impl Fn(&str) -> Vec<CodeDiagnostic> + 'static,
    ) -> Self {
        self.diagnostics_provider = Some(Arc::new(provider));
        self
    }

    /// Updates the stored diagnostics provider value and keeps the existing component identity.
    pub fn set_diagnostics_provider(
        &mut self,
        provider: impl Fn(&str) -> Vec<CodeDiagnostic> + 'static,
        cx: &mut Context<Self>,
    ) {
        self.diagnostics_provider = Some(Arc::new(provider));
        self.refresh_providers();
        cx.notify();
    }

    /// Clears the current diagnostics provider state.
    pub fn clear_diagnostics_provider(&mut self, cx: &mut Context<Self>) {
        self.diagnostics_provider = None;
        cx.notify();
    }

    /// Sets static completion candidates rendered by the editor.
    pub fn completions(mut self, items: impl IntoIterator<Item = CodeCompletionItem>) -> Self {
        self.completion_items = items.into_iter().collect();
        self
    }

    /// Installs a provider that derives completion candidates from the current source.
    pub fn completion_provider(
        mut self,
        provider: impl Fn(&str) -> Vec<CodeCompletionItem> + 'static,
    ) -> Self {
        self.completion_provider = Some(Arc::new(provider));
        self
    }

    /// Sets static hover/help content rendered by the editor.
    pub fn hover(mut self, hover: CodeHover) -> Self {
        self.hover = Some(hover);
        self
    }

    /// Installs a provider that derives hover/help content from the current source.
    pub fn hover_provider(
        mut self,
        provider: impl Fn(&str) -> Option<CodeHover> + 'static,
    ) -> Self {
        self.hover_provider = Some(Arc::new(provider));
        self
    }

    /// Sets a plain-text search query and renders match count metadata.
    pub fn search_query(mut self, query: impl Into<SharedString>) -> Self {
        let query = query.into();
        self.search_query = (!query.is_empty()).then_some(query);
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(
        mut self,
        callback: impl Fn(&str, &mut Context<CodeEditor>) + 'static,
    ) -> Self {
        self.on_change = Some(Arc::new(callback));
        self
    }

    /// Updates the stored on change value and keeps the existing component identity.
    pub fn set_on_change(
        &mut self,
        callback: impl Fn(&str, &mut Context<CodeEditor>) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.on_change = Some(Arc::new(callback));
        cx.notify();
    }

    /// Performs the indent unit operation used by this component.
    pub fn indent_unit(&self) -> String {
        if self.soft_tabs {
            " ".repeat(self.tab_size)
        } else {
            "\t".to_string()
        }
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("backspace", CodeEditorBackspace, None),
            KeyBinding::new("delete", CodeEditorDelete, None),
            KeyBinding::new("left", CodeEditorLeft, None),
            KeyBinding::new("shift-left", CodeEditorSelectLeft, None),
            KeyBinding::new("right", CodeEditorRight, None),
            KeyBinding::new("shift-right", CodeEditorSelectRight, None),
            KeyBinding::new("up", CodeEditorUp, None),
            KeyBinding::new("shift-up", CodeEditorSelectUp, None),
            KeyBinding::new("down", CodeEditorDown, None),
            KeyBinding::new("shift-down", CodeEditorSelectDown, None),
            KeyBinding::new("home", CodeEditorHome, None),
            KeyBinding::new("shift-home", CodeEditorSelectHome, None),
            KeyBinding::new("end", CodeEditorEnd, None),
            KeyBinding::new("shift-end", CodeEditorSelectEnd, None),
            KeyBinding::new("cmd-a", CodeEditorSelectAll, None),
            KeyBinding::new("ctrl-a", CodeEditorSelectAll, None),
            KeyBinding::new("cmd-c", CodeEditorCopy, None),
            KeyBinding::new("ctrl-c", CodeEditorCopy, None),
            KeyBinding::new("cmd-v", CodeEditorPaste, None),
            KeyBinding::new("ctrl-v", CodeEditorPaste, None),
            KeyBinding::new("cmd-x", CodeEditorCut, None),
            KeyBinding::new("ctrl-x", CodeEditorCut, None),
            KeyBinding::new("enter", CodeEditorEnter, None),
            KeyBinding::new("tab", CodeEditorIndent, None),
            KeyBinding::new("shift-tab", CodeEditorOutdent, None),
            KeyBinding::new("cmd-z", CodeEditorUndo, None),
            KeyBinding::new("ctrl-z", CodeEditorUndo, None),
            KeyBinding::new("cmd-shift-z", CodeEditorRedo, None),
            KeyBinding::new("ctrl-shift-z", CodeEditorRedo, None),
        ]);
    }

    fn display_map(&self) -> CodeDisplayMap {
        CodeDisplayMap::default_for(self.line_numbers)
    }

    fn local_editor_position(&self, position: Point<Pixels>) -> Point<Pixels> {
        if let Some(bounds) = self.editor_bounds {
            gpui::point(
                (position.x - bounds.left()).max(px(0.0)),
                (position.y - bounds.top()).max(px(0.0)),
            )
        } else {
            position
        }
    }

    fn point_for_editor_position(&self, position: Point<Pixels>) -> usize {
        let scroll_top = self.list_state.logical_scroll_top();
        self.display_map().offset_for_position(
            &self.buffer,
            self.local_editor_position(position),
            scroll_top.item_ix,
            scroll_top.offset_in_item,
            self.line_numbers,
        )
    }

    fn start_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        let executor = cx.background_executor().clone();
        self.blink_task = Some(cx.spawn(async move |this, cx| {
            loop {
                executor.timer(Duration::from_millis(500)).await;
                let result = this.update(cx, |this, cx| {
                    this.cursor_visible = !this.cursor_visible;
                    cx.notify();
                });
                if result.is_err() {
                    break;
                }
            }
        }));
    }

    fn reset_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        if self.blink_task.is_none() {
            self.start_blink(cx);
        } else {
            cx.notify();
        }
    }

    fn mouse_down_in_editor(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        let offset = self.point_for_editor_position(event.position);
        if event.modifiers.shift {
            self.selection.select_to(offset);
        } else {
            self.selection.set_cursor(offset);
        }
        self.drag_selecting = true;
        self.reveal_cursor();
        self.reset_blink(cx);
    }

    fn mouse_move_in_editor(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.pressed_button != Some(MouseButton::Left) || !self.drag_selecting {
            return;
        }
        let offset = self.point_for_editor_position(event.position);
        self.selection.select_to(offset);
        self.reveal_cursor();
        self.reset_blink(cx);
    }

    fn sync_list_state(&self) {
        let line_count = self.buffer.line_count();
        let current_count = self.list_state.item_count();
        if line_count > current_count {
            self.list_state
                .splice(current_count..current_count, line_count - current_count);
        } else if line_count < current_count {
            self.list_state.splice(line_count..current_count, 0);
        }
    }

    fn refresh_providers(&mut self) {
        let value = self.buffer.as_str();
        if let Some(provider) = self.diagnostics_provider.clone() {
            self.diagnostics = provider(value);
        }
        if let Some(provider) = self.completion_provider.clone() {
            self.completion_items = provider(value);
        }
        if let Some(provider) = self.hover_provider.clone() {
            self.hover = provider(value);
        }
    }

    fn handle_buffer_change(&mut self, cx: &mut Context<Self>) {
        self.refresh_providers();
        if let Some(callback) = self.on_change.clone() {
            let value = self.buffer.as_str().to_string();
            callback(&value, cx);
        }
        self.reveal_cursor();
        self.reset_blink(cx);
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        self.buffer.as_str()[..offset.min(self.buffer.len())]
            .chars()
            .map(char::len_utf16)
            .sum()
    }

    fn offset_from_utf16(&self, target: usize) -> usize {
        let mut utf8 = 0;
        let mut utf16 = 0;
        for ch in self.buffer.as_str().chars() {
            if utf16 >= target {
                break;
            }
            utf16 += ch.len_utf16();
            utf8 += ch.len_utf8();
        }
        self.buffer.clamp_offset(utf8)
    }

    fn replace_selection(&mut self, replacement: &str, cx: &mut Context<Self>) {
        let transaction =
            CodeTransaction::new(self.buffer.as_str().to_string(), self.selection.clone());
        let range = self.selection.range.clone();
        let cursor = self.buffer.replace_range(range, replacement);
        self.selection.set_cursor(cursor);
        self.commit_transaction(transaction, cx);
    }

    fn commit_transaction(&mut self, transaction: CodeTransaction, cx: &mut Context<Self>) {
        let transaction =
            transaction.finish(self.buffer.as_str().to_string(), self.selection.clone());
        if !transaction.changed() {
            return;
        }
        self.undo_stack.push(transaction);
        self.redo_stack.clear();
        self.sync_list_state();
        self.handle_buffer_change(cx);
    }

    fn restore_transaction_snapshot(
        &mut self,
        text: String,
        selection: CodeSelection,
        cx: &mut Context<Self>,
    ) {
        self.buffer.set_text(text);
        self.selection = selection;
        self.selection.range =
            normalize_replace_range(self.buffer.as_str(), self.selection.range.clone());
        self.selection.preferred_column = None;
        self.sync_list_state();
        self.handle_buffer_change(cx);
    }

    fn move_to(&mut self, offset: usize, select: bool, cx: &mut Context<Self>) {
        let offset = self.buffer.clamp_offset(offset);
        if select {
            self.selection.select_to(offset);
        } else {
            self.selection.set_cursor(offset);
        }
        self.reveal_cursor();
        self.reset_blink(cx);
    }

    fn move_vertical(&mut self, delta: isize, select: bool, cx: &mut Context<Self>) {
        let cursor = self.selection.cursor();
        let point = self.buffer.offset_to_point(cursor);
        let preferred = self.selection.preferred_column.unwrap_or(point.column);
        let max_row = self.buffer.line_count().saturating_sub(1) as isize;
        let target_row = (point.row as isize + delta).clamp(0, max_row) as usize;
        let target = self
            .buffer
            .point_to_offset(CodePoint::new(target_row, preferred));
        self.selection.preferred_column = Some(preferred);
        if select {
            self.selection.select_to(target);
        } else {
            self.selection.set_cursor(target);
            self.selection.preferred_column = Some(preferred);
        }
        self.reveal_cursor();
        self.reset_blink(cx);
    }

    fn reveal_cursor(&self) {
        let point = self.buffer.offset_to_point(self.selection.cursor());
        self.list_state.scroll_to_reveal_item(point.row);
    }

    fn indent(&mut self, _: &CodeEditorIndent, _: &mut Window, cx: &mut Context<Self>) {
        let indent = self.indent_unit();
        if indent.is_empty() {
            return;
        }
        if self.selection.is_empty() {
            self.replace_selection(&indent, cx);
            return;
        }
        self.reindent_selected_lines(&indent, true, cx);
    }

    fn outdent(&mut self, _: &CodeEditorOutdent, _: &mut Window, cx: &mut Context<Self>) {
        let indent = self.indent_unit();
        self.reindent_selected_lines(&indent, false, cx);
    }

    fn reindent_selected_lines(&mut self, indent: &str, indenting: bool, cx: &mut Context<Self>) {
        let selection = self.selection.range.clone();
        let line_bounds = self.buffer.selected_line_bounds(selection.clone());
        let source = self.buffer.as_str().to_string();
        let mut next = String::with_capacity(source.len() + indent.len() * 4);
        next.push_str(&source[..line_bounds.start]);

        let mut selection_start_delta = 0isize;
        let mut selection_end_delta = 0isize;
        let mut cursor = line_bounds.start;
        let mut changed = false;

        for line in source[line_bounds.clone()].split_inclusive('\n') {
            let line_abs_start = cursor;
            let (line_body, line_ending) = line
                .strip_suffix('\n')
                .map_or((line, ""), |body| (body, "\n"));
            if indenting {
                next.push_str(indent);
                next.push_str(line_body);
                next.push_str(line_ending);
                changed = true;
                if line_abs_start <= selection.start {
                    selection_start_delta += indent.len() as isize;
                }
                if line_abs_start < selection.end || selection.is_empty() {
                    selection_end_delta += indent.len() as isize;
                }
            } else if let Some(remove_len) = removable_indent_len(line_body, indent) {
                next.push_str(&line_body[remove_len..]);
                next.push_str(line_ending);
                changed = true;
                if line_abs_start < selection.start {
                    selection_start_delta -= remove_len as isize;
                }
                if line_abs_start < selection.end {
                    selection_end_delta -= remove_len as isize;
                }
            } else {
                next.push_str(line_body);
                next.push_str(line_ending);
            }
            cursor += line.len();
        }

        if !changed {
            return;
        }

        next.push_str(&source[line_bounds.end..]);
        let transaction = CodeTransaction::new(source.clone(), self.selection.clone());
        self.buffer.set_text(next);
        let start = apply_signed_delta(selection.start, selection_start_delta);
        let end = apply_signed_delta(selection.end, selection_end_delta).max(start);
        self.selection.range = self.buffer.clamp_offset(start)..self.buffer.clamp_offset(end);
        self.selection.reversed = false;
        self.selection.preferred_column = None;
        self.commit_transaction(transaction, cx);
    }

    fn backspace(&mut self, _: &CodeEditorBackspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.selection.is_empty() {
            let cursor = self.selection.cursor();
            let previous = self.buffer.prev_char(cursor);
            if previous == cursor {
                return;
            }
            self.selection.range = previous..cursor;
        }
        self.replace_selection("", cx);
    }

    fn delete(&mut self, _: &CodeEditorDelete, _: &mut Window, cx: &mut Context<Self>) {
        if self.selection.is_empty() {
            let cursor = self.selection.cursor();
            let next = self.buffer.next_char(cursor);
            if next == cursor {
                return;
            }
            self.selection.range = cursor..next;
        }
        self.replace_selection("", cx);
    }

    fn left(&mut self, _: &CodeEditorLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.buffer.prev_char(self.selection.cursor()), false, cx);
    }

    fn select_left(&mut self, _: &CodeEditorSelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.buffer.prev_char(self.selection.cursor()), true, cx);
    }

    fn right(&mut self, _: &CodeEditorRight, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.buffer.next_char(self.selection.cursor()), false, cx);
    }

    fn select_right(&mut self, _: &CodeEditorSelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.buffer.next_char(self.selection.cursor()), true, cx);
    }

    fn up(&mut self, _: &CodeEditorUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(-1, false, cx);
    }

    fn select_up(&mut self, _: &CodeEditorSelectUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(-1, true, cx);
    }

    fn down(&mut self, _: &CodeEditorDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(1, false, cx);
    }

    fn select_down(&mut self, _: &CodeEditorSelectDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(1, true, cx);
    }

    fn home(&mut self, _: &CodeEditorHome, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(
            self.buffer.line_start_at_offset(self.selection.cursor()),
            false,
            cx,
        );
    }

    fn select_home(&mut self, _: &CodeEditorSelectHome, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(
            self.buffer.line_start_at_offset(self.selection.cursor()),
            true,
            cx,
        );
    }

    fn end(&mut self, _: &CodeEditorEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(
            self.buffer.line_end_at_offset(self.selection.cursor()),
            false,
            cx,
        );
    }

    fn select_end(&mut self, _: &CodeEditorSelectEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(
            self.buffer.line_end_at_offset(self.selection.cursor()),
            true,
            cx,
        );
    }

    fn select_all(&mut self, _: &CodeEditorSelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selection.range = 0..self.buffer.len();
        self.selection.reversed = false;
        self.selection.preferred_column = None;
        self.reset_blink(cx);
    }

    fn copy(&mut self, _: &CodeEditorCopy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selection.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.buffer.as_str()[self.selection.range.clone()].to_string(),
            ));
        }
    }

    fn paste(&mut self, _: &CodeEditorPaste, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(clipboard) = cx.read_from_clipboard() {
            if let Some(text) = clipboard.text() {
                self.replace_selection(&text, cx);
            }
        }
    }

    fn cut(&mut self, _: &CodeEditorCut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selection.is_empty() {
            self.copy(&CodeEditorCopy, window, cx);
            self.replace_selection("", cx);
        }
    }

    fn enter(&mut self, _: &CodeEditorEnter, _: &mut Window, cx: &mut Context<Self>) {
        self.replace_selection("\n", cx);
    }

    fn undo(&mut self, _: &CodeEditorUndo, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(transaction) = self.undo_stack.pop() {
            self.restore_transaction_snapshot(
                transaction.before_text.clone(),
                transaction.before_selection.clone(),
                cx,
            );
            self.redo_stack.push(transaction);
        }
    }

    fn redo(&mut self, _: &CodeEditorRedo, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(transaction) = self.redo_stack.pop() {
            self.restore_transaction_snapshot(
                transaction.after_text.clone(),
                transaction.after_selection.clone(),
                cx,
            );
            self.undo_stack.push(transaction);
        }
    }
}

impl EntityInputHandler for CodeEditor {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let start = self.offset_from_utf16(range_utf16.start);
        let end = self.offset_from_utf16(range_utf16.end);
        (start <= end && end <= self.buffer.len())
            .then(|| self.buffer.as_str()[start..end].to_string())
    }

    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.offset_to_utf16(self.selection.range.start)
                ..self.offset_to_utf16(self.selection.range.end),
            reversed: self.selection.reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        None
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut Context<Self>) {}

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .map(|range| self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end))
            .unwrap_or_else(|| self.selection.range.clone());
        self.selection.range = normalize_replace_range(self.buffer.as_str(), range);
        self.selection.reversed = false;
        self.replace_selection(new_text, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let replacement_start = range_utf16
            .as_ref()
            .map(|range| self.offset_from_utf16(range.start))
            .unwrap_or_else(|| self.selection.range.start);
        let range = range_utf16
            .map(|range| self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end))
            .unwrap_or_else(|| self.selection.range.clone());
        self.selection.range = normalize_replace_range(self.buffer.as_str(), range);
        self.selection.reversed = false;
        self.replace_selection(new_text, cx);
        if let Some(selected) = new_selected {
            let start = replacement_start + selected.start;
            let end = replacement_start + selected.end;
            self.selection.range = normalize_replace_range(self.buffer.as_str(), start..end);
            self.selection.reversed = false;
            self.reset_blink(cx);
        }
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        None
    }

    fn character_index_for_point(
        &mut self,
        pt: Point<Pixels>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.offset_to_utf16(self.point_for_editor_position(pt)))
    }
}

struct CodeEditorInputLayer {
    editor: Entity<CodeEditor>,
}

impl IntoElement for CodeEditorInputLayer {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for CodeEditorInputLayer {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.position = gpui::Position::Absolute;
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.editor.read(cx).focus_handle.clone();
        self.editor.update(cx, |editor, _| {
            editor.editor_bounds = Some(bounds);
        });
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.editor.clone()),
            cx,
        );
    }
}

impl Focusable for CodeEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CodeEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.refresh_providers();
        let focused = self.focus_handle(cx).is_focused(window);
        if focused && self.blink_task.is_none() {
            self.start_blink(cx);
        } else if !focused && self.blink_task.is_some() {
            self.blink_task = None;
            self.cursor_visible = true;
        }

        let theme = cx.global::<Config>().theme.clone();
        let code_family = code_font_family(cx);
        let code_weight = code_font_weight(cx);
        let line_count = self.buffer.line_count();
        let cursor = self.cursor_point();
        let indent_label = if self.soft_tabs {
            format!("spaces:{}", self.tab_size)
        } else {
            "tabs".to_string()
        };
        let display_map = self.display_map();
        let editor_height = self
            .height
            .unwrap_or_else(|| self.viewport.height(display_map));
        let focus_handle = self.focus_handle(cx);
        let list_state = self.list_state.clone();
        let buffer = self.buffer.clone();
        let selection = self.selection.clone();
        let diagnostics = self.diagnostics.clone();
        let show_line_numbers = self.line_numbers;
        let row_display_map = display_map;
        let row_language = self.language;
        let row_code_theme = self.theme;
        let code_family_for_rows = code_family.clone();
        let theme_for_rows = theme.clone();
        let code_weight_for_rows = code_weight;
        let show_cursor = focused && self.cursor_visible;

        div()
            .flex()
            .flex_col()
            .w_full()
            .rounded(px(theme.radius.lg))
            .border_1()
            .border_color(if focused {
                theme.primary.base
            } else {
                theme.neutral.border
            })
            .bg(theme.neutral.card)
            .overflow_hidden()
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::select_up))
            .on_action(cx.listener(Self::down))
            .on_action(cx.listener(Self::select_down))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::select_home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::select_end))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::enter))
            .on_action(cx.listener(Self::indent))
            .on_action(cx.listener(Self::outdent))
            .on_action(cx.listener(Self::undo))
            .on_action(cx.listener(Self::redo))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.hover.opacity(0.52))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(theme.neutral.text_1)
                            .child(
                                Icon::new(IconName::FileCode)
                                    .size(px(14.0))
                                    .color(theme.primary.base),
                            )
                            .child("CodeEditor"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .text_xs()
                            .text_color(theme.neutral.text_3)
                            .child(self.language.label())
                            .child(indent_label)
                            .child(format!("{} lines", line_count))
                            .child(format!("{}:{}", cursor.row + 1, cursor.column + 1))
                            .when_some(self.search_query.clone(), |s, query| {
                                s.child(format!(
                                    "matches:{}",
                                    search_match_count(self.buffer.as_str(), query.as_ref())
                                ))
                            }),
                    ),
            )
            .child(
                div()
                    .relative()
                    .h(editor_height)
                    .bg(theme.neutral.hover.opacity(0.18))
                    .cursor_text()
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::mouse_down_in_editor))
                    .on_mouse_move(cx.listener(Self::mouse_move_in_editor))
                    .child(
                        list(list_state.clone(), move |row, _window, _cx| {
                            render_editor_row(
                                row,
                                &buffer,
                                &selection,
                                &diagnostics,
                                show_line_numbers,
                                row_language,
                                row_code_theme,
                                &theme_for_rows,
                                code_family_for_rows.clone(),
                                code_weight_for_rows,
                                row_display_map,
                                show_cursor,
                            )
                            .into_any_element()
                        })
                        .size_full()
                        .into_any_element(),
                    )
                    .child(CodeEditorInputLayer {
                        editor: cx.entity(),
                    })
                    .child(VirtualScrollbar::new(list_state)),
            )
            .when(!self.diagnostics.is_empty(), |s| {
                s.child(render_diagnostics(&self.diagnostics, &theme))
            })
            .when(!self.completion_items.is_empty(), |s| {
                s.child(render_completions(&self.completion_items, &theme))
            })
            .when_some(self.hover.clone(), |s, hover| {
                s.child(render_hover(hover, &theme))
            })
    }
}

fn render_editor_row(
    row: usize,
    buffer: &CodeBuffer,
    selection: &CodeSelection,
    diagnostics: &[CodeDiagnostic],
    line_numbers: bool,
    language: CodeLanguage,
    code_theme: CodeTheme,
    theme: &liora_theme::Theme,
    code_family: SharedString,
    code_weight: Option<gpui::FontWeight>,
    display_map: CodeDisplayMap,
    show_cursor: bool,
) -> gpui::Div {
    let line = buffer.line(row);
    let cursor_point = buffer.offset_to_point(selection.cursor());
    let cursor_row = cursor_point.row == row;
    let cursor_column = cursor_row.then_some(cursor_point.column);
    let selection_range = line_selection_range(buffer, selection, row);
    let line_diagnostics = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.line.saturating_sub(1) == row)
        .collect::<Vec<_>>();
    let row_bg = if cursor_row {
        theme.primary.light_9.opacity(0.24)
    } else {
        theme.neutral.card.opacity(0.0)
    };

    div()
        .flex()
        .items_start()
        .min_h(display_map.row_height())
        .bg(row_bg)
        .child(if line_numbers {
            div()
                .flex_none()
                .w(display_map.gutter_width(line_numbers))
                .px_3()
                .py_1()
                .border_r_1()
                .border_color(theme.neutral.border)
                .font_family(code_family.clone())
                .when_some(code_weight, |s, weight| s.font_weight(weight))
                .text_xs()
                .text_color(if cursor_row {
                    theme.primary.base
                } else {
                    theme.neutral.text_3
                })
                .text_right()
                .child(format!("{}", row + 1))
                .into_any_element()
        } else {
            div().into_any_element()
        })
        .child(
            div()
                .flex_1()
                .min_w_0()
                .px_3()
                .py_1()
                .font_family(code_family.clone())
                .when_some(code_weight, |s, weight| s.font_weight(weight))
                .text_sm()
                .text_color(theme.neutral.text_1)
                .child(render_line_text(
                    line,
                    selection_range,
                    cursor_column,
                    show_cursor,
                    language,
                    code_theme,
                    theme,
                    &code_family,
                    code_weight,
                ))
                .when(!line_diagnostics.is_empty(), |s| {
                    let mut diagnostics_row = div().flex().flex_col().gap_1().mt_1();
                    for diagnostic in line_diagnostics {
                        diagnostics_row = diagnostics_row.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .text_xs()
                                .text_color(diagnostic.severity.color(theme))
                                .child(
                                    div()
                                        .size(px(5.0))
                                        .rounded_full()
                                        .bg(diagnostic.severity.color(theme)),
                                )
                                .child(diagnostic.message.clone()),
                        );
                    }
                    s.child(diagnostics_row)
                }),
        )
}

fn line_selection_range(
    buffer: &CodeBuffer,
    selection: &CodeSelection,
    row: usize,
) -> Option<Range<usize>> {
    if selection.is_empty() {
        return None;
    }
    let row_start = buffer.line_start(row);
    let row_end = buffer.line_end(row);
    let start = selection.range.start.max(row_start);
    let end = selection.range.end.min(row_end);
    if start < end {
        Some(start - row_start..end - row_start)
    } else {
        None
    }
}

fn render_line_text(
    line: &str,
    selection_range: Option<Range<usize>>,
    cursor_column: Option<usize>,
    show_cursor: bool,
    language: CodeLanguage,
    code_theme: CodeTheme,
    theme: &liora_theme::Theme,
    code_family: &SharedString,
    code_weight: Option<gpui::FontWeight>,
) -> gpui::Div {
    render_line_segments(
        line,
        selection_range,
        cursor_column,
        show_cursor,
        language,
        code_theme,
        theme,
        code_family,
        code_weight,
    )
}

fn render_line_segments(
    line: &str,
    selection_range: Option<Range<usize>>,
    cursor_column: Option<usize>,
    show_cursor: bool,
    language: CodeLanguage,
    code_theme: CodeTheme,
    theme: &liora_theme::Theme,
    code_family: &SharedString,
    code_weight: Option<gpui::FontWeight>,
) -> gpui::Div {
    let cursor_column = cursor_column.map(|column| clamp_to_char_boundary(line, column));
    let selection_range = selection_range.map(|range| normalize_replace_range(line, range));
    let mut boundaries = vec![0, line.len()];
    if let Some(column) = cursor_column {
        boundaries.push(column);
    }
    if let Some(range) = selection_range.clone() {
        boundaries.push(range.start);
        boundaries.push(range.end);
    }
    boundaries.sort_unstable();
    boundaries.dedup();

    let mut row = div().flex().items_center().min_w_0();
    let mut rendered_cursor = false;
    for window in boundaries.windows(2) {
        let start = window[0];
        let end = window[1];
        if cursor_column == Some(start) && show_cursor && !rendered_cursor {
            row = row.child(cursor_element(theme));
            rendered_cursor = true;
        }
        if start == end {
            continue;
        }
        let segment = &line[start..end];
        let selected = selection_range
            .as_ref()
            .is_some_and(|range| start < range.end && end > range.start);
        row = row.child(render_line_segment(
            segment,
            selected,
            language,
            code_theme,
            theme,
            code_family,
            code_weight,
        ));
    }

    if cursor_column == Some(line.len()) && show_cursor && !rendered_cursor {
        row = row.child(cursor_element(theme));
        rendered_cursor = true;
    }
    if line.is_empty() {
        if show_cursor && !rendered_cursor {
            row = row.child(cursor_element(theme));
        }
        row = row.child(div().child(" "));
    }
    row
}

fn render_line_segment(
    segment: &str,
    selected: bool,
    language: CodeLanguage,
    code_theme: CodeTheme,
    theme: &liora_theme::Theme,
    code_family: &SharedString,
    code_weight: Option<gpui::FontWeight>,
) -> gpui::Div {
    div()
        .rounded(px(2.0))
        .when(selected, |s| s.bg(theme.primary.base.opacity(0.30)))
        .child(highlighted_code_text(
            SharedString::from(segment.to_string()),
            language,
            code_theme,
            theme,
            code_family,
            code_weight,
        ))
}

fn cursor_element(theme: &liora_theme::Theme) -> gpui::Div {
    div()
        .flex_none()
        .w(px(2.0))
        .h(px(17.0))
        .rounded(px(1.0))
        .bg(theme.primary.base)
}

fn search_match_count(value: &str, query: &str) -> usize {
    if query.is_empty() {
        return 0;
    }
    value.matches(query).count()
}

fn clamp_to_char_boundary(text: &str, mut offset: usize) -> usize {
    offset = offset.min(text.len());
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

fn normalize_replace_range(text: &str, range: Range<usize>) -> Range<usize> {
    let start = clamp_to_char_boundary(text, range.start);
    let end = clamp_to_char_boundary(text, range.end);
    start.min(end)..start.max(end)
}

fn removable_indent_len(line: &str, indent: &str) -> Option<usize> {
    if line.starts_with(indent) {
        return Some(indent.len());
    }
    if indent.chars().all(|ch| ch == ' ') {
        let max_spaces = indent.len();
        let spaces = line
            .as_bytes()
            .iter()
            .take_while(|byte| **byte == b' ')
            .take(max_spaces)
            .count();
        if spaces > 0 {
            return Some(spaces);
        }
    }
    if indent == "\t" && line.starts_with('\t') {
        return Some(1);
    }
    None
}

fn apply_signed_delta(value: usize, delta: isize) -> usize {
    if delta.is_negative() {
        value.saturating_sub(delta.unsigned_abs())
    } else {
        value.saturating_add(delta as usize)
    }
}

fn render_diagnostics(diagnostics: &[CodeDiagnostic], theme: &liora_theme::Theme) -> gpui::Div {
    let mut panel = div()
        .flex()
        .flex_col()
        .gap_1()
        .border_t_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.hover.opacity(0.36))
        .px_4()
        .py_3();

    for diagnostic in diagnostics {
        let color = diagnostic.severity.color(theme);
        panel = panel.child(
            div()
                .flex()
                .items_start()
                .gap_2()
                .text_sm()
                .child(div().mt(px(7.0)).size(px(6.0)).rounded_full().bg(color))
                .child(
                    div()
                        .flex_1()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(color)
                                .child(format!(
                                    "{} at {}:{}",
                                    diagnostic.severity.label(),
                                    diagnostic.line,
                                    diagnostic.column
                                )),
                        )
                        .child(
                            div()
                                .text_color(theme.neutral.text_2)
                                .child(diagnostic.message.clone()),
                        ),
                ),
        );
    }

    panel
}

fn render_completions(items: &[CodeCompletionItem], theme: &liora_theme::Theme) -> gpui::Div {
    let mut panel = div()
        .flex()
        .flex_col()
        .gap_1()
        .border_t_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .px_4()
        .py_3()
        .child(
            div()
                .text_xs()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(theme.neutral.text_3)
                .child("Completions"),
        );
    for item in items.iter().take(6) {
        panel = panel.child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .text_sm()
                .child(
                    div()
                        .text_color(theme.primary.base)
                        .child(item.label.clone()),
                )
                .when_some(item.kind.clone(), |s, kind| {
                    s.child(div().text_xs().text_color(theme.neutral.text_3).child(kind))
                })
                .when_some(item.detail.clone(), |s, detail| {
                    s.child(div().text_color(theme.neutral.text_2).child(detail))
                }),
        );
    }
    panel
}

fn render_hover(hover: CodeHover, theme: &liora_theme::Theme) -> gpui::Div {
    div()
        .border_t_1()
        .border_color(theme.neutral.border)
        .bg(theme.info.light_9)
        .px_4()
        .py_3()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(theme.info.base)
                .child(hover.title),
        )
        .child(
            div()
                .text_sm()
                .text_color(theme.neutral.text_2)
                .child(hover.description),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_constructors_clamp_to_one_based_locations() {
        let diagnostic = CodeDiagnostic::warning(0, 0, "missing semicolon");
        assert_eq!(diagnostic.line, 1);
        assert_eq!(diagnostic.column, 1);
        assert_eq!(diagnostic.severity, CodeDiagnosticSeverity::Warning);
    }

    #[test]
    fn code_buffer_maps_offsets_and_points() {
        let buffer = CodeBuffer::new("alpha\nbeta\nγamma");
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.point_to_offset(CodePoint::new(1, 2)), 8);
        assert_eq!(buffer.offset_to_point(8), CodePoint::new(1, 2));
        assert_eq!(buffer.line(2), "γamma");
    }

    #[test]
    fn code_buffer_replaces_ranges_on_char_boundaries() {
        let mut buffer = CodeBuffer::new("aγc");
        let cursor = buffer.replace_range(1..3, "b");
        assert_eq!(buffer.as_str(), "abc");
        assert_eq!(cursor, 2);
    }

    #[test]
    fn code_selection_tracks_directionless_range() {
        let mut selection = CodeSelection::new(5);
        selection.select_to(2);
        assert_eq!(selection.range, 2..5);
        assert!(selection.reversed);
        assert_eq!(selection.cursor(), 2);
    }

    #[test]
    fn code_editor_exposes_v2_foundation_api() {
        let source = include_str!("code_editor.rs");
        assert!(source.contains("struct CodeBuffer"));
        assert!(source.contains("struct CodeSelection"));
        assert!(source.contains("struct CodeViewport"));
        assert!(source.contains("ListState::new"));
        assert!(source.contains("VirtualScrollbar::new"));
        assert!(source.contains("mouse_down_in_editor"));
        assert!(source.contains("point_for_editor_position"));
        assert!(source.contains("render_editor_row"));
        assert!(source.contains("highlighted_code_text"));
        assert!(
            !source
                .lines()
                .any(|line| line.trim_start().starts_with("input:"))
        );
    }

    #[test]
    fn code_editor_advanced_models_track_content() {
        let item = CodeCompletionItem::new("println!")
            .kind("macro")
            .detail("debug output");
        assert_eq!(item.label, SharedString::from("println!"));
        assert_eq!(search_match_count("let value = value + 1", "value"), 2);
        assert_eq!(
            CodeHover::new("fn main", "entry point").title,
            SharedString::from("fn main")
        );
    }

    #[test]
    fn selected_line_bounds_cover_complete_lines() {
        let buffer = CodeBuffer::new("one\ntwo\nthree");
        assert_eq!(buffer.selected_line_bounds(5..6), 4..7);
        assert_eq!(buffer.selected_line_bounds(1..6), 0..7);
    }

    #[test]
    fn code_display_map_maps_pointer_positions_to_buffer_offsets() {
        let buffer = CodeBuffer::new("alpha\nbeta\ncharlie");
        let display = CodeDisplayMap::default_for(true);

        assert_eq!(
            display.offset_for_position(&buffer, gpui::point(px(84.0), px(30.0)), 0, px(0.0), true),
            buffer.point_to_offset(CodePoint::new(1, 1))
        );
        assert_eq!(
            display.offset_for_position(&buffer, gpui::point(px(10.0), px(72.0)), 1, px(0.0), true),
            buffer.point_to_offset(CodePoint::new(2, 0))
        );
    }

    #[test]
    fn code_display_map_accounts_for_list_scroll_offset() {
        let buffer = CodeBuffer::new("alpha\nbeta\ncharlie");
        let display = CodeDisplayMap::default_for(true);

        assert_eq!(
            display.offset_for_position(
                &buffer,
                gpui::point(px(84.0), px(20.0)),
                1,
                px(12.0),
                true
            ),
            buffer.point_to_offset(CodePoint::new(2, 1))
        );
    }

    #[test]
    fn code_editor_preserves_gpui_list_scroll_state_between_renders() {
        let source = include_str!("code_editor.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("code editor source should have a production section");
        assert!(production_source.contains("fn sync_list_state(&self)"));
        assert!(production_source.contains("self.list_state.item_count()"));
        assert!(production_source.contains(".splice(current_count..current_count"));
        assert!(production_source.contains(".splice(line_count..current_count, 0)"));
        assert!(production_source.contains("self.list_state.logical_scroll_top()"));
        assert!(!production_source.contains("fn scroll_wheel_in_editor"));
        assert!(
            !production_source
                .contains(".on_scroll_wheel(cx.listener(Self::scroll_wheel_in_editor))")
        );
    }

    #[test]
    fn code_editor_pointer_hit_testing_uses_editor_bounds() {
        let source = include_str!("code_editor.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("code editor source should have a production section");
        assert!(production_source.contains("editor_bounds: Option<Bounds<Pixels>>"));
        assert!(production_source.contains("fn local_editor_position"));
        assert!(production_source.contains("position.x - bounds.left()"));
        assert!(production_source.contains("position.y - bounds.top()"));
        assert!(production_source.contains("editor.editor_bounds = Some(bounds)"));
        assert!(production_source.contains("self.local_editor_position(position)"));
    }

    #[test]
    fn code_display_map_maps_columns_beyond_line_start() {
        let buffer = CodeBuffer::new("alpha\nbeta");
        let display = CodeDisplayMap::default_for(true);

        assert_eq!(
            display.offset_for_position(
                &buffer,
                gpui::point(px(64.0 + 14.0 + 8.0 * 3.0), px(2.0)),
                0,
                px(0.0),
                true,
            ),
            buffer.point_to_offset(CodePoint::new(0, 3))
        );
    }

    #[test]
    fn code_line_selection_range_is_column_scoped() {
        let buffer = CodeBuffer::new("alpha\nbeta\ngamma");
        let selection = CodeSelection {
            range: 2..8,
            reversed: false,
            preferred_column: None,
        };

        assert_eq!(line_selection_range(&buffer, &selection, 0), Some(2..5));
        assert_eq!(line_selection_range(&buffer, &selection, 1), Some(0..2));
        assert_eq!(line_selection_range(&buffer, &selection, 2), None);
    }

    #[test]
    fn code_editor_renders_column_level_caret_and_selection() {
        let source = include_str!("code_editor.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("code editor source should have a production section");
        assert!(production_source.contains("cursor_visible: bool"));
        assert!(production_source.contains("blink_task: Option<gpui::Task<()>>"));
        assert!(production_source.contains("fn start_blink"));
        assert!(production_source.contains("fn line_selection_range"));
        assert!(production_source.contains("cursor_column"));
        assert!(production_source.contains("render_line_segments"));
        assert!(!production_source.contains(
            "let selected = selection.range.start < row_end && selection.range.end > row_start;"
        ));
        assert!(
            !production_source
                .contains(".when(selected, |s| s.bg(theme.primary.base.opacity(0.22)))")
        );
    }

    #[test]
    fn code_transactions_restore_text_and_selection_snapshots() {
        let before = CodeSelection::new(0);
        let mut after = CodeSelection::new(5);
        after.select_to(9);
        let transaction = CodeTransaction::new("alpha".to_string(), before.clone());
        let transaction = transaction.finish("alpha beta".to_string(), after.clone());

        assert_eq!(transaction.before_text, "alpha");
        assert_eq!(transaction.after_text, "alpha beta");
        assert_eq!(transaction.before_selection, before);
        assert_eq!(transaction.after_selection, after);
    }

    #[test]
    fn code_editor_exposes_undo_redo_actions_and_bindings() {
        let source = include_str!("code_editor.rs");
        assert!(source.contains("CodeEditorUndo"));
        assert!(source.contains("CodeEditorRedo"));
        assert!(source.contains("KeyBinding::new(\"cmd-z\""));
        assert!(source.contains("KeyBinding::new(\"ctrl-z\""));
        assert!(source.contains("undo_stack"));
        assert!(source.contains("redo_stack"));
    }
}
