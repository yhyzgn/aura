use aura_components::{Affix, AffixPosition, Button, ButtonVariant};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| AffixDemo {
        top_affix: cx.new(|_| {
            Affix::new().offset(px(80.0)).content(|_, _| {
                Button::new("固钉在距离顶部 80px 的位置")
                    .variant(ButtonVariant::Primary)
                    .into_any_element()
            })
        }),
        bottom_affix: cx.new(|_| {
            Affix::new()
                .position(AffixPosition::Bottom)
                .offset(px(20.0))
                .content(|_, _| {
                    Button::new("固钉在距离底部 20px 的位置")
                        .variant(ButtonVariant::Success)
                        .into_any_element()
                })
        }),
    })
    .into()
}

struct AffixDemo {
    top_affix: Entity<Affix>,
    bottom_affix: Entity<Affix>,
}

impl Render for AffixDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Affix 固钉"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("将内容固定在特定可视区域。"),
                    ),
            )
            .child(
                div()
                    .relative()
                    .h(px(560.0))
                    .overflow_hidden()
                    .border_1()
                    .border_color(theme.neutral.border)
                    .rounded(px(theme.radius.md))
                    .bg(theme.neutral.hover)
                    .child(
                        div()
                            .size_full()
                            .id("affix-scroll-view")
                            .overflow_y_scroll()
                            .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                                cx.notify();
                            }))
                            .p_4()
                            .child(
                                div()
                                    .h(px(200.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.neutral.text_3)
                                            .child("向下滚动查看固钉效果"),
                                    ),
                            )
                            .child(self.top_affix.clone())
                            .child(
                                div()
                                    .h(px(800.0))
                                    .bg(theme.neutral.card)
                                    .my_4()
                                    .border_1()
                                    .border_color(theme.neutral.border)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(div().child("长内容占位")),
                            )
                            .child(self.bottom_affix.clone())
                            .child(div().h(px(400.0))),
                    ),
            )
    }
}
