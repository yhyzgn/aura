use aura_components::{BarChart, ChartPoint, ChartSeries};
use gpui::{IntoElement, px};

pub fn bar_chart_basic() -> impl IntoElement {
    BarChart::new([ChartSeries::new(
        "Revenue",
        [
            ChartPoint::new("Q1", 42.0),
            ChartPoint::new("Q2", 58.0),
            ChartPoint::new("Q3", 73.0),
            ChartPoint::new("Q4", 96.0),
        ],
    )])
    .height(px(260.0))
}
