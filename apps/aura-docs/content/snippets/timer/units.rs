use aura_components::{Space, Timer, TimerUnit};
use gpui::IntoElement;
use std::time::Duration;

pub fn timer_units() -> impl IntoElement {
    Space::new().gap_lg().children([
        Timer::count_up(Duration::from_millis(1532))
            .title("Latency")
            .display_unit(TimerUnit::Milliseconds)
            .compact()
            .into_any_element(),
        Timer::count_up(Duration::from_secs(64))
            .display_unit(TimerUnit::Seconds)
            .show_unit(false)
            .prefix("T+")
            .suffix("seconds")
            .compact()
            .into_any_element(),
    ])
}
