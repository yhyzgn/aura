use aura_components::{Segmented, SegmentedOption};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| SegmentedDemo {
        basic: cx.new(|_| {
            Segmented::new(vec![
                SegmentedOption::new("Daily", "daily"),
                SegmentedOption::new("Weekly", "weekly"),
                SegmentedOption::new("Monthly", "monthly"),
                SegmentedOption::new("Quarterly", "quarterly"),
                SegmentedOption::new("Yearly", "yearly"),
            ])
            .id("segmented-demo-basic")
            .on_change(|val, _, _| println!("Selected: {}", val))
        }),
        disabled: cx.new(|_| {
            Segmented::new(vec![
                SegmentedOption::new("Map", "Map"),
                SegmentedOption::new("Transit", "Transit"),
                SegmentedOption::new("Satellite", "Satellite").disabled(true),
            ])
            .id("segmented-demo-disabled")
            .value("Map")
            .on_change(|val, _, _| println!("Selected: {}", val))
        }),
        block: cx.new(|_| {
            Segmented::new(vec![
                SegmentedOption::new("123", "123"),
                SegmentedOption::new("456", "456"),
                SegmentedOption::new("long-text-option", "long"),
            ])
            .id("segmented-demo-block")
            .block(true)
            .on_change(|val, _, _| println!("Selected: {}", val))
        }),
    })
    .into()
}

struct SegmentedDemo {
    basic: Entity<Segmented>,
    disabled: Entity<Segmented>,
    block: Entity<Segmented>,
}

impl Render for SegmentedDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

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
                            .child("Segmented 分段控制器"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于展示多个选项并允许用户选择其中单个选项。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(self.basic.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("不可用状态"),
                    )
                    .child(self.disabled.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Block 模式 (撑满宽度)"),
                    )
                    .child(self.block.clone()),
            )
    }
}
