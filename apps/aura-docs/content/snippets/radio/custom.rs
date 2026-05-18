//! RadioGroup custom option layout and selected styles.

use aura_components::{RadioGroup, RadioOptionStyle, Space};
use gpui::{AppContext, Context, Render, Window, px, rgb};

fn styled_cards(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(vec!["Daily", "Weekly", "Monthly"], 1, cx)
        .horizontal()
        .option_style(
            RadioOptionStyle::new()
                .bg(rgb(0xf8fafc).into())
                .selected_bg(rgb(0xecfeff).into())
                .selected_text_color(rgb(0x0e7490).into())
                .selected_border_color(rgb(0x06b6d4).into())
                .hover_bg(rgb(0xf0fdfa).into())
                .radius(px(12.0))
                .padding(px(14.0), px(10.0)),
        )
}

fn styled_chips(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(vec!["Low", "Medium", "High"], 2, cx)
        .horizontal()
        .option_style(
            RadioOptionStyle::new()
                .bg(gpui::transparent_black())
                .selected_bg(rgb(0x7c3aed).into())
                .selected_text_color(gpui::white())
                .selected_border_color(rgb(0x7c3aed).into())
                .radius(px(999.0))
                .padding(px(16.0), px(8.0))
                .show_indicator(false),
        )
}

struct RadioCustomDemo {
    cards: gpui::Entity<RadioGroup>,
    chips: gpui::Entity<RadioGroup>,
}

impl RadioCustomDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            cards: cx.new(styled_cards),
            chips: cx.new(styled_chips),
        }
    }
}

impl Render for RadioCustomDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .vertical()
            .gap_md()
            .child(self.cards.clone())
            .child(self.chips.clone())
    }
}

fn main() {}
