use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*, rgb};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{
    CodeCompletionItem, CodeDiagnostic, CodeEditor, CodeEditorHighlightTheme,
    CodeEditorInlineDiagnostics, CodeEditorOptions, CodeEditorWhitespaceMode, CodeHover,
    CodeLanguage, CodeTheme, Space, Text, toast_info,
};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| CodeEditorDemo::new(cx)).into()
}

struct CodeEditorDemo {
    basic: Entity<CodeEditor>,
    diagnostics: Entity<CodeEditor>,
    advanced: Entity<CodeEditor>,
    configurable: Entity<CodeEditor>,
    themed: Entity<CodeEditor>,
}

impl CodeEditorDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic = cx.new(|cx| {
            CodeEditor::new(RUST_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .tab_size(4)
                .soft_tabs(true)
                .line_numbers(true)
                .theme(CodeTheme::OneDark)
                .on_change(|value, _| {
                    let line_count = value.lines().count().max(1);
                    toast_info!("CodeEditor changed: {} lines", line_count);
                })
        });
        let diagnostics = cx.new(|cx| {
            CodeEditor::new(TS_SAMPLE, cx)
                .language(CodeLanguage::TypeScript)
                .tab_size(2)
                .soft_tabs(true)
                .line_numbers(true)
                .theme(CodeTheme::GitHubDark)
                .diagnostics([
                    CodeDiagnostic::warning(3, 7, "Prefer an explicit return type."),
                    CodeDiagnostic::info(
                        5,
                        3,
                        "Diagnostics are provider-driven and can be replaced by LSP later.",
                    ),
                ])
        });

        let advanced = cx.new(|cx| {
            CodeEditor::new(RUST_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .theme(CodeTheme::OneDark)
                .search_query("Space")
                .completions([
                    CodeCompletionItem::new("Space::new")
                        .kind("struct")
                        .detail("layout container"),
                    CodeCompletionItem::new("Button::new")
                        .kind("function")
                        .detail("action control"),
                    CodeCompletionItem::new("toast_info!")
                        .kind("macro")
                        .detail("show message"),
                ])
                .hover(CodeHover::new(
                    "Space::new",
                    "Creates a flexible native layout container.",
                ))
        });

        let themed = cx.new(|cx| {
            CodeEditor::new(THEME_SAMPLE, cx)
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
                .completions([
                    CodeCompletionItem::new("highlight_theme(...)"),
                    CodeCompletionItem::new("whitespace(CodeEditorWhitespaceMode::Boundary)"),
                    CodeCompletionItem::new("ruler_column(88)"),
                ])
        });

        let configurable = cx.new(|cx| {
            CodeEditor::new(CONFIG_SAMPLE, cx)
                .language(CodeLanguage::Rust)
                .theme(CodeTheme::OneDark)
                .options(CodeEditorOptions {
                    read_only: true,
                    header: false,
                    status_bar: true,
                    line_numbers: false,
                    diagnostics_panel: false,
                    completions_panel: true,
                    hover_panel: false,
                    current_line_highlight: false,
                    completion_limit: 3,
                    ..CodeEditorOptions::default()
                })
                .completions([
                    CodeCompletionItem::new("CodeEditorOptions::default")
                        .kind("config")
                        .detail("start from full chrome"),
                    CodeCompletionItem::new("read_only(true)")
                        .kind("builder")
                        .detail("disable mutation commands"),
                    CodeCompletionItem::new("current_line_highlight(true)")
                        .kind("builder")
                        .detail("highlight the active row"),
                ])
        });

        Self {
            basic,
            diagnostics,
            advanced,
            configurable,
            themed,
        }
    }
}

impl Render for CodeEditorDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "CodeEditor 代码编辑器",
            "原生 GPUI 代码编辑控件 v2 基座，支持独立 buffer/selection/viewport、可见行渲染、行内实时语法高亮和 diagnostics 扩展点。",
            Space::new().vertical().gap_xl().child(section(
                "Editor showcase",
                "代码编辑示例统一使用宽卡片展示，避免编辑器高度和说明文本打散页面节奏。",
                showcase_stack(vec![
                    showcase_card_wide(
                        "Rust 编辑器",
                        "使用独立 CodeBuffer 与 GPUI list 可见行渲染，不再把 Input 当作编辑核心。",
                        self.basic.clone(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "Diagnostics 扩展点",
                        "业务层可以通过 diagnostics(...) 或 set_diagnostics(...) 注入任意诊断结果。",
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.diagnostics.clone())
                            .child(Text::new("v2 基座已完成基础输入、导航、选择、行号、缩进元数据、虚拟行渲染、诊断渲染、display map 与 undo/redo 事务基座；当前已切到 tree-sitter + Zed-style SyntaxTheme 高亮路线；后续继续增强 LSP bridge、软换行和增量解析。")),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "高级扩展点：搜索、补全和 hover",
                        "Provider-ready 的搜索、补全候选和 hover/help 数据模型可接入语言服务。",
                        self.advanced.clone(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "配置矩阵：只读、Chrome 和扩展面板",
                        "通过 CodeEditorOptions 或 builder 控制 header/status、行号、只读、当前行高亮、诊断/补全/hover 面板和补全数量。",
                        self.configurable.clone(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "自定义高亮主题和高级编辑行为",
                        "CodeEditorHighlightTheme 可覆盖编辑器底色、gutter、selection、caret、ruler、whitespace、诊断色，并可装载 Zed-style syntax capture 主题。",
                        self.themed.clone(),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}

const RUST_SAMPLE: &str = r#"use liora_components::{Button, CodeEditor, Space};

pub fn editor_panel() -> impl gpui::IntoElement {
    Space::new()
        .vertical()
        .child(Button::new("Run").primary())
}
"#;

const TS_SAMPLE: &str = r#"type Metric = { label: string; value: number };

export function summarize(items: Metric[]) {
  return items.map((item) => `${item.label}: ${item.value}`);
}
"#;

const CONFIG_SAMPLE: &str = r#"use liora_components::{CodeEditor, CodeEditorOptions};

let options = CodeEditorOptions::default();
// Turn panels on/off per product surface.
// Read-only mode still supports selection, copy, scroll and hover.
"#;

const THEME_SAMPLE: &str = r#"use liora_components::{
    CodeEditorHighlightTheme, CodeEditorOptions, CodeEditorWhitespaceMode,
};

let options = CodeEditorOptions {
    current_line_highlight: true,
    rulers: true,
    whitespace: CodeEditorWhitespaceMode::Boundary,
    ..CodeEditorOptions::default()
};
"#;

#[cfg(test)]
mod tests {
    #[test]
    fn code_editor_demo_uses_component_api() {
        let source = include_str!("code_editor_demo.rs");
        assert!(source.contains("CodeEditor::new"));
        assert!(source.contains("CodeDiagnostic::warning"));
        assert!(source.contains("line_numbers"));
        assert!(source.contains("tab_size"));
        assert!(source.contains("on_change"));
        assert!(source.contains("CodeCompletionItem"));
        assert!(source.contains("CodeHover"));
        assert!(source.contains("search_query"));
        assert!(source.contains("CodeEditorOptions"));
        assert!(source.contains("read_only(true)"));
        assert!(source.contains("current_line_highlight(true)"));
        assert!(source.contains("CodeEditorHighlightTheme"));
        assert!(source.contains("CodeEditorWhitespaceMode"));
        assert!(source.contains("ruler_column"));
    }
}
