use aura_core::{Config, Placement, Popper, set_active_popover, clear_active_popover};
use gpui::{
    prelude::*, px, App, Component, Context, IntoElement, Render, Window,
    Bounds, Pixels, div, AnyElement, MouseButton, RenderOnce, ElementId, LayoutId, GlobalElementId, InspectorElementId,
    SharedString,
};
use std::sync::Arc;
use std::rc::Rc;
use std::cell::Cell;

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
    pub fn new(
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
        
        let content = (self.content)(_window, cx);
        let viewport_size = _window.viewport_size();
        let viewport = Bounds {
            origin: gpui::Point::default(),
            size: viewport_size,
        };

        let popper = Popper {
            anchor_bounds,
            placement,
            offset,
        };

        // reference size for popper's internal flip logic
        let reference_size = gpui::Size {
            width: px(200.0),
            height: px(150.0),
        };

        let (pos, final_placement) = popper.calculate_position_with_flip(reference_size, viewport);

        let mut popover_div = div()
            .absolute()
            .on_mouse_down(MouseButton::Left, |_, _, _| {}) // Consume click
            .bg(theme.neutral.card)
            .border_1().border_color(theme.neutral.border)
            .rounded(px(theme.radius.md))
            .shadow_lg()
            .child(content);

        // Positioning logic based on final_placement
        match final_placement {
            Placement::Top | Placement::TopStart | Placement::TopEnd => {
                let dist_from_bottom = viewport_size.height - anchor_bounds.top() + offset;
                popover_div = popover_div.bottom(dist_from_bottom);
            }
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                popover_div = popover_div.top(anchor_bounds.bottom() + offset);
            }
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                let dist_from_right = viewport_size.width - anchor_bounds.left() + offset;
                popover_div = popover_div.right(dist_from_right);
            }
            Placement::Right | Placement::RightStart | Placement::RightEnd => {
                popover_div = popover_div.left(anchor_bounds.right() + offset);
            }
        }

        // Alignment
        match final_placement {
            Placement::Top | Placement::Bottom => {
                popover_div = popover_div.left(pos.x);
            }
            Placement::TopStart | Placement::BottomStart => {
                popover_div = popover_div.left(anchor_bounds.left());
            }
            Placement::TopEnd | Placement::BottomEnd => {
                let dist_from_right = viewport_size.width - anchor_bounds.right();
                popover_div = popover_div.right(dist_from_right);
            }
            Placement::Left | Placement::Right => {
                popover_div = popover_div.top(pos.y);
            }
            Placement::LeftStart | Placement::RightStart => {
                popover_div = popover_div.top(anchor_bounds.top());
            }
            Placement::LeftEnd | Placement::RightEnd => {
                let dist_from_bottom = viewport_size.height - anchor_bounds.bottom();
                popover_div = popover_div.bottom(dist_from_bottom);
            }
        }

        div()
            .absolute()
            .size_full()
            .on_mouse_down(MouseButton::Left, cx.listener(move |_, _, window, cx| {
                on_close(window, cx);
            }))
            .child(popover_div)
    }
}

impl Popover {
    #[track_caller]
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
    #[track_caller]
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let placement = self.placement;
        let offset = self.offset;
        let content = self.content.clone();
        
        let bounds_cell = Rc::new(Cell::new(None));
        let bounds_cell_clone = bounds_cell.clone();

        // Generate a stable ID based on caller location
        let caller = std::panic::Location::caller();
        let id = ElementId::from(SharedString::from(format!("popover-trigger-{}", caller)));

        div()
            .id(id)
            .child(
                BoundsTracker {
                    trigger: self.trigger,
                    bounds: bounds_cell,
                }
            )
            .on_click(move |_event, _window, cx| {
                if let Some(anchor_bounds) = bounds_cell_clone.get() {
                    let content = content.clone();
                    let view = cx.new(|_cx| {
                        PopoverView::new(
                            content,
                            anchor_bounds,
                            placement,
                            offset,
                            |_window, _cx| {
                                clear_active_popover(_cx);
                            }
                        )
                    });
                    set_active_popover(view.into(), cx);
                }
            })
    }
}

impl IntoElement for Popover {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

struct BoundsTracker {
    trigger: AnyElement,
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
}

impl IntoElement for BoundsTracker {
    type Element = Self;
    fn into_element(self) -> Self::Element { self }
}

impl gpui::Element for BoundsTracker {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> { None }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> { None }

    fn request_layout(&mut self, _id: Option<&GlobalElementId>, _id2: Option<&InspectorElementId>, window: &mut Window, cx: &mut App) -> (LayoutId, ()) {
        (self.trigger.request_layout(window, cx), ())
    }

    fn prepaint(&mut self, _id: Option<&GlobalElementId>, _id2: Option<&InspectorElementId>, _bounds: Bounds<Pixels>, _rl: &mut (), window: &mut Window, cx: &mut App) -> () {
        self.trigger.prepaint(window, cx);
    }

    fn paint(&mut self, _id: Option<&GlobalElementId>, _id2: Option<&InspectorElementId>, bounds: Bounds<Pixels>, _rl: &mut (), _ps: &mut (), window: &mut Window, cx: &mut App) {
        self.bounds.set(Some(bounds));
        self.trigger.paint(window, cx);
    }
}
