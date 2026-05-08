use aura_components::Backtop;
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Entity, Render, ScrollHandle, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| BacktopDemo::new(cx)).into()
}

struct BacktopDemo {
    scroll_handle: ScrollHandle,
    primary: Entity<Backtop>,
    custom: Entity<Backtop>,
}

impl BacktopDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        Self {
            primary: cx.new({
                let scroll_handle = scroll_handle.clone();
                |_| {
                    Backtop::new(scroll_handle)
                        .id("backtop-demo-primary")
                        .visibility_height(px(100.0))
                }
            }),
            custom: cx.new({
                let scroll_handle = scroll_handle.clone();
                |_| {
                    Backtop::new(scroll_handle)
                        .id("backtop-demo-custom")
                        .visibility_height(px(200.0))
                        .right(px(100.0))
                        .content(|_, _| {
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .size_full()
                                .bg(gpui::blue())
                                .child(
                                    Icon::new(IconName::ArrowUp)
                                        .size(px(20.0))
                                        .color(gpui::white()),
                                )
                                .into_any_element()
                        })
                }
            }),
            scroll_handle,
        }
    }
}

impl Render for BacktopDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let scroll_handle = self.scroll_handle.clone();

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
                            .child("Backtop 回到顶部"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("向下滚动查看右下角的回到顶部按钮。"),
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
                    .child(
                        div()
                            .size_full()
                            .id("backtop-scroll-view")
                            .overflow_y_scroll()
                            .track_scroll(&scroll_handle)
                            .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                                cx.notify();
                            }))
                            .child(div().flex().flex_col().gap_4().p_4().children((0..50).map(
                                |i| {
                                    div()
                                        .h(px(40.0))
                                        .flex()
                                        .items_center()
                                        .px_4()
                                        .bg(theme.neutral.hover)
                                        .rounded(px(theme.radius.sm))
                                        .child(format!("Scroll Item {}", i))
                                },
                            ))),
                    )
                    .child(self.primary.clone())
                    .child(self.custom.clone()),
            )
    }
}
