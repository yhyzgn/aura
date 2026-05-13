//! Basic Loading indicators.

use aura_components::{Loading, Space};
use gpui::IntoElement;

pub fn basic_loading() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_lg()
        .child(Loading::new())
        .child(Loading::new().text("Loading..."))
}
