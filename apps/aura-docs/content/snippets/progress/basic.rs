//! Basic line progress examples.

use aura_components::{Progress, ProgressStatus, Space};
use gpui::IntoElement;

pub fn basic_progress() -> impl IntoElement {
    Space::new().vertical().gap_md().children(vec![
        Progress::new(0.0),
        Progress::new(30.0),
        Progress::new(50.0),
        Progress::new(100.0).status(ProgressStatus::Success),
    ])
}
