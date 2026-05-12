//! Minimal Markdown-to-native rendering entry point used by Aura Docs.
//!
//! This snippet intentionally keeps parser internals stubbed so the public
//! contract is clear: Markdown becomes a GPUI-native element, not web markup.

use gpui::{AnyElement, Component, IntoElement, RenderOnce, prelude::*};

struct MarkdownDocument;

impl MarkdownDocument {
    fn parse(_md_text: &str) -> Self {
        // In Aura Docs the real parser builds an AST with pulldown-cmark.
        Self
    }
}

impl RenderOnce for MarkdownDocument {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        gpui::div().child("Native markdown document")
    }
}

pub fn render_markdown(md_text: &str) -> AnyElement {
    Component::new(MarkdownDocument::parse(md_text)).into_any_element()
}

fn main() {}
