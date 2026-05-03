use aura_components::AuraButton;
use aura_core::AuraConfig;
use aura_theme::AuraTheme;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement {
    Component::new(ButtonDemo).into_any_element()
}

struct ButtonDemo;

impl RenderOnce for ButtonDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<AuraConfig>().theme;
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(hdr(theme, "Types 按钮类型"))
            .child(row(types()))
            .child(hdr(theme, "Secondary 次要按钮"))
            .child(row(secondary()))
            .child(hdr(theme, "Secondary · no border"))
            .child(row(secondary_nb()))
            .child(hdr(theme, "Sizes 尺寸"))
            .child(row(sizes()))
            .child(hdr(theme, "States 状态"))
            .child(row(states()))
            .child(hdr(theme, "Rounded 圆角"))
            .child(row(rounded()))
    }
}

fn hdr(theme: &AuraTheme, s: &str) -> impl IntoElement {
    div()
        .text_size(px(theme.font_size.lg))
        .text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD)
        .mt_2()
        .child(s.to_string())
}

fn row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .gap_2()
        .items_center()
        .flex_wrap()
        .children(elements)
}

fn types() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Default"),
        AuraButton::new("Tertiary").tertiary(),
        AuraButton::new("Primary").primary(),
        AuraButton::new("Info").info(),
        AuraButton::new("Success").success(),
        AuraButton::new("Warning").warning(),
        AuraButton::new("Error").danger(),
    ]
}

fn secondary() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Default").secondary(),
        AuraButton::new("Tertiary").tertiary().secondary(),
        AuraButton::new("Primary").primary().secondary(),
        AuraButton::new("Info").info().secondary(),
        AuraButton::new("Success").success().secondary(),
        AuraButton::new("Warning").warning().secondary(),
        AuraButton::new("Error").danger().secondary(),
    ]
}

fn secondary_nb() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Default").secondary().border(false),
        AuraButton::new("Primary")
            .primary()
            .secondary()
            .border(false),
        AuraButton::new("Info").info().secondary().border(false),
        AuraButton::new("Success")
            .success()
            .secondary()
            .border(false),
        AuraButton::new("Warning")
            .warning()
            .secondary()
            .border(false),
        AuraButton::new("Error").danger().secondary().border(false),
    ]
}

fn sizes() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Small").primary().small(),
        AuraButton::new("Default").primary(),
        AuraButton::new("Large").primary().large(),
    ]
}

fn states() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("Disabled").primary().disabled(true),
        AuraButton::new("Loading").primary().loading(true),
        AuraButton::new("Sec Disabled")
            .primary()
            .secondary()
            .disabled(true),
    ]
}

fn rounded() -> Vec<impl IntoElement> {
    vec![
        AuraButton::new("4px").primary().rounded(4.0),
        AuraButton::new("12px").primary().rounded(12.0),
        AuraButton::new("20px").primary().rounded(20.0),
        AuraButton::new("Pill").primary().rounded(9999.0),
    ]
}
