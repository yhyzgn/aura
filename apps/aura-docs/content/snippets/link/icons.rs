//! Links with leading icons.

use aura_components::{Link, Space};
use aura_icons_lucide::IconName;
use gpui::IntoElement;

pub fn icon_links() -> impl IntoElement {
    Space::new().wrap().gap_lg().children(vec![
        Link::new("GitHub")
            .icon_start(IconName::ExternalLink)
            .href("https://github.com"),
        Link::new("Home")
            .icon_start(IconName::House)
            .href("https://example.com"),
    ])
}
