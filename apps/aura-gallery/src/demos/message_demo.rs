use aura_components::{Button, toastError, toastInfo, toastSuccess, toastWarning};
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, row};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MessageDemo).into()
}

struct MessageDemo;

impl Render for MessageDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Message 全局提示",
            "常用于主动操作后的反馈提示。",
            row(vec![
                Button::new("Info Message").on_click(|_, _, _| {
                    toastInfo!("This is an info message");
                }),
                Button::new("Success Message")
                    .primary()
                    .on_click(|_, _, _| {
                        toastSuccess!("Congrats! Operation success.");
                    }),
                Button::new("Warning Message")
                    .warning()
                    .on_click(|_, _, _| {
                        toastWarning!("Be careful! This is a {}.", "warning");
                    }),
                Button::new("Error Message").danger().on_click(|_, _, _| {
                    toastError!("Oops! {} went wrong.", "Something");
                }),
            ]),
        )
    }
}
