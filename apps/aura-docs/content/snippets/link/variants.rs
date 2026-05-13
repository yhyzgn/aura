//! Link semantic variants.

use aura_components::{Link, Space};
use gpui::IntoElement;

pub fn link_variants() -> impl IntoElement {
    Space::new().wrap().gap_lg().children(vec![
        Link::new("Default").href("https://github.com"),
        Link::new("Primary").primary().href("https://github.com"),
        Link::new("Success").success().href("https://github.com"),
        Link::new("Warning").warning().href("https://github.com"),
        Link::new("Danger").danger().href("https://github.com"),
        Link::new("Info").info().href("https://github.com"),
    ])
}
