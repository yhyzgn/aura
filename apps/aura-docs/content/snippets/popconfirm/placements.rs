//! Popconfirm placement variants.

use aura_components::{Button, Popconfirm, Space};
use aura_core::Placement;
use gpui::IntoElement;

pub fn popconfirm_placements() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        confirm_at("Top", Placement::Top),
        confirm_at("Bottom", Placement::Bottom),
        confirm_at("Left", Placement::Left),
        confirm_at("Right", Placement::Right),
        confirm_at("BottomEnd", Placement::BottomEnd),
    ])
}

fn confirm_at(label: &'static str, placement: Placement) -> Popconfirm {
    Popconfirm::new(Button::new(label).small())
        .id(format!("docs-popconfirm-placement-{label}"))
        .title(format!("Confirm at {placement:?}?"))
        .placement(placement)
}
