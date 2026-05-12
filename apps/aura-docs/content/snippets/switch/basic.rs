//! Basic Switch states.

use aura_components::{Space, Switch};
use gpui::{Context, Entity, Render, Window, prelude::*};

struct SwitchBasicDemo {
    on: Entity<Switch>,
    off: Entity<Switch>,
}

impl SwitchBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            on: cx.new(|cx| Switch::new(true, cx)),
            off: cx.new(|cx| Switch::new(false, cx)),
        }
    }
}

impl Render for SwitchBasicDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .wrap()
            .gap_sm()
            .child(self.on.clone())
            .child(self.off.clone())
    }
}

fn main() {}
