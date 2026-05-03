mod category;
mod demos;

use gpui::{
    div, prelude::*, px, App, Bounds, Context, Render, Window,
    WindowBounds, WindowOptions, size, AnyElement,
};
use gpui_platform::application;
use aura_core::{init_aura, AuraContextExt};
use aura_theme::AuraTheme;
use category::Category;

pub struct Gallery;

fn run_gallery() {
    application().run(|cx: &mut App| {
        init_aura(cx, AuraTheme::light());
        let bounds = Bounds::centered(None, size(px(960.0), px(720.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Gallery),
        )
        .unwrap();
        cx.activate(true);
    });
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let registry = demos::registry();

        let header = div()
            .flex().flex_col().gap_1().mb_4().pb_4().border_b_1()
            .border_color(cx.aura().neutral.border)
            .child(div().text_2xl().text_color(cx.aura().neutral.text_1).font_weight(gpui::FontWeight::BOLD).child("Aura UI"))
            .child(div().text_size(px(cx.aura().font_size.md)).text_color(cx.aura().neutral.text_3).child(format!("Native Component Library · {} components", registry.len())));

        let grouped: Vec<(Category, Vec<&demos::DemoEntry>)> = Category::ALL
            .iter()
            .filter_map(|&cat| {
                let es: Vec<_> = registry.iter().filter(|e| e.category == cat).collect();
                if es.is_empty() { None } else { Some((cat, es)) }
            })
            .collect();

        let sections: Vec<AnyElement> = grouped.iter().map(|(cat, entries)| {
            category_section(cx, *cat, entries, _window)
        }).collect();

        let mut body = div().flex().flex_col().size_full().bg(cx.aura().neutral.body).gap_4().p_8().child(header);
        for s in sections { body = body.child(s); }
        body
    }
}

fn category_section(
    cx: &mut Context<Gallery>,
    category: Category,
    entries: &[&demos::DemoEntry],
    window: &mut Window,
) -> AnyElement {
    let name = category.name();
    let icon = category.icon();
    let count = entries.len();
    let border = cx.aura().neutral.border;
    let primary = cx.aura().primary.base;
    let font_sm = cx.aura().font_size.sm;
    let text3 = cx.aura().neutral.text_3;

    let mut section = div()
        .flex().flex_col().gap_2().mb_8()
        .child(
            div().flex().items_center().gap_2().mb_4().pb_2().border_b_1()
                .border_color(border)
                .child(div().text_xl().text_color(primary).child(format!("{} {}", icon, name)))
                .child(div().text_size(px(font_sm)).text_color(text3).child(format!("{} components", count))),
        );

    for entry in entries {
        let name = entry.name;
        let desc = entry.description;
        let render_fn = entry.render;
        // Extract theme tokens first, then release borrow
        let theme = cx.aura();
        let divider = theme.neutral.divider;
        let radius = theme.radius.lg;
        let card_bg = theme.neutral.card;
        let font_lg = theme.font_size.lg;
        let font_sm = theme.font_size.sm;
        let text1 = theme.neutral.text_1;
        let text3 = theme.neutral.text_3;
        let rendered = render_fn(window, cx);
        section = section.child(
            div()
                .flex().flex_col().gap_4().p_4().border_1()
                .border_color(divider).rounded(px(radius)).bg(card_bg)
                .child(
                    div().flex().flex_col().gap_1()
                        .child(div().text_size(px(font_lg)).text_color(text1).font_weight(gpui::FontWeight::BOLD).child(name))
                        .child(div().text_size(px(font_sm)).text_color(text3).child(desc)),
                )
                .child(rendered)
        );
    }
    section.into_any_element()
}

#[cfg(not(target_family = "wasm"))]
fn main() { run_gallery(); }

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() { gpui_platform::web_init(); run_gallery(); }
