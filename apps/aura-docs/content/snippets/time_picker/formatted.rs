//! TimePicker with custom display format.

use aura_components::{TimePicker, TimeValue};

pub fn formatted_time_picker() -> TimePicker {
    TimePicker::new()
        .id("docs-time-picker-formatted")
        .value(TimeValue::new(9, 30, 15).expect("valid time"))
        .format("HH时mm分ss秒")
        .width_md()
}
