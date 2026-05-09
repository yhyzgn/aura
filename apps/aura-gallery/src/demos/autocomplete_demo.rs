use aura_components::{Autocomplete, AutocompleteItem, Card};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| AutocompleteDemo::new(cx)).into()
}

struct AutocompleteDemo {
    basic: Entity<Autocomplete>,
    custom: Entity<Autocomplete>,
    disabled: Entity<Autocomplete>,
}

impl AutocompleteDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let suggestions = vec![
            AutocompleteItem::labeled("rust", "Rust"),
            AutocompleteItem::labeled("gpui", "GPUI"),
            AutocompleteItem::labeled("aura", "Aura UI"),
            AutocompleteItem::labeled("element-plus", "Element Plus"),
            AutocompleteItem::labeled("autocomplete", "Autocomplete"),
        ];

        Self {
            basic: cx.new({
                let suggestions = suggestions.clone();
                move |cx| Autocomplete::new(suggestions, cx).placeholder("Search component")
            }),
            custom: cx.new({
                let suggestions = vec![
                    AutocompleteItem::labeled("/dashboard", "Dashboard"),
                    AutocompleteItem::labeled("/settings", "Settings"),
                    AutocompleteItem::labeled("/profile", "Profile"),
                    AutocompleteItem::labeled("/billing", "Billing"),
                ];
                move |cx| {
                    Autocomplete::new(suggestions, cx)
                        .placeholder("Jump to route")
                        .width(px(320.0))
                        .max_suggestions(4)
                }
            }),
            disabled: cx.new({
                let suggestions = suggestions.clone();
                move |cx| {
                    Autocomplete::new(suggestions, cx)
                        .placeholder("Disabled")
                        .disabled(true)
                }
            }),
        }
    }
}

impl Render for AutocompleteDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        div()
            .flex()
            .flex_col()
            .gap_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Autocomplete 自动补全"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("输入时展示匹配建议，点击选项回填输入框。"),
                    ),
            )
            .child(section(
                "基础用法",
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(self.basic.clone())
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("Try: rust, gpui, aura"),
                    ),
            ))
            .child(section(
                "自定义建议",
                Card::new(
                    div()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(self.custom.clone())
                        .child(div().text_sm().text_color(theme.neutral.text_3).child(
                            "Value and label can be different, useful for routes or commands.",
                        )),
                )
                .no_shadow(),
            ))
            .child(section("禁用状态", self.disabled.clone()))
    }
}

fn section(title: &'static str, content: impl IntoElement) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(content)
}
