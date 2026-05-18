use aura_components::{HeatBar, HeatBarItem, HeatBarLegend};
use gpui::{IntoElement, rgb};

pub fn heat_bar_events() -> impl IntoElement {
    // 时间轴密集竖条：颜色通常由事件类型或严重等级决定。
    let items = (0..48).map(|index| {
        let value = ((index * 7 + 3) % 11) as f64;
        let color = if value > 7.0 {
            rgb(0xef4444).into()
        } else if value > 3.0 {
            rgb(0xf59e0b).into()
        } else {
            rgb(0x93c5fd).into()
        };
        HeatBarItem::new(format!("t{index}"), value, color)
    });

    HeatBar::new(items)
        .legends([
            HeatBarLegend::new("错误", 3, rgb(0xef4444).into()),
            HeatBarLegend::new("警告", 24, rgb(0xf59e0b).into()),
        ])
        .max_value(10.0)
        .x_labels(["00:00", "06:00", "12:00", "18:00", "24:00"])
}
