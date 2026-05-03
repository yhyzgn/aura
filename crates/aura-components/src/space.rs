use gpui::{prelude::*, px, IntoElement, App, Component, DefiniteLength, RenderOnce, Window};

pub struct Space {
    w: Option<DefiniteLength>,
    h: Option<DefiniteLength>,
}

impl Space {
    pub fn horizontal(w: impl Into<DefiniteLength>) -> Self { Self { w: Some(w.into()), h: None } }
    pub fn vertical(h: impl Into<DefiniteLength>) -> Self   { Self { w: None, h: Some(h.into()) } }
}

impl RenderOnce for Space {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut div = gpui::div().flex_none();
        if let Some(w) = self.w { div = div.w(w); }
        if let Some(h) = self.h { div = div.h(h); }
        div
    }
}

impl IntoElement for Space {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
