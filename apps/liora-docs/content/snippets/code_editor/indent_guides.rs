use gpui::{App, AppContext, Entity};
use liora_components::{CodeEditor, CodeEditorWhitespaceMode, CodeLanguage, CodeTheme};

const SOURCE: &str = r#"pub fn build_panel() {
    let root = Shell::new()
        .sidebar(|sidebar| {
            sidebar
                .brand("Liora")
                .item("CodeEditor")
                .item("Diagnostics")
        })
        .content(|cx| {
            CodeEditor::new("fn main() {}", cx)
                .indent_guides(true)
        });
}
"#;

pub fn code_editor_indent_guides(cx: &mut App) -> Entity<CodeEditor> {
    cx.new(|cx| {
        CodeEditor::new(SOURCE, cx)
            .language(CodeLanguage::Rust)
            .theme(CodeTheme::OneDark)
            .rows(8)
            .indent_guides(true)
            .whitespace(CodeEditorWhitespaceMode::Boundary)
            .current_line_highlight(true)
    })
}
