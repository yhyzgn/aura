use aura_components::{Dialog, Button, Space};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, px, AnyView};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DialogDemo).into()
}

struct DialogDemo;

impl Render for DialogDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Dialog 对话框"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("在保留当前页面状态的情况下，告知用户出现或重要的信息。"))
            )
            .child(
                div()
                    .child(
                        Button::new("Open Dialog").primary()
                            .on_click(|_, _, cx| {
                                Dialog::new()
                                    .title("Tips")
                                    .content(|_, _| {
                                        div().flex().flex_col().gap_4()
                                            .child("This is a message from the dialog.")
                                            .child(
                                                div().flex().justify_end()
                                                    .child(Button::new("Close").primary().on_click(|_, _, cx| {
                                                        aura_core::popper::clear_portals(cx);
                                                    }))
                                            )
                                    })
                                    .show(cx);
                            })
                    )
            )
    }
}
