use aura_core::Config;
use aura_theme::Theme;
use gpui::{
    App, Component, IntoElement, RenderOnce, SharedString, Window,
    prelude::*, px,
};

pub struct Text {
    content: SharedString,
    size: Option<f32>,
    color: Option<gpui::Hsla>,
    truncate: bool,
}

impl Text {
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self { content: content.into(), size: None, color: None, truncate: true }
    }
    pub fn size(mut self, s: f32) -> Self { self.size = Some(s); self }
    pub fn color(mut self, c: gpui::Hsla) -> Self { self.color = Some(c); self }
    pub fn no_truncate(mut self) -> Self { self.truncate = false; self }

    fn render_with_theme(self, theme: &Theme) -> impl IntoElement {
        let fs = self.size.unwrap_or(theme.font_size.md);
        let color = self.color.unwrap_or(theme.neutral.text_2);

        let mut div = gpui::div().text_size(px(fs)).text_color(color);

        if self.truncate {
            div = div.whitespace_nowrap().text_ellipsis();
        }

        div.child(self.content.clone())
    }
}

impl RenderOnce for Text {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        self.render_with_theme(theme)
    }
}

impl IntoElement for Text {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
