pub mod button_demo;

use gpui::{AnyElement, Window, Context};
use crate::category::Category;
use crate::Gallery;

pub struct DemoEntry {
    pub name: &'static str,
    pub category: Category,
    pub description: &'static str,
    pub render: fn(&mut Window, &mut Context<Gallery>) -> AnyElement,
}

pub fn registry() -> Vec<DemoEntry> {
    vec![
        DemoEntry {
            name: "Button 按钮",
            category: Category::Basic,
            description: "常用的操作按钮",
            render: button_demo::render,
        },
    ]
}
