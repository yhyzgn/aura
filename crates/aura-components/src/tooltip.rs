use aura_core::{Config, Placement, Popper, push_portal};
use gpui::{
    prelude::*, px, App, Component, IntoElement, RenderOnce, SharedString, Window,
    Bounds, Pixels, div, AnyElement,
};

pub struct Tooltip {
    trigger: AnyElement,
    content: SharedString,
    placement: Placement,
    offset: Pixels,
}

impl Tooltip {
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            content: SharedString::default(),
            placement: Placement::Top,
            offset: px(8.0),
        }
    }

    pub fn content(mut self, content: impl Into<SharedString>) -> Self {
        self.content = content.into();
        self
    }

    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let content = self.content.clone();
        let placement = self.placement;
        let offset = self.offset;

        div()
            .child(self.trigger)
            .on_mouse_move(move |event, _window, cx| {
                // Heuristic: create bounds around mouse position for now
                let anchor_bounds = Bounds {
                    origin: event.position,
                    size: gpui::Size { width: px(1.0), height: px(1.0) },
                };
                let content = content.clone();
                let theme = theme.clone();
                
                push_portal(move |window, cx| {
                    let viewport = Bounds {
                        origin: gpui::Point::default(),
                        size: window.viewport_size(),
                    };

                    let popper = Popper {
                        anchor_bounds,
                        placement,
                        offset,
                    };

                    let font_size = theme.font_size.sm as f32;
                    let padding = 16.0;
                    let width = (content.len() as f32 * font_size * 0.6 + padding).min(300.0);
                    let height = 32.0;
                    
                    let content_size = gpui::Size {
                        width: px(width),
                        height: px(height),
                    };

                    let (pos, _final_placement) = popper.calculate_position_with_flip(content_size, viewport);

                    div()
                        .absolute()
                        .top(pos.y)
                        .left(pos.x)
                        .bg(theme.neutral.text_1)
                        .text_color(theme.neutral.body)
                        .px_3().py_1()
                        .rounded(px(theme.radius.sm))
                        .shadow_md()
                        .text_size(px(theme.font_size.sm))
                        .child(content.clone())
                        .into_any_element()
                }, cx);
            })
    }
}

impl IntoElement for Tooltip {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
