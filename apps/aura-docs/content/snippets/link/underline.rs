//! Link underline control.

use aura_components::{Link, Space};
use gpui::IntoElement;

pub fn underline_links() -> impl IntoElement {
    Space::new().wrap().gap_lg().children(vec![
        Link::new("With underline").href("https://github.com"),
        Link::new("No underline")
            .underline(false)
            .href("https://github.com"),
    ])
}
