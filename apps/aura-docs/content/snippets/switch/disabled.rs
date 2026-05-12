//! Disabled Switch states.

use aura_components::{Space, Switch};
use gpui::{Context, Entity, Render, Window, prelude::*};

struct SwitchDisabledDemo {
    off: Entity<Switch>,
    on: Entity<Switch>,
}

impl SwitchDisabledDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            off: cx.new(|cx| Switch::new(false, cx).disabled(true)),
            on: cx.new(|cx| Switch::new(true, cx).disabled(true)),
        }
    }
}

impl Render for SwitchDisabledDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .wrap()
            .gap_sm()
            .child(self.off.clone())
            .child(self.on.clone())
    }
}

fn main() {}
