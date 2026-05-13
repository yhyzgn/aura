//! Compact ColorPicker trigger without text label.

use aura_components::ColorPicker;

pub fn compact_color_picker() -> ColorPicker {
    ColorPicker::new("#F56C6C")
        .id("docs-color-picker-compact")
        .show_label(false)
}
