//! Toast macros support standard format! arguments.

use aura_components::{toast_info, toast_success};

fn formatted_toasts() {
    let name = "Aura";
    let count = 4;
    toast_info!("{}, you have {} toast variants.", name, count);

    let component = "Message";
    let api = "toast_success!";
    toast_success!("{component} macro {api} works.");
}

fn main() {
    // In an app, call MessageManager::init(cx) before these macros run.
    let _ = formatted_toasts as fn();
}
