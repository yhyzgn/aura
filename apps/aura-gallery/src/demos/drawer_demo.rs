use aura_components::{Drawer, DrawerPlacement, Button, Space};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, px, AnyView};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DrawerDemo).into()
}

struct DrawerDemo;

impl Render for DrawerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Drawer 抽屉"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("屏幕边缘滑出的浮层面板。"))
            )
            .child(
                Space::new().gap(px(16.0))
                    .child(
                        Button::new("Right Drawer").primary()
                            .on_click(|_, _, cx| {
                                Drawer::new().title("Right Drawer").show(cx);
                            })
                    )
                    .child(
                        Button::new("Left Drawer")
                            .on_click(|_, _, cx| {
                                Drawer::new().title("Left Drawer").placement(DrawerPlacement::Left).show(cx);
                            })
                    )
                    .child(
                        Button::new("Top Drawer")
                            .on_click(|_, _, cx| {
                                Drawer::new().title("Top Drawer").placement(DrawerPlacement::Top).height(px(200.0)).show(cx);
                            })
                    )
                    .child(
                        Button::new("Bottom Drawer")
                            .on_click(|_, _, cx| {
                                Drawer::new().title("Bottom Drawer").placement(DrawerPlacement::Bottom).height(px(200.0)).show(cx);
                            })
                    )
            )
    }
}
