use aura_core::init_aura;
use aura_theme::Theme;

fn main() {
    gpui_platform::application().run(|cx| {
        init_aura(cx, Theme::light());
        // open_window(...)
    });
}
