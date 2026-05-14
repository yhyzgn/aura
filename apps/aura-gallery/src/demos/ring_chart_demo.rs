use aura_components::layout_helpers::{page, section};
use aura_components::{
    ChartPoint, ChartSeries, ChartValueLabelContent, ChartValueLabelPlacement, RingChart, Space,
};
use gpui::{AnyView, App, Context, Render, Window, blue, green, prelude::*, px, red, yellow};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| RingChartDemo).into()
}

struct RingChartDemo;

impl Render for RingChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "RingChart 圆环图",
            "使用中心留空的扇形 path 展示圆环占比。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础圆环",
                    "适合 KPI 占比展示。",
                    RingChart::new(slices())
                        .id("ring-chart-demo-basic")
                        .height(px(420.0))
                        .percentage_decimals(1)
                        .value_label_placement(ChartValueLabelPlacement::OutsideAligned),
                ))
                .child(section(
                    "两侧对齐外部标注",
                    "所有标签统一放在图形两侧并端点对齐，颜色由每个扇区独立定义。",
                    RingChart::new(colored_slices())
                        .id("ring-chart-demo-aligned-labels")
                        .height(px(420.0))
                        .inner_ratio(0.48)
                        .value_label_content(ChartValueLabelContent::ValueOverTotalAndPercentage)
                        .value_label_placement(ChartValueLabelPlacement::OutsideAligned)
                        .percentage_decimals(1)
                        .outside_label_threshold_degrees(120),
                ))
                .child(section(
                    "更厚圆环",
                    "增强中心空间感。",
                    RingChart::new(slices())
                        .id("ring-chart-demo-thick")
                        .height(px(420.0))
                        .inner_ratio(0.44)
                        .percentage_decimals(2)
                        .outside_label_threshold_degrees(34)
                        .value_label_placement(ChartValueLabelPlacement::OutsideAligned),
                )),
        )
    }
}

fn colored_slices() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("Desktop", [ChartPoint::new("Desktop", 62.0)]).fill_color(blue()),
        ChartSeries::new("Mobile", [ChartPoint::new("Mobile", 24.0)]).fill_color(green()),
        ChartSeries::new("Tablet", [ChartPoint::new("Tablet", 9.0)]).fill_color(yellow()),
        ChartSeries::new("Other", [ChartPoint::new("Other", 5.0)]).fill_color(red()),
    ]
}

fn slices() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("Desktop", [ChartPoint::new("Desktop", 62.0)]),
        ChartSeries::new("Mobile", [ChartPoint::new("Mobile", 24.0)]),
        ChartSeries::new("Tablet", [ChartPoint::new("Tablet", 9.0)]),
        ChartSeries::new("Other", [ChartPoint::new("Other", 5.0)]),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn ring_chart_demo_uses_component_api() {
        let source = include_str!("ring_chart_demo.rs");
        assert!(source.contains("RingChart::new"));
        assert!(source.contains("OutsideAligned"));
        assert!(source.contains("fill_color"));
    }
}
