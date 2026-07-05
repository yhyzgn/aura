use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{
    CodeCompletionItem, CodeDiagnostic, CodeEditor, CodeHover, CodeLanguage, CodeTheme, Space,
    Text, toast_info,
};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| CodeEditorDemo::new(cx)).into()
}

struct CodeEditorDemo {
    basic: Entity<CodeEditor>,
    diagnostics: Entity<CodeEditor>,
    advanced: Entity<CodeEditor>,
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

        Self {
            basic,
            diagnostics,
            advanced,
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
                            .child(Text::new("v2 基座已完成基础输入、导航、选择、行号、缩进元数据、虚拟行渲染和诊断渲染；后续继续增强 undo/redo、display map、Tree-sitter 与 LSP bridge。")),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "高级扩展点：搜索、补全和 hover",
                        "Provider-ready 的搜索、补全候选和 hover/help 数据模型可接入语言服务。",
                        self.advanced.clone(),
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
    }
}
