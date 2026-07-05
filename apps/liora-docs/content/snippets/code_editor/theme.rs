use gpui::{App, AppContext, Entity, rgb};
use liora_components::{
    CodeDiagnostic, CodeEditor, CodeEditorHighlightTheme, CodeEditorInlineDiagnostics,
    CodeEditorOptions, CodeEditorWhitespaceMode, CodeLanguage, CodeTheme,
};

const SOURCE: &str = r#"use liora_components::{
    CodeEditorHighlightTheme, CodeEditorOptions, CodeEditorWhitespaceMode,
};

let options = CodeEditorOptions {
    current_line_highlight: true,
    rulers: true,
    whitespace: CodeEditorWhitespaceMode::Boundary,
    ..CodeEditorOptions::default()
};
"#;

pub fn code_editor_theme(cx: &mut App) -> Entity<CodeEditor> {
    cx.new(|cx| {
        CodeEditor::new(SOURCE, cx)
            .language(CodeLanguage::Rust)
            .highlight_theme(
                CodeEditorHighlightTheme::new(CodeTheme::Nord)
                    .surface(rgb(0x0f172a).into())
                    .chrome_surface(rgb(0x111827).into())
                    .gutter_surface(rgb(0x0b1220).into())
                    .border(rgb(0x334155).into())
                    .text(rgb(0xe5e7eb).into())
                    .muted_text(rgb(0x94a3b8).into())
                    .interaction(
                        rgb(0x38bdf8).into(),
                        rgb(0x2563eb).opacity(0.32).into(),
                        rgb(0x1e293b).into(),
                    )
                    .ruler(rgb(0x475569).opacity(0.72).into())
                    .whitespace(rgb(0x64748b).opacity(0.58).into())
                    .diagnostics(
                        rgb(0x22d3ee).into(),
                        rgb(0xfacc15).into(),
                        rgb(0xfb7185).into(),
                    ),
            )
            .options(CodeEditorOptions {
                current_line_highlight: true,
                rulers: true,
                ruler_column: 88,
                whitespace: CodeEditorWhitespaceMode::Boundary,
                inline_diagnostics: CodeEditorInlineDiagnostics::WarningsAndErrors,
                diagnostics_limit: 4,
                completion_limit: 4,
                ..CodeEditorOptions::default()
            })
            .diagnostics([CodeDiagnostic::warning(
                4,
                5,
                "Theme overrides also recolor diagnostics.",
            )])
    })
}
