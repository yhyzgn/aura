//! Basic Anchor links bound to a ScrollHandle.

use aura_components::{Anchor, AnchorLink};
use gpui::ScrollHandle;

pub fn basic_anchor(scroll_handle: ScrollHandle) -> Anchor {
    Anchor::new(scroll_handle)
        .offset_sm()
        .link(AnchorLink::new("基础用法", "basic"))
        .link(AnchorLink::new("API", "api"))
}
