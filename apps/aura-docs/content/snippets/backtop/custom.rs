//! Custom Backtop button content and placement.

use aura_components::{Backtop, Flex};
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{IntoElement, ScrollHandle};

pub fn custom_backtop(scroll_handle: ScrollHandle) -> Backtop {
    Backtop::new(scroll_handle)
        .id("docs-backtop-custom")
        .right_lg()
        .content(|_, cx| {
            let theme = cx.global::<Config>().theme.clone();
            Flex::new()
                .size_full()
                .center()
                .bg(theme.primary.base)
                .child(
                    Icon::new(IconName::ArrowUp)
                        .size_md()
                        .color(theme.neutral.card),
                )
                .into_any_element()
        })
}
