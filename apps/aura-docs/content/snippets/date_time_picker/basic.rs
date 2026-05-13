//! Basic DateTimePicker with a medium width.

use aura_components::DateTimePicker;

pub fn basic_date_time_picker() -> DateTimePicker {
    DateTimePicker::new()
        .id("docs-date-time-picker-basic")
        .width_md()
}
