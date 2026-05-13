//! Full-screen loading overlay configuration.

use aura_components::Loading;

pub fn fullscreen_loading() -> Loading {
    Loading::new().text("Preparing workspace...").full_screen()
}
