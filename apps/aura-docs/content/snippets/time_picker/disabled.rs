//! Disabled TimePicker.

use aura_components::TimePicker;

pub fn disabled_time_picker() -> TimePicker {
    TimePicker::new()
        .id("docs-time-picker-disabled")
        .disabled(true)
        .width_md()
}
