//! Semantic progress states.

use aura_components::{Progress, ProgressStatus, Space};
use gpui::IntoElement;

pub fn status_progress() -> impl IntoElement {
    Space::new().vertical().gap_md().children(vec![
        Progress::new(30.0),
        Progress::new(50.0).status(ProgressStatus::Warning),
        Progress::new(70.0).status(ProgressStatus::Exception),
        Progress::new(100.0).status(ProgressStatus::Success),
    ])
}
