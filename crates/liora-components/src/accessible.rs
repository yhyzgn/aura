//! Accessibility helpers for Liora components.
//!
//! This module provides lightweight wrappers that attach stable accessibility
//! metadata to existing Liora elements without coupling application code to GPUI
//! internals.

use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Window, div, prelude::*};
use liora_core::stable_unique_id;

/// Wraps any element with a stable accessibility identifier.
///
/// This is useful when a consumer wants to apply GPUI's `accessibility_id`
/// support to a Liora component without changing the component's own API.
pub struct Accessible<E> {
    child: E,
    accessibility_id: SharedString,
    role: Option<gpui::Role>,
    label: Option<SharedString>,
    description: Option<SharedString>,
}

impl<E> Accessible<E> {
    /// Creates a wrapper around `child` with the provided accessibility id.
    pub fn new(accessibility_id: impl Into<SharedString>, child: E) -> Self {
        Self {
            child,
            accessibility_id: accessibility_id.into(),
            role: None,
            label: None,
            description: None,
        }
    }

    /// Overrides the accessibility role.
    pub fn role(mut self, role: gpui::Role) -> Self {
        self.role = Some(role);
        self
    }

    /// Sets the accessible label.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the accessible description.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl<E: IntoElement + 'static> RenderOnce for Accessible<E> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let element_id = stable_unique_id(
            format!("liora-accessible:{}", self.accessibility_id),
            "liora-accessible",
            window,
            cx,
        );

        let mut wrapper = div()
            .id(element_id)
            .accessibility_id(self.accessibility_id)
            .when_some(self.role, |this, role| this.role(role));

        if let Some(label) = self.label {
            wrapper = wrapper.aria_label(label);
        }
        if let Some(description) = self.description {
            wrapper = wrapper.aria_description(description);
        }

        wrapper.child(self.child)
    }
}

impl<E: IntoElement + 'static> IntoElement for Accessible<E> {
    type Element = gpui::ViewElement<Self>;

    fn into_element(self) -> Self::Element {
        gpui::ViewElement::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessible_wrapper_carries_configuration() {
        let wrapped = Accessible::new("settings.save", "Save")
            .label("Save button")
            .description("Persists settings");
        assert_eq!(wrapped.accessibility_id, "settings.save");
        assert!(wrapped.label.is_some());
        assert!(wrapped.description.is_some());
    }
}
