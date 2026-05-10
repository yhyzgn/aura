use gpui::{
    AnyElement, App, Component, DefiniteLength, IntoElement, RenderOnce, Window, prelude::*, px,
};

pub struct Space {
    children: Vec<AnyElement>,
    vertical: bool,
    wrap: bool,
    gap: Option<DefiniteLength>,
}

impl Space {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            vertical: false,
            wrap: false,
            gap: None,
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn gap(mut self, gap: impl Into<DefiniteLength>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    pub fn gap_xs(self) -> Self {
        self.gap(px(4.0))
    }

    pub fn gap_sm(self) -> Self {
        self.gap(px(8.0))
    }

    pub fn gap_md(self) -> Self {
        self.gap(px(12.0))
    }

    pub fn gap_lg(self) -> Self {
        self.gap(px(16.0))
    }

    pub fn gap_xl(self) -> Self {
        self.gap(px(24.0))
    }

    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl RenderOnce for Space {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut div = gpui::div().flex();
        if self.vertical {
            div = div.flex_col();
        } else {
            div = div.flex_row().items_center();
        }

        if self.wrap {
            div = div.flex_wrap();
        }

        if let Some(gap) = self.gap {
            div = div.gap(gap);
        } else {
            div = div.gap_2(); // Default gap
        }

        div.children(self.children)
    }
}

impl IntoElement for Space {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_wrap_builder_tracks_state() {
        let space = Space::new().wrap();

        assert!(space.wrap);
    }
}
