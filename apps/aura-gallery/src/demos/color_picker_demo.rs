use aura_components::{Card, ColorPicker, Text};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| ColorPickerDemo::new(cx)).into()
}

struct ColorPickerDemo {
    basic: Entity<ColorPicker>,
    custom: Entity<ColorPicker>,
    compact: Entity<ColorPicker>,
    disabled: Entity<ColorPicker>,
}

impl ColorPickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| ColorPicker::new("#409eff").width(px(360.0))),
            custom: cx.new(|_| {
                ColorPicker::new("#13c2c2").width(px(360.0)).presets([
                    "#13C2C2", "#52C41A", "#FAAD14", "#F5222D", "#722ED1", "#EB2F96",
                ])
            }),
            compact: cx.new(|_| ColorPicker::new("#F56C6C").show_label(false)),
            disabled: cx.new(|_| ColorPicker::new("#909399").disabled(true).width(px(360.0))),
        }
    }
}

impl Render for ColorPickerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("ColorPicker 颜色选择器"),
                    )
                    .child(div().text_sm().text_color(theme.neutral.text_3).child(
                        "点击颜色方块弹出类似取色器的面板，支持自由选择色相、明度/饱和度和 alpha 透明度。",
                    )),
            )
            .child(section(
                "基础用法",
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(self.basic.clone())
                    .child(
                        Text::new(
                            "点击颜色方块打开 popup；在大色板中选择颜色，右侧切换 hue，下方选择 alpha。支持 #RGB、#RRGGBB 和 rgba 展示。",
                        )
                        .size(px(theme.font_size.sm)),
                    ),
            ))
            .child(section("自定义 Popup 预设色", self.custom.clone()))
            .child(section("隐藏文本标签", self.compact.clone()))
            .child(section("禁用状态", self.disabled.clone()))
    }
}

fn section(title: &'static str, content: impl IntoElement) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(Card::new(content))
}
