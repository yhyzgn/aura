match event {
    Event::Start(tag) => state.start_tag(tag),
    Event::End(tag) => state.end_tag(tag),
    Event::Text(text) => state.push_text_with_live_demos(text.as_ref(), style),
    Event::Code(text) => state.push_inline_code(text.as_ref()),
    Event::Rule => state.push_block(Block::Rule),
    _ => {}
}
