use aura_components::{Tag, TagFlow};
use gpui::{IntoElement, px};

pub fn tag_flow() -> impl IntoElement {
    // TagFlow 仍然使用 Tag 控件，只负责自动换行与间距。
    TagFlow::new([
        Tag::new("Design").round(true),
        Tag::new("GPUI").success().round(true),
        Tag::new("Animation").warning().round(true),
        Tag::new("Native Rust").danger().round(true),
        Tag::new("Charts").round(true),
    ])
    .gap(px(10.0))
}
