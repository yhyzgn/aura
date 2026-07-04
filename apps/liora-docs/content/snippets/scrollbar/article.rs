//! Scrollbar for long readable text.

use gpui::{Context, IntoElement};
use liora_components::{Scrollbar, Space, Text};

pub fn article_scrollbar(cx: &mut Context<Scrollbar>) -> Scrollbar {
    Scrollbar::new(cx, |_, _| {
        Space::new()
            .vertical()
            .gap_lg()
            .children((1..=8).map(|i| {
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(Text::new(format!("Section {}", i)).bold())
                    .child(Text::new("Long wrapped text remains selectable while the scrollbar reports a stable thumb height and position.").wrap())
            }))
            .into_any_element()
    })
    .height(260.0)
}
