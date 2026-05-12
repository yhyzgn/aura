fn render_code_block(language: Option<SharedString>, code: SharedString) -> AnyElement {
    let mut code_block = CodeBlock::new(code);
    if let Some(language) = language {
        code_block = code_block.language(language.as_ref());
    }
    code_block.into_any_element()
}
