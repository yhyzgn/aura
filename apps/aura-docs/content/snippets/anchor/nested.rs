//! Nested Anchor links for second-level sections.

use aura_components::{Anchor, AnchorLink};
use gpui::ScrollHandle;

pub fn nested_anchor(scroll_handle: ScrollHandle) -> Anchor {
    Anchor::new(scroll_handle).offset_sm().link(
        AnchorLink::new("API", "api")
            .child(AnchorLink::new("Attributes", "attributes"))
            .child(AnchorLink::new("Events", "events")),
    )
}
