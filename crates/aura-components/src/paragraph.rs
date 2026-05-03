use aura_core::Config;
use gpui::{
    App, Component, IntoElement, RenderOnce, SharedString, Window,
    prelude::*, px,
};

pub struct Paragraph {
    content: SharedString,
}

impl Paragraph {
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self { content: content.into() }
    }

    fn render_with_theme(self, theme: &aura_theme::Theme) -> impl IntoElement {
        gpui::div()
            .text_size(px(theme.font_size.md))
            .text_color(theme.neutral.text_2)
            .line_height(px(theme.font_size.md * 1.6))
            .child(self.content.clone())
    }
}

impl RenderOnce for Paragraph {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        self.render_with_theme(theme)
    }
}

impl IntoElement for Paragraph {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
