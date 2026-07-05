use gpui::{App, AppContext, Entity};
use liora_components::{CodeEditor, CodeLanguage, CodeTheme};

const SQL: &str = r#"select component, count(*) as usage_count
from ui_events
where created_at >= now() - interval '30 days'
group by component
order by usage_count desc;
"#;

const SHELL: &str = r#"#!/usr/bin/env bash
set -euo pipefail
cargo test -p liora-components code_editor
"#;

const TOML: &str = r#"[editor]
language = "rust"
theme_name = "One Dark"
rows = 18

[options]
line_numbers = true
"#;

pub fn code_editor_language_matrix(cx: &mut App) -> Vec<Entity<CodeEditor>> {
    [
        (CodeLanguage::Sql, SQL),
        (CodeLanguage::Shell, SHELL),
        (CodeLanguage::Toml, TOML),
    ]
    .into_iter()
    .map(|(language, source)| {
        cx.new(move |cx| {
            CodeEditor::new(source, cx)
                .language(language)
                .theme(CodeTheme::OneDark)
                .rows(7)
                .line_numbers(true)
        })
    })
    .collect()
}
