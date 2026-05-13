//! Divider examples for page-level content sections.

use aura_components::{Divider, Flex, Space, Text};
use gpui::IntoElement;

pub fn container_divider_examples() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(
            Space::new()
                .vertical()
                .child(Text::new("Above divider"))
                .child(Divider::new())
                .child(Text::new("Below divider")),
        )
        .child(Divider::new().label("Center Label"))
        .child(
            Flex::new()
                .row()
                .align_center()
                .gap_lg()
                .height_units(48.0)
                .child(Text::new("Section 1"))
                .child(Divider::new().vertical())
                .child(Text::new("Section 2"))
                .child(Divider::new().vertical())
                .child(Text::new("Section 3")),
        )
}
