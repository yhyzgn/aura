use aura_core::{Config, Placement, set_active_popover, clear_active_popover};
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
    close_on_click_outside: bool,
}

pub struct PopoverView {
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    anchor_bounds: Bounds<Pixels>,
    placement: Placement,
    offset: Pixels,
    close_on_click_outside: bool,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl PopoverView {
    pub fn new(
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        anchor_bounds: Bounds<Pixels>,
        placement: Placement,
        offset: Pixels,
        close_on_click_outside: bool,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            content,
            anchor_bounds,
            placement,
            offset,
            close_on_click_outside,
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
        let close_on_click_outside = self.close_on_click_outside;
        
        let content = (self.content)(_window, cx);
        let viewport_size = _window.viewport_size();
        let viewport = Bounds {
            origin: gpui::Point::default(),
            size: viewport_size,
        };

        // Use Popper just for flip logic
        let popper = aura_core::Popper {
            anchor_bounds,
            placement,
            offset,
        };
        
        let reference_size = gpui::Size { width: px(400.0), height: px(300.0) };
        let (_pos, final_placement) = popper.calculate_position_with_flip(reference_size, viewport);

        let mut pivot_container = div().absolute().flex();

        let ideal_center_x = anchor_bounds.left() + anchor_bounds.size.width / 2.0;
        let ideal_center_y = anchor_bounds.top() + anchor_bounds.size.height / 2.0;
        let half_w = reference_size.width / 2.0;
        let half_h = reference_size.height / 2.0;

        let clamped_center_x = ideal_center_x.max(half_w).min(viewport_size.width - half_w);
        let clamped_center_y = ideal_center_y.max(half_h).min(viewport_size.height - half_h);
        let clamped_left = anchor_bounds.left().max(px(0.0)).min(viewport_size.width - reference_size.width);
        let clamped_right = anchor_bounds.right().min(viewport_size.width).max(reference_size.width);
        let clamped_top = anchor_bounds.top().max(px(0.0)).min(viewport_size.height - reference_size.height);
        let clamped_bottom = anchor_bounds.bottom().min(viewport_size.height).max(reference_size.height);

        // Bounding Box Layout Strategy:
        // We create an exact container that bounds the available space for the popover to grow into.
        // We then use flex alignment (justify_end/start and items_center) to perfectly position the content within this box.
        match final_placement {
            Placement::Top => {
                pivot_container = pivot_container
                    .left(clamped_center_x - px(1000.0)).w(px(2000.0))
                    .top(px(0.0)).h(anchor_bounds.top() - offset)
                    .flex_col().justify_end().items_center();
            }
            Placement::Bottom => {
                pivot_container = pivot_container
                    .left(clamped_center_x - px(1000.0)).w(px(2000.0))
                    .top(anchor_bounds.bottom() + offset).h(viewport_size.height - anchor_bounds.bottom() - offset)
                    .flex_col().justify_start().items_center();
            }
            Placement::Left => {
                pivot_container = pivot_container
                    .top(clamped_center_y - px(1000.0)).h(px(2000.0))
                    .left(px(0.0)).w(anchor_bounds.left() - offset)
                    .flex_row().justify_end().items_center();
            }
            Placement::Right => {
                pivot_container = pivot_container
                    .top(clamped_center_y - px(1000.0)).h(px(2000.0))
                    .left(anchor_bounds.right() + offset).w(viewport_size.width - anchor_bounds.right() - offset)
                    .flex_row().justify_start().items_center();
            }
            Placement::TopStart => {
                pivot_container = pivot_container
                    .left(clamped_left).w(viewport_size.width - clamped_left)
                    .top(px(0.0)).h(anchor_bounds.top() - offset)
                    .flex_col().justify_end().items_start();
            }
            Placement::TopEnd => {
                pivot_container = pivot_container
                    .left(px(0.0)).w(clamped_right)
                    .top(px(0.0)).h(anchor_bounds.top() - offset)
                    .flex_col().justify_end().items_end();
            }
            Placement::BottomStart => {
                pivot_container = pivot_container
                    .left(clamped_left).w(viewport_size.width - clamped_left)
                    .top(anchor_bounds.bottom() + offset).h(viewport_size.height - anchor_bounds.bottom() - offset)
                    .flex_col().justify_start().items_start();
            }
            Placement::BottomEnd => {
                pivot_container = pivot_container
                    .left(px(0.0)).w(clamped_right)
                    .top(anchor_bounds.bottom() + offset).h(viewport_size.height - anchor_bounds.bottom() - offset)
                    .flex_col().justify_start().items_end();
            }
            Placement::LeftStart => {
                pivot_container = pivot_container
                    .top(clamped_top).h(viewport_size.height - clamped_top)
                    .left(px(0.0)).w(anchor_bounds.left() - offset)
                    .flex_row().justify_end().items_start();
            }
            Placement::LeftEnd => {
                pivot_container = pivot_container
                    .top(px(0.0)).h(clamped_bottom)
                    .left(px(0.0)).w(anchor_bounds.left() - offset)
                    .flex_row().justify_end().items_end();
            }
            Placement::RightStart => {
                pivot_container = pivot_container
                    .top(clamped_top).h(viewport_size.height - clamped_top)
                    .left(anchor_bounds.right() + offset).w(viewport_size.width - anchor_bounds.right() - offset)
                    .flex_row().justify_start().items_start();
            }
            Placement::RightEnd => {
                pivot_container = pivot_container
                    .top(px(0.0)).h(clamped_bottom)
                    .left(anchor_bounds.right() + offset).w(viewport_size.width - anchor_bounds.right() - offset)
                    .flex_row().justify_start().items_end();
            }
        }

        div()
            .absolute()
            .size_full()
            .when(close_on_click_outside, |s| s.on_mouse_down(MouseButton::Left, cx.listener(move |_, _, window, cx| {
                on_close(window, cx);
            })))
            .child(
                pivot_container
                    .child(
                        div()
                            .flex_shrink_0() // Ensure content is not squeezed by flex layout
                            .on_mouse_down(MouseButton::Left, |_, _, _| {}) // Consume click so it doesn't trigger the background
                            .bg(theme.neutral.card)
                            .border_1().border_color(theme.neutral.border)
                            .rounded(px(theme.radius.md))
                            .shadow_lg()
                            .child(content)
                    )
            )
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
            close_on_click_outside: true,
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

    pub fn close_on_click_outside(mut self, c: bool) -> Self {
        self.close_on_click_outside = c;
        self
    }
}

impl RenderOnce for Popover {
    #[track_caller]
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let placement = self.placement;
        let offset = self.offset;
        let close_on_click_outside = self.close_on_click_outside;
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
                            close_on_click_outside,
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
