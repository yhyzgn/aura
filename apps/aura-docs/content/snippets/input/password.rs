//! Password Input examples.

use aura_components::{Input, Space};
use gpui::{AppContext, Context, Entity, Render, Window};

struct InputPasswordDemo {
    password: Entity<Input>,
    custom_mask: Entity<Input>,
}

impl InputPasswordDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            password: cx.new(|cx| Input::new("", cx).password().placeholder("Password")),
            custom_mask: cx.new(|cx| Input::new("secret", cx).password().mask_char('*')),
        }
    }
}

impl Render for InputPasswordDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .vertical()
            .gap_md()
            .child(self.password.clone())
            .child(self.custom_mask.clone())
    }
}

fn main() {}
