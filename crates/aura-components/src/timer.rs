use aura_core::Config;
use gpui::{App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimerDirection {
    #[default]
    CountUp,
    CountDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimerUnit {
    Milliseconds,
    #[default]
    Seconds,
    Minutes,
    Hours,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerFormat {
    Unit,
    Clock,
}

impl Default for TimerFormat {
    fn default() -> Self {
        Self::Unit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerSnapshot {
    pub elapsed: Duration,
    pub remaining: Option<Duration>,
    pub finished: bool,
}

impl TimerSnapshot {
    pub fn elapsed_as(self, unit: TimerUnit) -> f64 {
        duration_as(self.elapsed, unit)
    }

    pub fn remaining_as(self, unit: TimerUnit) -> Option<f64> {
        self.remaining.map(|remaining| duration_as(remaining, unit))
    }
}

#[derive(Clone)]
pub struct Timer {
    elapsed: Duration,
    duration: Option<Duration>,
    direction: TimerDirection,
    display_unit: TimerUnit,
    format: TimerFormat,
    show_unit: bool,
    title: Option<SharedString>,
    prefix: Option<SharedString>,
    suffix: Option<SharedString>,
    compact: bool,
}

impl Timer {
    pub fn count_up(elapsed: Duration) -> Self {
        Self::new(TimerDirection::CountUp, elapsed, None)
    }

    pub fn count_down(duration: Duration, elapsed: Duration) -> Self {
        Self::new(TimerDirection::CountDown, elapsed, Some(duration))
    }

    pub fn new(direction: TimerDirection, elapsed: Duration, duration: Option<Duration>) -> Self {
        Self {
            elapsed,
            duration,
            direction,
            display_unit: TimerUnit::Seconds,
            format: TimerFormat::Unit,
            show_unit: true,
            title: None,
            prefix: None,
            suffix: None,
            compact: false,
        }
    }

    pub fn elapsed(mut self, elapsed: Duration) -> Self {
        self.elapsed = elapsed;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn direction(mut self, direction: TimerDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn countup(mut self) -> Self {
        self.direction = TimerDirection::CountUp;
        self
    }

    pub fn countdown(mut self) -> Self {
        self.direction = TimerDirection::CountDown;
        self
    }

    pub fn display_unit(mut self, unit: TimerUnit) -> Self {
        self.display_unit = unit;
        self.format = TimerFormat::Unit;
        self
    }

    pub fn format(mut self, format: TimerFormat) -> Self {
        self.format = format;
        self
    }

    pub fn clock_format(mut self) -> Self {
        self.format = TimerFormat::Clock;
        self
    }

    pub fn show_unit(mut self, show: bool) -> Self {
        self.show_unit = show;
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn prefix(mut self, prefix: impl Into<SharedString>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn suffix(mut self, suffix: impl Into<SharedString>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    pub fn snapshot(&self) -> TimerSnapshot {
        let remaining = self
            .duration
            .map(|duration| duration.saturating_sub(self.elapsed));
        TimerSnapshot {
            elapsed: self.elapsed,
            remaining,
            finished: matches!(self.direction, TimerDirection::CountDown)
                && remaining.is_some_and(|remaining| remaining.is_zero()),
        }
    }

    pub fn elapsed_as(&self, unit: TimerUnit) -> f64 {
        self.snapshot().elapsed_as(unit)
    }

    pub fn remaining_as(&self, unit: TimerUnit) -> Option<f64> {
        self.snapshot().remaining_as(unit)
    }

    fn display_duration(&self) -> Duration {
        match self.direction {
            TimerDirection::CountUp => self.elapsed,
            TimerDirection::CountDown => self
                .duration
                .map(|duration| duration.saturating_sub(self.elapsed))
                .unwrap_or_default(),
        }
    }

    fn format_value(&self) -> SharedString {
        match self.format {
            TimerFormat::Unit => {
                format_duration(self.display_duration(), self.display_unit, self.show_unit)
            }
            TimerFormat::Clock => format_clock(self.display_duration()),
        }
    }
}

impl RenderOnce for Timer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let value = self.format_value();
        div()
            .flex()
            .flex_col()
            .gap_1()
            .when(!self.compact, |s| {
                s.p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.card)
            })
            .when_some(self.title, |s, title| {
                s.child(
                    div()
                        .text_xs()
                        .text_color(theme.neutral.text_3)
                        .child(title),
                )
            })
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap_1()
                    .text_color(theme.neutral.text_1)
                    .when_some(self.prefix, |s, prefix| {
                        s.child(
                            div()
                                .text_sm()
                                .text_color(theme.neutral.text_3)
                                .child(prefix),
                        )
                    })
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(value),
                    )
                    .when_some(self.suffix, |s, suffix| {
                        s.child(
                            div()
                                .text_sm()
                                .text_color(theme.neutral.text_3)
                                .child(suffix),
                        )
                    }),
            )
    }
}

impl IntoElement for Timer {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

pub fn duration_as(duration: Duration, unit: TimerUnit) -> f64 {
    match unit {
        TimerUnit::Milliseconds => duration.as_secs_f64() * 1000.0,
        TimerUnit::Seconds => duration.as_secs_f64(),
        TimerUnit::Minutes => duration.as_secs_f64() / 60.0,
        TimerUnit::Hours => duration.as_secs_f64() / 3600.0,
    }
}

pub fn format_duration(duration: Duration, unit: TimerUnit, show_unit: bool) -> SharedString {
    let value = duration_as(duration, unit);
    let text = match unit {
        TimerUnit::Milliseconds => format!("{value:.0}"),
        TimerUnit::Seconds => format!("{value:.1}"),
        TimerUnit::Minutes => format!("{value:.2}"),
        TimerUnit::Hours => format!("{value:.2}"),
    };
    if show_unit {
        format!("{} {}", text, unit_label(unit)).into()
    } else {
        text.into()
    }
}

pub fn format_clock(duration: Duration) -> SharedString {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}").into()
}

fn unit_label(unit: TimerUnit) -> &'static str {
    match unit {
        TimerUnit::Milliseconds => "ms",
        TimerUnit::Seconds => "s",
        TimerUnit::Minutes => "min",
        TimerUnit::Hours => "h",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_snapshot_tracks_countdown_remaining() {
        let timer = Timer::count_down(Duration::from_secs(10), Duration::from_secs(4));
        let snapshot = timer.snapshot();
        assert_eq!(snapshot.elapsed, Duration::from_secs(4));
        assert_eq!(snapshot.remaining, Some(Duration::from_secs(6)));
        assert!(!snapshot.finished);
    }

    #[test]
    fn timer_countdown_saturates_at_zero() {
        let timer = Timer::count_down(Duration::from_secs(10), Duration::from_secs(12));
        let snapshot = timer.snapshot();
        assert_eq!(snapshot.remaining, Some(Duration::ZERO));
        assert!(snapshot.finished);
    }

    #[test]
    fn timer_formats_units() {
        assert_eq!(
            format_duration(Duration::from_millis(1500), TimerUnit::Milliseconds, true),
            SharedString::from("1500 ms")
        );
        assert_eq!(
            format_duration(Duration::from_secs(90), TimerUnit::Minutes, true),
            SharedString::from("1.50 min")
        );
        assert_eq!(
            Timer::count_up(Duration::from_secs(7200)).elapsed_as(TimerUnit::Hours),
            2.0
        );
    }

    #[test]
    fn timer_formats_clock() {
        assert_eq!(
            format_clock(Duration::from_secs(0)),
            SharedString::from("00:00:00")
        );
        assert_eq!(
            format_clock(Duration::from_secs(3661)),
            SharedString::from("01:01:01")
        );
        assert_eq!(
            Timer::count_up(Duration::from_secs(3661))
                .clock_format()
                .format_value(),
            SharedString::from("01:01:01")
        );
    }
}
