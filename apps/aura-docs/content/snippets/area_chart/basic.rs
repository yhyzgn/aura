use aura_components::{AreaChart, ChartPoint, ChartSeries};
use gpui::{IntoElement, px};

pub fn area_chart_basic() -> impl IntoElement {
    AreaChart::new([ChartSeries::new(
        "Visitors",
        [
            ChartPoint::new("Mon", 24.0),
            ChartPoint::new("Tue", 32.0),
            ChartPoint::new("Wed", 45.0),
            ChartPoint::new("Thu", 52.0),
            ChartPoint::new("Fri", 61.0),
            ChartPoint::new("Sat", 72.0),
            ChartPoint::new("Sun", 68.0),
        ],
    )])
    .height(px(260.0))
}
