//! Keep icon dimensions consistent with semantic size helpers.

use aura_components::{Space, Text};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{IntoElement, prelude::*};

pub fn icon_sizes() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(labeled(Icon::new(IconName::House).size_xs(), "12px"))
        .child(labeled(Icon::new(IconName::House).size_md(), "18px"))
        .child(labeled(Icon::new(IconName::House).size_lg(), "24px"))
        .child(labeled(Icon::new(IconName::House).size_xl(), "32px"))
}

fn labeled(icon: Icon, label: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .align_center()
        .gap_xs()
        .child(icon)
        .child(Text::new(label).nowrap())
}
