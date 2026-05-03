use aura_components::Text;
use aura_core::Config;
use aura_theme::Theme;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement { Component::new(TextDemo).into_any_element() }

struct TextDemo;
impl RenderOnce for TextDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        div().flex().flex_col().gap_3()
            .child(hdr(theme, "Sizes"))
            .child(row(vec![
                Text::new("xs text").size(theme.font_size.xs),
                Text::new("sm text").size(theme.font_size.sm),
                Text::new("md text (default)"),
                Text::new("lg text").size(theme.font_size.lg),
                Text::new("xl text").size(theme.font_size.xl),
            ]))
            .child(hdr(theme, "Truncate"))
            .child(div().w(px(200.0)).child(
                Text::new("This is a very long text that should be truncated with ellipsis at the end")
            ))
            .child(hdr(theme, "No truncate"))
            .child(div().w(px(200.0)).child(
                Text::new("This text wraps naturally without truncation").no_truncate()
            ))
    }
}

fn hdr(theme: &Theme, s: &str) -> impl IntoElement {
    div().text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD).mt_2().child(s.to_string())
}

fn row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    div().flex().flex_row().gap_4().items_center().flex_wrap().children(elements)
}
