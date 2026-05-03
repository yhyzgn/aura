use gpui::{prelude::*, px, Hsla, IntoElement, App, Component, RenderOnce, Window};
use std::borrow::Cow;

pub trait IntoIconPath {
    fn icon_path(&self) -> Cow<'static, str>;
}

impl IntoIconPath for &str {
    fn icon_path(&self) -> Cow<'static, str> { Cow::Owned(self.to_string()) }
}
impl IntoIconPath for String {
    fn icon_path(&self) -> Cow<'static, str> { Cow::Owned(self.clone()) }
}

pub struct AuraIcon {
    size: Option<f32>,
    color: Option<Hsla>,
    asset_path: String,
}

impl AuraIcon {
    pub fn new(path: impl IntoIconPath) -> Self {
        Self { size: None, color: None, asset_path: path.icon_path().into_owned() }
    }

    pub fn size(mut self, px_size: f32) -> Self { self.size = Some(px_size); self }
    pub fn color(mut self, color: Hsla) -> Self { self.color = Some(color); self }
}

impl RenderOnce for AuraIcon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let sz = self.size.unwrap_or(18.0);
        let mut el = gpui::svg()
            .external_path(self.asset_path)
            .size(px(sz));
        if let Some(color) = self.color {
            el = el.text_color(color);
        }
        el
    }
}

impl IntoElement for AuraIcon {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
