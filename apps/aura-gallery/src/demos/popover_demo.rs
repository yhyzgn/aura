use aura_components::{Popover, Button, Space};
use aura_core::{Placement, Config};
use gpui::{prelude::*, App, Context, Render, Window, div, px, AnyView};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PopoverDemo).into()
}

struct PopoverDemo;

impl Render for PopoverDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Popover 基础用法"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("点击按钮出现气泡卡片"))
            )
            .child(
                Space::new().gap(px(16.0))
                    .child(
                        Popover::new(Button::new("Bottom Center"))
                            .placement(Placement::Bottom)
                            .content(|_window, _cx| {
                                div().p_4().flex().flex_col().gap_2()
                                    .child(div().font_weight(gpui::FontWeight::BOLD).child("Title"))
                                    .child(div().child("This is the popover content."))
                                    .child(Button::new("Confirm").primary().small())
                            })
                    )
                    .child(
                        Popover::new(Button::new("Top Center"))
                            .placement(Placement::Top)
                            .content(|_window, _cx| {
                                div().p_4().flex().flex_col().gap_2()
                                    .child(div().child("Simple text content without title."))
                            })
                    )
            )
    }
}
