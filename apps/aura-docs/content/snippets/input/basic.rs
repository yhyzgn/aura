//! Basic Input fields.

use aura_components::{Input, Space};
use gpui::{Context, Entity, Render, Window, prelude::*};

struct InputBasicDemo {
    plain: Entity<Input>,
    placeholder: Entity<Input>,
}

impl InputBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            plain: cx.new(|cx| Input::new("", cx)),
            placeholder: cx.new(|cx| Input::new("", cx).placeholder("Type something...")),
        }
    }
}

impl Render for InputBasicDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        // Keep Input entities as fields so focus, cursor, and typed value persist.
        Space::new()
            .vertical()
            .gap_md()
            .child(self.plain.clone())
            .child(self.placeholder.clone())
    }
}

fn main() {}
