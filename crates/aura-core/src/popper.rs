use gpui::{AnyElement, Global, Bounds, Pixels, Point, App, Window};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Top, TopStart, TopEnd,
    Bottom, BottomStart, BottomEnd,
    Left, LeftStart, LeftEnd,
    Right, RightStart, RightEnd,
}

impl Placement {
    pub fn flip(&self) -> Self {
        match self {
            Placement::Top => Placement::Bottom,
            Placement::TopStart => Placement::BottomStart,
            Placement::TopEnd => Placement::BottomEnd,
            Placement::Bottom => Placement::Top,
            Placement::BottomStart => Placement::TopStart,
            Placement::BottomEnd => Placement::TopEnd,
            Placement::Left => Placement::Right,
            Placement::LeftStart => Placement::RightStart,
            Placement::LeftEnd => Placement::RightEnd,
            Placement::Right => Placement::Left,
            Placement::RightStart => Placement::LeftStart,
            Placement::RightEnd => Placement::LeftEnd,
        }
    }
}

pub type PortalRender = Box<dyn Fn(&mut Window, &mut App) -> AnyElement>;

pub struct Portal(pub Vec<PortalRender>);
impl Global for Portal {}

pub fn push_portal(render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static, cx: &mut App) {
    if !cx.has_global::<Portal>() {
        cx.set_global(Portal(vec![]));
    }
    cx.global_mut::<Portal>().0.push(Box::new(render));
}

pub fn clear_portals(cx: &mut App) {
    if cx.has_global::<Portal>() {
        cx.global_mut::<Portal>().0.clear();
    }
}

pub struct ZIndexStack {
    pub base: u32,
    pub popup: u32,
    pub modal: u32,
    pub notification: u32,
    pub tooltip: u32,
}

impl Default for ZIndexStack {
    fn default() -> Self {
        Self {
            base: 1000,
            popup: 1100,
            modal: 1200,
            notification: 1300,
            tooltip: 1400,
        }
    }
}

impl Global for ZIndexStack {}

pub struct Popper {
    pub anchor_bounds: Bounds<Pixels>,
    pub placement: Placement,
    pub offset: Pixels,
}

impl Popper {
    pub fn calculate_position(&self, content_size: gpui::Size<Pixels>) -> Point<Pixels> {
        self.calculate_position_with_placement(self.placement, content_size)
    }

    fn calculate_position_with_placement(&self, placement: Placement, content_size: gpui::Size<Pixels>) -> Point<Pixels> {
        let anchor = self.anchor_bounds;
        let (x, y) = match placement {
            Placement::Top => (
                anchor.left() + (anchor.size.width - content_size.width) / 2.0,
                anchor.top() - content_size.height - self.offset,
            ),
            Placement::TopStart => (
                anchor.left(),
                anchor.top() - content_size.height - self.offset,
            ),
            Placement::TopEnd => (
                anchor.right() - content_size.width,
                anchor.top() - content_size.height - self.offset,
            ),
            Placement::Bottom => (
                anchor.left() + (anchor.size.width - content_size.width) / 2.0,
                anchor.bottom() + self.offset,
            ),
            Placement::BottomStart => (
                anchor.left(),
                anchor.bottom() + self.offset,
            ),
            Placement::BottomEnd => (
                anchor.right() - content_size.width,
                anchor.bottom() + self.offset,
            ),
            Placement::Left => (
                anchor.left() - content_size.width - self.offset,
                anchor.top() + (anchor.size.height - content_size.height) / 2.0,
            ),
            Placement::LeftStart => (
                anchor.left() - content_size.width - self.offset,
                anchor.top(),
            ),
            Placement::LeftEnd => (
                anchor.left() - content_size.width - self.offset,
                anchor.bottom() - content_size.height,
            ),
            Placement::Right => (
                anchor.right() + self.offset,
                anchor.top() + (anchor.size.height - content_size.height) / 2.0,
            ),
            Placement::RightStart => (
                anchor.right() + self.offset,
                anchor.top(),
            ),
            Placement::RightEnd => (
                anchor.right() + self.offset,
                anchor.bottom() - content_size.height,
            ),
        };

        Point { x, y }
    }

    pub fn calculate_position_with_flip(
        &self,
        content_size: gpui::Size<Pixels>,
        viewport: Bounds<Pixels>,
    ) -> (Point<Pixels>, Placement) {
        let pos = self.calculate_position_with_placement(self.placement, content_size);
        let mut final_pos = pos;
        let mut final_placement = self.placement;

        let out_of_bounds = pos.x < viewport.left()
            || pos.x + content_size.width > viewport.right()
            || pos.y < viewport.top()
            || pos.y + content_size.height > viewport.bottom();

        if out_of_bounds {
            let flipped_placement = self.placement.flip();
            let flipped_pos = self.calculate_position_with_placement(flipped_placement, content_size);

            let flipped_out_of_bounds = flipped_pos.x < viewport.left()
                || flipped_pos.x + content_size.width > viewport.right()
                || flipped_pos.y < viewport.top()
                || flipped_pos.y + content_size.height > viewport.bottom();

            // If flipped is better (stays within bounds or at least doesn't overflow as much), use it.
            if !flipped_out_of_bounds {
                final_pos = flipped_pos;
                final_placement = flipped_placement;
            }
        }

        // Clamp to viewport as a final fallback
        final_pos.x = final_pos.x.clamp(viewport.left(), viewport.right() - content_size.width);
        final_pos.y = final_pos.y.clamp(viewport.top(), viewport.bottom() - content_size.height);

        (final_pos, final_placement)
    }
}
