use aura_components::{Text, TimePicker, TimeValue};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TimePickerDemo::new(cx)).into()
}

struct TimePickerDemo {
    basic: Entity<TimePicker>,
    formatted: Entity<TimePicker>,
    stepped: Entity<TimePicker>,
    no_seconds: Entity<TimePicker>,
    disabled: Entity<TimePicker>,
}

impl TimePickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| TimePicker::new().width(px(240.0))),
            formatted: cx.new(|_| {
                TimePicker::new()
                    .value(TimeValue::new(9, 30, 15).expect("valid time"))
                    .format("HH时mm分ss秒")
                    .width(px(240.0))
            }),
            stepped: cx.new(|_| {
                TimePicker::new()
                    .value(TimeValue::new(14, 30, 0).expect("valid time"))
                    .minute_step(15)
                    .second_step(30)
                    .width(px(240.0))
            }),
            no_seconds: cx.new(|_| {
                TimePicker::new()
                    .without_seconds()
                    .value(TimeValue::new(18, 45, 0).expect("valid time"))
                    .width(px(240.0))
            }),
            disabled: cx.new(|_| TimePicker::new().disabled(true).width(px(240.0))),
        }
    }
}

impl Render for TimePickerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let selected_text = self
            .basic
            .read(cx)
            .value_ref()
            .map(|value| value.format())
            .unwrap_or_else(|| "尚未选择".to_string());

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
                            .child("TimePicker 时间选择器"),
                    )
                    .child(div().text_sm().text_color(theme.neutral.text_3).child(
                        "用于选择固定步进时间，支持自定义展示格式、隐藏秒、禁用状态和变更回调。",
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
                        Text::new(format!("当前选择：{}", selected_text))
                            .size(px(theme.font_size.sm)),
                    ),
            ))
            .child(section("自定义展示格式", self.formatted.clone()))
            .child(section("固定步进", self.stepped.clone()))
            .child(section("隐藏秒", self.no_seconds.clone()))
            .child(section("禁用状态", self.disabled.clone()))
    }
}

fn section(title: &'static str, content: impl IntoElement) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(content)
}
