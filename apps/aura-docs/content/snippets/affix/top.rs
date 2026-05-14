//! Top Affix fixed after the scroll container passes the offset.

use aura_components::{Affix, Button};
use gpui::{IntoElement, px};

pub fn top_affix() -> Affix {
    Affix::new().offset(px(80.0)).content(|_, _| {
        Button::new("固钉在距离顶部 80px 的位置")
            .primary()
            .into_any_element()
    })
}

fn main() {
    let _ = top_affix();
}
