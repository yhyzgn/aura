use crate::chart::{
    ChartOptions, ChartPalette, ChartSeries, collect_labels, has_chart_data, normalized_domain,
};
use crate::chart_frame::{format_chart_value, paint_chart_frame, paint_chart_label_aligned};
use crate::chart_scale::{ScaleLinear, ScalePoint};
use crate::chart_shape::{area_path, line_path, smooth_area_path, smooth_line_path};
use crate::{Empty, Space, Text};
use aura_core::{Config, unique_id};
use gpui::{
    App, Background, Component, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    Pixels, RenderOnce, SharedString, Styled, Window, canvas, div, fill, point, px, size,
};

#[derive(Clone)]
pub struct LineChart {
    series: Vec<ChartSeries>,
    options: ChartOptions,
    point_markers: bool,
    smooth: bool,
    area_fill: bool,
}

impl LineChart {
    pub fn new(series: impl IntoIterator<Item = ChartSeries>) -> Self {
        Self {
            series: series.into_iter().collect(),
            options: ChartOptions {
                id: unique_id("line-chart"),
                ..ChartOptions::default()
            },
            point_markers: true,
            smooth: true,
            area_fill: true,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.options.id = id.into();
        self
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.options.height = height;
        self
    }

    pub fn show_grid(mut self, show: bool) -> Self {
        self.options.show_grid = show;
        self
    }

    pub fn show_axis(mut self, show: bool) -> Self {
        self.options.show_axis = show;
        self
    }

    pub fn show_legend(mut self, show: bool) -> Self {
        self.options.show_legend = show;
        self
    }

    pub fn y_domain(mut self, min: f64, max: f64) -> Self {
        self.options.y_domain = Some((min, max));
        self
    }

    pub fn y_format(mut self, formatter: fn(f64) -> SharedString) -> Self {
        self.options.y_format = Some(formatter);
        self
    }

    pub fn point_markers(mut self, enabled: bool) -> Self {
        self.point_markers = enabled;
        self
    }

    pub fn smooth(mut self, enabled: bool) -> Self {
        self.smooth = enabled;
        self
    }

    pub fn area_fill(mut self, enabled: bool) -> Self {
        self.area_fill = enabled;
        self
    }

    pub fn show_value_labels(mut self, show: bool) -> Self {
        self.options.show_value_labels = show;
        self
    }

    pub fn series(&self) -> &[ChartSeries] {
        &self.series
    }

    pub fn options(&self) -> &ChartOptions {
        &self.options
    }
}

impl IntoElement for LineChart {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for LineChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let palette = ChartPalette::from_config(cx.global::<Config>());
        let has_data = has_chart_data(&self.series);
        let height = self.options.height;
        let id = self.options.id.clone();

        let mut shell = div()
            .id(ElementId::from(id.clone()))
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .p_3()
            .rounded_md()
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card);

        if !has_data {
            return shell
                .h(height)
                .items_center()
                .justify_center()
                .child(Empty::new().description("暂无图表数据"))
                .into_any_element();
        }

        if self.options.show_legend {
            shell = shell.child(render_legend(&self.series, &palette));
        }

        shell
            .child(render_line_canvas(
                self.series,
                self.options,
                palette,
                self.point_markers,
                self.smooth,
                self.area_fill,
            ))
            .into_any_element()
    }
}

fn gradient_for_series(color: Hsla) -> gpui::Background {
    // GPUI uses CSS-like linear gradient angles. 180deg keeps the strongest
    // color on the curve edge and fades vertically toward the chart baseline.
    gpui::linear_gradient(
        180.0,
        gpui::linear_color_stop(color.opacity(0.28), 0.0),
        gpui::linear_color_stop(color.opacity(0.0), 1.0),
    )
}

fn render_legend(series: &[ChartSeries], palette: &ChartPalette) -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .children(series.iter().enumerate().map(|(index, series)| {
            let color = series.color.unwrap_or_else(|| palette.series_color(index));
            Space::new()
                .gap_xs()
                .align_center()
                .child(div().w(px(10.0)).h(px(10.0)).rounded_full().bg(color))
                .child(Text::new(series.name.clone()).size(px(12.0)))
        }))
}

