//! Divider examples: plain horizontal, labeled, and vertical.

use aura_components::{Divider, Flex, Space, Text};
use gpui::IntoElement;

pub fn divider_examples() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(Text::new("Horizontal (default)"))
        .child(Divider::new())
        .child(Text::new("With label"))
        .child(Divider::new().label("Center Text"))
        .child(Text::new("Vertical"))
        .child(
            Flex::new()
                .row()
                .height_units(60.0)
                .gap_lg()
                .align_center()
                .child(Text::new("Left"))
                .child(Divider::new().vertical())
                .child(Text::new("Right")),
        )
}
