//! Preselect a date and display it in a custom format.

use aura_components::{DatePicker, DateValue};

pub fn formatted_date_picker() -> DatePicker {
    DatePicker::new()
        .id("docs-date-picker-formatted")
        .value(DateValue::new(2026, 5, 8).expect("valid date"))
        .format("YYYY年M月D日")
        .width_md()
}
