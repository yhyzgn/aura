use gpui::{App, AppContext, Entity};
use liora_components::{CodeEditor, CodeLanguage, CodeTheme};

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
                .line_height_units(32.0)
        });
}
"#;

pub fn code_editor_line_height(cx: &mut App) -> Entity<CodeEditor> {
    cx.new(|cx| {
        CodeEditor::new(SOURCE, cx)
            .language(CodeLanguage::Rust)
            .theme(CodeTheme::OneDark)
            .rows(8)
            .line_height_units(32.0)
            .current_line_highlight(true)
            .rulers(true)
            .ruler_column(96)
    })
}