fn render_line_canvas(
    series: Vec<ChartSeries>,
    options: ChartOptions,
    palette: ChartPalette,
    point_markers: bool,
    smooth: bool,
    area_fill: bool,
) -> impl IntoElement {
    let height = options.height;
    canvas(
        |_, _, _| (),
        move |bounds, _, window, cx| {
            let labels = collect_labels(&series);
            if labels.is_empty() {
                return;
            }

            let padding = options.padding;
            let left = bounds.left() + padding.left;
            let right = bounds.right() - padding.right;
            let top = bounds.top() + padding.top;
            let bottom = bounds.bottom() - padding.bottom;
            let width = (right - left).max(px(1.0));
            let plot_height = (bottom - top).max(px(1.0));

            let x = ScalePoint::new(labels.clone(), (0.0, width.as_f32()));
            let domain = normalized_domain(options.y_domain, &series);
            let y = ScaleLinear::new(domain, (plot_height.as_f32(), 0.0));
            if options.show_grid || options.show_axis {
                paint_chart_frame(
                    left,
                    top,
                    width,
                    plot_height,
                    &labels,
                    &x,
                    &y,
                    &palette,
                    &options,
                    window,
                    cx,
                );
            }

            for (series_index, current) in series.iter().enumerate() {
                let color = current
                    .color
                    .unwrap_or_else(|| palette.series_color(series_index));
                let point_data = current
                    .points
                    .iter()
                    .enumerate()
                    .filter(|(_, chart_point)| chart_point.is_finite())
                    .filter_map(|(index, chart_point)| {
                        let x_pos = x.tick_index(index)?;
                        let position = point(
                            left + px(x_pos),
                            top + px(y.tick(chart_point.value).clamp(0.0, plot_height.as_f32())),
                        );
                        Some((position, chart_point.value))
                    })
                    .collect::<Vec<_>>();
                let points = point_data
                    .iter()
                    .map(|(position, _)| *position)
                    .collect::<Vec<_>>();
                if area_fill {
                    let baseline_y = top + px(plot_height.as_f32());
                    let area = if smooth {
                        smooth_area_path(&points, baseline_y)
                    } else {
                        area_path(&points, baseline_y)
                    };
                    if let Some(path) = area {
                        let gradient = gradient_for_series(color);
                        window.paint_path(path, gradient);
                    }
                }
                if let Some(path) = if smooth {
                    smooth_line_path(&points, px(2.4))
                } else {
                    line_path(&points, px(2.0))
                } {
                    window.paint_path(path, color);
                }
                if point_markers {
                    for (point_pos, _) in &point_data {
                        window.paint_quad(fill(
                            gpui::Bounds::new(
                                point(point_pos.x - px(3.0), point_pos.y - px(3.0)),
                                size(px(6.0), px(6.0)),
                            ),
                            Background::from(color),
                        ));
                    }
                }
                if options.show_value_labels {
                    for (point_pos, value) in &point_data {
                        paint_chart_label_aligned(
                            format_chart_value(*value, options.y_format),
                            point(point_pos.x - px(18.0), point_pos.y - px(20.0)),
                            palette.label,
                            gpui::TextAlign::Center,
                            Some(px(36.0)),
                            window,
                            cx,
                        );
                    }
                }
            }
        },
    )
    .w_full()
    .h(height)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_series() -> Vec<ChartSeries> {
        vec![ChartSeries::new(
            "CPU",
            [
                ChartPoint::new("10:00", 20.0),
                ChartPoint::new("10:05", 35.0),
                ChartPoint::new("10:10", 28.0),
            ],
        )]
    }

    use crate::chart::ChartPoint;

    #[test]
    fn line_chart_builder_tracks_options() {
        let chart = LineChart::new(sample_series())
            .id("cpu-line")
            .height(px(320.0))
            .show_grid(false)
            .show_axis(false)
            .show_legend(false)
            .y_domain(0.0, 100.0)
            .point_markers(false)
            .show_value_labels(false);

        assert_eq!(chart.options().id, SharedString::from("cpu-line"));
        assert_eq!(chart.options().height, px(320.0));
        assert!(!chart.options().show_grid);
        assert!(!chart.options().show_axis);
        assert!(!chart.options().show_legend);
        assert_eq!(chart.options().y_domain, Some((0.0, 100.0)));
        assert!(!chart.point_markers);
        assert!(!chart.options().show_value_labels);
    }

    #[test]
    fn line_chart_keeps_series_data() {
        let chart = LineChart::new(sample_series());
        assert_eq!(chart.series().len(), 1);
        assert_eq!(chart.series()[0].name, SharedString::from("CPU"));
        assert_eq!(chart.series()[0].points.len(), 3);
    }
}
