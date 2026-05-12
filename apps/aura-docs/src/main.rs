mod markdown;

use aura_components::CodeBlock;
use aura_core::init_aura;
use aura_theme::Theme;
use gpui::{App, Bounds, WindowBounds, WindowOptions, px, size};

fn run_docs() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());
        CodeBlock::register_key_bindings(cx);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(Bounds {
                    origin: gpui::Point::default(),
                    size: size(px(1680.0), px(1080.0)),
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Aura Docs — Native Main Window".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| markdown::render_docs_shell(cx),
        );
    });
}

fn main() {
    run_docs();
}
