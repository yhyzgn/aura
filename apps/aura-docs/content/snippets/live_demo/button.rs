Button::new("Native Button")
    .primary()
    .on_click(|_, _, _| {
        toast_success!("Live demo clicked: {}", "Button");
    });
