use aura_components::AuraLink;
use aura_core::AuraConfig;
use aura_icons_lucide::IconName;
use aura_theme::AuraTheme;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement {
    Component::new(LinkDemo).into_any_element()
}

struct LinkDemo;

impl RenderOnce for LinkDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<AuraConfig>().theme;
        div().flex().flex_col().gap_3()
            .child(hdr(theme, "Variants"))
            .child(row(vec![
                AuraLink::new("Default"),
                AuraLink::new("Primary").primary(),
                AuraLink::new("Success").success(),
                AuraLink::new("Warning").warning(),
                AuraLink::new("Danger").danger(),
                AuraLink::new("Info").info(),
            ]))
            .child(hdr(theme, "Underline"))
            .child(row(vec![
                AuraLink::new("With underline"),
                AuraLink::new("No underline").underline(false),
            ]))
            .child(hdr(theme, "Disabled"))
            .child(row(vec![
                AuraLink::new("Disabled link").disabled(true),
            ]))
            .child(hdr(theme, "With icons"))
            .child(row(vec![
                AuraLink::new("External").icon_end(IconName::ArrowRight),
                AuraLink::new("Home").icon_start(IconName::House),
            ]))
    }
}

fn hdr(theme: &AuraTheme, s: &str) -> impl IntoElement {
    div().text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD).mt_2().child(s.to_string())
}

fn row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    div().flex().flex_row().gap_4().items_center().flex_wrap().children(elements)
}
