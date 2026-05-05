use aura_components::{Popconfirm, Button, Space};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, px, AnyView};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PopconfirmDemo).into()
}

struct PopconfirmDemo;

impl Render for PopconfirmDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Popconfirm 气泡确认框"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("点击按钮出现气泡确认框"))
            )
            .child(
                Space::new().gap(px(16.0))
                    .child(
                        Popconfirm::new(Button::new("Delete").danger())
                            .title("Are you sure to delete this task?")
                            .on_confirm(|_, _| println!("Confirmed!"))
                            .on_cancel(|_, _| println!("Cancelled!"))
                    )
                    .child(
                        Popconfirm::new(Button::new("Archive"))
                            .title("Archive this item?")
                            .confirm_text("Yes")
                            .cancel_text("No")
                    )
            )
    }
}
