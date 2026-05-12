cx.new(|cx| Input::new("", cx).password().placeholder("Password"));
cx.new(|cx| Input::new("secret", cx).password().mask_char('*'));
