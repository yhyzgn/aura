use aura_core::AuraConfig;
use gpui::{App, Component, Hsla, IntoElement, RenderOnce, SharedString, Window, prelude::*, px};
use std::borrow::Cow;

pub trait IntoIconPath {
    fn icon_path(&self) -> Cow<'static, str>;
}

impl IntoIconPath for &str {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}
impl IntoIconPath for String {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.clone())
    }
}

pub struct Icon {
    size: Option<f32>,
    color: Option<Hsla>,
    group_hover_color: Option<(SharedString, Hsla)>,
    asset_path: String,
}

impl Icon {
    pub fn new(path: impl IntoIconPath) -> Self {
        Self {
            size: None,
            color: None,
            group_hover_color: None,
            asset_path: path.icon_path().into_owned(),
        }
    }

    pub fn size(mut self, px_size: f32) -> Self {
        self.size = Some(px_size);
        self
    }

    /// Set explicit color. If not called, inherits parent's text_color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Change icon color when a parent/group member is hovered.
    pub fn group_hover_color(mut self, group: impl Into<SharedString>, color: Hsla) -> Self {
        self.group_hover_color = Some((group.into(), color));
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<AuraConfig>().theme;

        let sz = self.size.unwrap_or(18.0);
        let mut el = gpui::svg().external_path(self.asset_path).size(px(sz));
        if let Some(color) = self.color {
            el = el.text_color(color);
        } else {
            el = el.text_color(theme.neutral.icon);
        }
        if let Some((group, color)) = self.group_hover_color {
            el = el.group_hover(group, move |style| style.text_color(color));
        }
        el
    }
}

impl IntoElement for Icon {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
