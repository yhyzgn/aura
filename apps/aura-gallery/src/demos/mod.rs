pub mod button_demo;
pub mod icon_demo;

use gpui::AnyElement;

pub struct DemoEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub render: fn() -> AnyElement,
}

pub fn registry() -> Vec<DemoEntry> {
    vec![
        DemoEntry {
            name: "Button 按钮",
            description: "常用的操作按钮",
            render: button_demo::render,
        },
        DemoEntry {
            name: "Icon 图标",
            description: "基于 Lucide 的图标系统",
            render: icon_demo::render,
        },
    ]
}
