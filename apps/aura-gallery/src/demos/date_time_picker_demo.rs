use aura_components::{DateTimePicker, DateTimeValue, Text};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| DateTimePickerDemo::new(cx)).into()
}

struct DateTimePickerDemo {
    basic: Entity<DateTimePicker>,
    formatted: Entity<DateTimePicker>,
    stepped: Entity<DateTimePicker>,
    no_seconds: Entity<DateTimePicker>,
    range: Entity<DateTimePicker>,
    disabled: Entity<DateTimePicker>,
}

impl DateTimePickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| DateTimePicker::new().width(px(280.0))),
            formatted: cx.new(|_| {
                DateTimePicker::new()
                    .value(DateTimeValue::new(2026, 5, 8, 9, 30, 15).expect("valid datetime"))
                    .format("YYYY年M月D日 HH:mm:ss")
                    .width(px(300.0))
            }),
            stepped: cx.new(|_| {
                DateTimePicker::new()
                    .value(DateTimeValue::new(2026, 5, 8, 14, 30, 0).expect("valid datetime"))
                    .minute_step(15)
                    .second_step(30)
                    .width(px(280.0))
            }),
            no_seconds: cx.new(|_| {
                DateTimePicker::new()
                    .without_seconds()
                    .value(DateTimeValue::new(2026, 5, 8, 18, 45, 0).expect("valid datetime"))
                    .width(px(280.0))
            }),
            range: cx.new(|_| {
                DateTimePicker::new()
                    .date_time_range()
                    .range(
                        DateTimeValue::new(2026, 5, 8, 9, 0, 0).expect("valid datetime"),
                        DateTimeValue::new(2026, 5, 18, 18, 30, 0).expect("valid datetime"),
                    )
                    .width(px(460.0))
            }),
            disabled: cx.new(|_| DateTimePicker::new().disabled(true).width(px(280.0))),
        }
    }
}

impl Render for DateTimePickerDemo {
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
                            .child("DateTimePicker 日期时间选择器"),
                    )
                    .child(div().text_sm().text_color(theme.neutral.text_3).child(
                        "用于选择日期时间和日期时间范围，支持自定义展示格式、时间步进、隐藏秒和确认操作。",
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
            .child(section("日期时间范围", self.range.clone()))
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
