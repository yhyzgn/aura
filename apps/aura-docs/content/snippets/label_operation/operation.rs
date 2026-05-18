use aura_components::{Button, Label, Operation};
use aura_icons_lucide::IconName;
use gpui::IntoElement;

pub fn operation_basic() -> impl IntoElement {
    Operation::new(
        Label::new("执行操作").icon(IconName::Play),
        Button::new("Run").small(),
    )
}
