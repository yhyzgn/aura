cx.new(|cx| {
    Switch::new(false, cx).on_change(|checked, _window, _cx| {
        if checked {
            toast_success!("Switch is on");
        } else {
            toast_info!("Switch is off");
        }
    })
});
