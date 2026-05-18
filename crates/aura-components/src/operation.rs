use crate::Label;
use aura_core::Config;
use gpui::{
    AnyElement, App, Component, IntoElement, Pixels, RenderOnce, Window, div, prelude::*, px,
};

pub struct Operation {
    label: AnyElement,
    action: AnyElement,
    gap: Pixels,
    padded: bool,
}

impl Operation {
    pub fn new(label: impl IntoElement, action: impl IntoElement) -> Self {
        Self {
            label: label.into_any_element(),
            action: action.into_any_element(),
            gap: px(16.0),
            padded: true,
        }
    }

    pub fn with_text(text: impl Into<gpui::SharedString>, action: impl IntoElement) -> Self {
        Self::new(Label::new(text), action)
    }
    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = gap.max(px(0.0));
        self
    }
    pub fn no_padding(mut self) -> Self {
        self.padded = false;
        self
    }
}

impl RenderOnce for Operation {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(self.gap)
            .w_full()
            .when(self.padded, |s| {
                s.p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.card)
            })
            .child(div().min_w_0().child(self.label))
            .child(div().flex_none().child(self.action))
    }
}

impl IntoElement for Operation {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn operation_tracks_layout_options() {
        let op = Operation::with_text("Auto save", div())
            .gap(px(20.0))
            .no_padding();
        assert_eq!(op.gap, px(20.0));
        assert!(!op.padded);
    }
}
