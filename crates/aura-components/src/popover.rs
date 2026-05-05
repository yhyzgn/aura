use aura_core::{Config, Placement, Popper, push_portal};
use gpui::{
    prelude::*, px, App, Component, Context, IntoElement, Render, Window,
    Bounds, Pixels, div, AnyElement, MouseButton,
};
use std::sync::Arc;

pub struct Popover {
    trigger: AnyElement,
    content: Arc<dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement + 'static>,
    placement: Placement,
    offset: Pixels,
}

pub struct PopoverView {
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    anchor_bounds: Bounds<Pixels>,
    placement: Placement,
    offset: Pixels,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl PopoverView {
    fn new(
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        anchor_bounds: Bounds<Pixels>,
        placement: Placement,
        offset: Pixels,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            content,
            anchor_bounds,
            placement,
            offset,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for PopoverView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let anchor_bounds = self.anchor_bounds;
        let placement = self.placement;
        let offset = self.offset;
        let on_close = self.on_close.clone();
        
        // Render the content view
        let content = (self.content)(_window, cx);
        
        div()
            .absolute()
            .size_full()
            .on_mouse_down_out(cx.listener(move |_, _, window, cx| {
                on_close(window, cx);
            }))
            .child({
                push_portal(move |window, _cx| {
                    let viewport = Bounds {
                        origin: gpui::Point::default(),
                        size: window.viewport_size(),
                    };

                    let popper = Popper {
                        anchor_bounds,
                        placement,
                        offset,
                    };

                    let content_size = gpui::Size {
                        width: px(200.0),
                        height: px(150.0),
                    };

                    let (pos, _final_placement) = popper.calculate_position_with_flip(content_size, viewport);

                    div()
                        .absolute()
                        .top(pos.y)
                        .left(pos.x)
                        .w(content_size.width)
                        .bg(theme.neutral.card)
                        .border_1().border_color(theme.neutral.border)
                        .rounded(px(theme.radius.md))
                        .shadow_lg()
                        .child(content)
                        .into_any_element()
                }, cx);
                
                div()
            })
    }
}

impl Popover {
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            content: Arc::new(|_, _| div().child("Popover Content").into_any_element()),
            placement: Placement::Bottom,
            offset: px(8.0),
        }
    }

    pub fn content<F, E>(mut self, f: F) -> Self 
    where 
        F: Fn(&mut Window, &mut Context<PopoverView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| f(window, cx).into_any_element());
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

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let placement = self.placement;
        let offset = self.offset;
        let content = self.content.clone();
        
        div()
            .child(self.trigger)
            .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
                let anchor_bounds = Bounds {
                    origin: event.position,
                    size: gpui::Size { width: px(1.0), height: px(1.0) },
                };
                let content = content.clone();
                cx.new(|_cx| {
                    PopoverView::new(
                        content,
                        anchor_bounds,
                        placement,
                        offset,
                        |_window, _cx| {
                            aura_core::popper::clear_portals(_cx);
                        }
                    )
                });
            })
    }
}

impl IntoElement for Popover {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
