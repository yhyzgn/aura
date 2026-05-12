//! Rendering a fenced Markdown block as Aura's native CodeBlock.

use aura_components::CodeBlock;
use gpui::{AnyElement, IntoElement, SharedString};

fn render_code_block(language: Option<SharedString>, code: SharedString) -> AnyElement {
    let mut code_block = CodeBlock::new(code);
    if let Some(language) = language {
        // Language can come from the Markdown fence, e.g. ```rust.
        code_block = code_block.language(language.as_ref());
    }
    code_block.into_any_element()
}

fn main() {
    let _ = render_code_block(Some("rust".into()), "fn main() {}".into());
}
