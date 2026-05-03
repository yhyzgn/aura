use aura_components::{Splitter, Text, Title};
use aura_core::Config;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement { Component::new(SplitterDemo).into_any_element() }

struct SplitterDemo;
impl RenderOnce for SplitterDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div().flex().flex_col().gap_2()
            .child(Title::new("Splitter 分隔面板").h2())
            .child(Text::new("Left panel + divider + right panel:"))
            .child(
                div().h(px(200.0)).border_1().border_color(theme.neutral.border).rounded(px(4.0))
                    .child(
                        Splitter::new()
                            .left(div().p_2().child(Text::new("Left panel")))
                            .right(div().p_2().child(Text::new("Right panel")))
                    )
            )
    }
}
