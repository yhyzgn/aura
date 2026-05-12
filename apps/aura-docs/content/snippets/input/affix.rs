cx.new(|cx| Input::new("", cx).prepend_text("http://"));
cx.new(|cx| Input::new("", cx).append_text(".com"));
cx.new(|cx| {
    Input::new("", cx)
        .prepend_icon(IconName::User)
        .append_text("Admin")
});
