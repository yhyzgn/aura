//! Minimal Aura application bootstrap.

use aura_core::init_aura;
use aura_theme::Theme;
use gpui::App;

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());
        // Open your first GPUI window here with cx.open_window(...).
    });
}
