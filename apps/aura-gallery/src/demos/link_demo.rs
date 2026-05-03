use aura_components::Link;
use aura_core::Config;
use aura_icons_lucide::IconName;
use aura_theme::Theme;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement { Component::new(LinkDemo).into_any_element() }

struct LinkDemo;
impl RenderOnce for LinkDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        div().flex().flex_col().gap_3()
            .child(hdr(theme, "Variants 类型"))
            .child(row(vec![
                Link::new("Default").href("https://github.com"),
                Link::new("Primary").primary().href("https://github.com"),
                Link::new("Success").success().href("https://github.com"),
                Link::new("Warning").warning().href("https://github.com"),
                Link::new("Danger").danger().href("https://github.com"),
                Link::new("Info").info().href("https://github.com".to_string()),
            ]))
            .child(hdr(theme, "Underline 下划线"))
            .child(row(vec![
                Link::new("With underline").href("https://github.com"),
                Link::new("No underline").underline(false).href("https://github.com"),
            ]))
            .child(hdr(theme, "Disabled 禁用"))
            .child(row(vec![
                Link::new("Disabled").disabled(true).href("https://github.com"),
            ]))
            .child(hdr(theme, "Icons 图标"))
            .child(row(vec![
                Link::new("GitHub").icon_start(IconName::ExternalLink).href("https://github.com"),
                Link::new("Home").icon_start(IconName::House).href("https://example.com"),
            ]))
    }
}

fn hdr(theme: &Theme, s: &str) -> impl IntoElement {
    div().text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD).mt_2().child(s.to_string())
}

fn row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    div().flex().flex_row().gap_4().items_center().flex_wrap().children(elements)
}
