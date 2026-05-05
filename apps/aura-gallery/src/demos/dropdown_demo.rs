use aura_components::{Dropdown, Button, Space};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, px, AnyView};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DropdownDemo).into()
}

struct DropdownDemo;

impl Render for DropdownDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Dropdown 下拉菜单"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("将动作或选项折叠到下拉菜单中。"))
            )
            .child(
                Space::new().gap(px(16.0))
                    .child(
                        Dropdown::new(Button::new("Hover me"))
                            .item("Action 1", |_, _| println!("Action 1"))
                            .item("Action 2", |_, _| println!("Action 2"))
                            .item("Action 3", |_, _| println!("Action 3"))
                    )
            )
    }
}
