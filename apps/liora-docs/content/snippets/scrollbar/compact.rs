//! Compact scrollbar for constrained popup-like regions.

use gpui::{Context, IntoElement};
use liora_components::{Scrollbar, Space, Text};

pub fn compact_scrollbar(cx: &mut Context<Scrollbar>) -> Scrollbar {
    Scrollbar::new(cx, |_, _| {
        Space::new()
            .vertical()
            .gap_sm()
            .children((1..=18).map(|i| Text::new(format!("Compact item {}", i))))
            .into_any_element()
    })
    .height(120.0)
}
