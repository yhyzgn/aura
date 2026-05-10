use aura_components::{Scrollbar, Space, Text};
use gpui::{App, Entity, prelude::*};

pub fn render(cx: &mut App) -> Entity<Scrollbar> {
    cx.new(|cx| {
        Scrollbar::new(cx, |_, _| {
            let items = (1..=20).map(|i| Text::new(format!("Scrollable line {}", i)));

            Space::new().vertical().gap_lg().children(items)
        })
        .height(150.0)
    })
}
