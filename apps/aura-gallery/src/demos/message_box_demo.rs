use aura_components::{alert, confirm, Button, Space};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, px, AnyView};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MessageBoxDemo).into()
}

struct MessageBoxDemo;

impl Render for MessageBoxDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("MessageBox 弹窗消息"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("模拟系统的消息提示框而设计。"))
            )
            .child(
                Space::new().gap(px(16.0))
                    .child(
                        Button::new("Open Alert")
                            .on_click(|_, _, cx| {
                                alert("Alert Title", "This is an alert message.", cx);
                            })
                    )
                    .child(
                        Button::new("Open Confirm").primary()
                            .on_click(|_, _, cx| {
                                confirm("Confirm Title", "Are you sure you want to proceed?", |_, _| println!("Confirmed in MB!"), cx);
                            })
                    )
            )
    }
}
