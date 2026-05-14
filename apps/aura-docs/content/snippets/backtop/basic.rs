//! Basic Backtop bound to a scroll handle.

use aura_components::Backtop;
use gpui::ScrollHandle;

pub fn basic_backtop(scroll_handle: ScrollHandle) -> Backtop {
    Backtop::new(scroll_handle)
        .id("docs-backtop-basic")
        .visibility_height_sm()
}
