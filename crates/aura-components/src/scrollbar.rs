use gpui::{prelude::*, px, IntoElement, App, Component, ScrollHandle, RenderOnce, Window, DefiniteLength};

pub struct Scrollbar {
    height: Option<DefiniteLength>,
    children: Vec<gpui::AnyElement>,
}

impl Scrollbar {
    pub fn new() -> Self { Self { height: None, children: vec![] } }
    pub fn height(mut self, h: impl Into<DefiniteLength>) -> Self { self.height = Some(h.into()); self }
    pub fn child(mut self, el: impl IntoElement) -> Self {
        self.children.push(el.into_any_element()); self
    }
}

impl RenderOnce for Scrollbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;
        let child_count = self.children.len().max(1);
        // Thumb height ≈ container / total_lines ratio
        let visible_lines = 5.0;
        let thumb_ratio = (visible_lines / child_count as f32).clamp(0.05, 1.0);
        let thumb_h = (150.0 * thumb_ratio).max(20.0);

        let scroll_handle = ScrollHandle::new();

        let content = gpui::div()
            .flex_1().flex().flex_col()
            .id("scrollbar-content")
            .track_scroll(&scroll_handle)
            .overflow_y_scroll()
            .children(self.children);

        let thumb = gpui::div()
            .flex_none().w(px(6.0)).h(px(thumb_h))
            .bg(theme.neutral.border)
            .rounded(px(3.0))
            .cursor_pointer();

        let track = gpui::div()
            .flex_none().w(px(14.0)).h_full()
            .flex().flex_col().justify_start().items_center()
            .px(px(4.0)).py(px(2.0))
            .child(thumb);

        let mut outer = gpui::div().flex().flex_row().size_full();
        if let Some(h) = self.height { outer = outer.h(h); }
        outer.child(content).child(track)
    }
}

impl IntoElement for Scrollbar {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
