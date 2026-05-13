//! Disabled Link state.

use aura_components::Link;

pub fn disabled_link() -> Link {
    Link::new("Disabled")
        .disabled(true)
        .href("https://github.com")
}
