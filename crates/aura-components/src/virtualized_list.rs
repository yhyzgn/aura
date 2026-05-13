use gpui::{
    AnyElement, App, Context, Entity, IntoElement, ListAlignment, ListState, Pixels, Render,
    Window, div, list, prelude::*, px,
};
use std::sync::Arc;

type RenderItem = dyn Fn(usize, &mut Window, &mut App) -> AnyElement + 'static;

/// A native virtualized vertical list for large or expensive item trees.
///
/// The component owns GPUI's [`ListState`] and renders only the visible item
/// range plus a configurable overdraw area. Pair it with [`crate::VirtualScrollbar`]
/// when a custom Aura scrollbar is needed.
pub struct VirtualizedList {
    item_count: usize,
    list_state: ListState,
    render_item: Arc<RenderItem>,
    overdraw: Pixels,
    item_spacing: Pixels,
    height: Option<Pixels>,
}

impl VirtualizedList {
    pub fn new(
        item_count: usize,
        _cx: &mut Context<Self>,
        render_item: impl Fn(usize, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        let overdraw = px(640.0);
        Self {
            item_count,
            list_state: ListState::new(item_count, ListAlignment::Top, overdraw),
            render_item: Arc::new(render_item),
            overdraw,
            item_spacing: px(0.0),
            height: None,
        }
    }

    pub fn entity(
        item_count: usize,
        cx: &mut App,
        render_item: impl Fn(usize, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(item_count, cx, render_item))
    }

    pub fn list_state(&self) -> ListState {
        self.list_state.clone()
    }

    pub fn set_item_count(&mut self, item_count: usize) {
        if self.item_count == item_count {
            return;
        }
        self.item_count = item_count;
        self.list_state = ListState::new(item_count, ListAlignment::Top, self.overdraw);
    }

    pub fn set_render_item(
        &mut self,
        render_item: impl Fn(usize, &mut Window, &mut App) -> AnyElement + 'static,
    ) {
        self.render_item = Arc::new(render_item);
    }

    pub fn set_item_spacing(&mut self, spacing: impl Into<Pixels>) {
        let spacing = spacing.into();
        if self.item_spacing == spacing {
            return;
        }
        self.item_spacing = spacing;
        self.list_state.remeasure();
    }

    pub fn set_overdraw(&mut self, overdraw: impl Into<Pixels>) {
        let overdraw = overdraw.into();
        if self.overdraw == overdraw {
            return;
        }
        self.overdraw = overdraw;
        self.list_state = ListState::new(self.item_count, ListAlignment::Top, overdraw);
    }

    pub fn set_height(&mut self, height: Option<Pixels>) {
        if self.height == height {
            return;
        }
        self.height = height;
        self.list_state.remeasure();
    }

    /// Mark every item for remeasurement while preserving proportional scroll.
    ///
    /// Updating the render closure alone does not remeasure automatically, so
    /// callers that know item heights changed can opt into the heavier work.
    pub fn remeasure(&self) {
        self.list_state.remeasure();
    }

    /// Mark one item range for remeasurement while preserving proportional scroll.
    pub fn remeasure_items(&self, range: std::ops::Range<usize>) {
        self.list_state.remeasure_items(range);
    }
}

impl Render for VirtualizedList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let render_item = self.render_item.clone();
        let spacing = self.item_spacing;

        div()
            .relative()
            .size_full()
            .when_some(self.height, |el, height| el.h(height))
            .child(
                list(self.list_state.clone(), move |index, window, cx| {
                    let item = (render_item)(index, window, cx);
                    if spacing > px(0.0) {
                        div().pb(spacing).child(item).into_any_element()
                    } else {
                        item
                    }
                })
                .size_full(),
            )
            .child(crate::VirtualScrollbar::new(self.list_state.clone()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn virtualized_list_owns_list_state_and_uses_aura_scrollbar() {
        let source = include_str!("virtualized_list.rs");

        assert!(source.contains("pub struct VirtualizedList"));
        assert!(source.contains("ListState::new"));
        assert!(source.contains("list(self.list_state.clone()"));
        assert!(source.contains("VirtualScrollbar::new"));
        assert!(source.contains("set_item_spacing"));
        assert!(source.contains("set_render_item"));
    }

    #[test]
    fn virtualized_list_resets_state_when_count_or_overdraw_changes() {
        let source = include_str!("virtualized_list.rs");

        assert!(source.contains("set_item_count"));
        assert!(source.contains("set_overdraw"));
        assert!(source.contains("self.list_state = ListState::new"));
        assert!(source.contains("pub fn remeasure(&self)"));
        assert!(source.contains("pub fn remeasure_items"));
    }
}
