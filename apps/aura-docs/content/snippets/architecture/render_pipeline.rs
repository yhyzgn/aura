pub fn render_markdown(md_text: &str) -> gpui::AnyElement {
    Component::new(MarkdownDocument::parse(md_text)).into_any_element()
}
