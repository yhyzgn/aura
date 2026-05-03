use aura_components::{Title, Paragraph, Text};
use aura_core::Config;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement { Component::new(TypographyDemo).into_any_element() }

struct TypographyDemo;
impl RenderOnce for TypographyDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        div().flex().flex_col().gap_3()
            .child(Title::new("Title 标题").h1())
            .child(row(vec![
                Title::new("H1").h1(),
                Title::new("H2").h2(),
                Title::new("H3").h3(),
                Title::new("H4").h4(),
                Title::new("H5").h5(),
                Title::new("H6").h6(),
            ]))
            .child(div().h(px(8.0)))
            .child(Title::new("Paragraph 段落").h2())
            .child(Paragraph::new("Aura is a native component library built with Rust and GPUI. It provides a set of enterprise-grade UI components inspired by Element Plus and Naive UI. All components follow the RenderOnce + IntoElement pattern and read theme from global context automatically."))
            .child(div().h(px(8.0)))
            .child(Title::new("Text 文本").h2())
            .child(row(vec![
                Text::new("xs text").size(px(theme.font_size.xs)),
                Text::new("sm text").size(px(theme.font_size.sm)),
                Text::new("md text (default)"),
                Text::new("lg text").size(px(theme.font_size.lg)),
                Text::new("xl text").size(px(theme.font_size.xl)),
            ]))
            .child(div().w(px(300.0)).child(Text::new("Truncated text that is too long and gets cut with ellipsis...")))
            .child(div().h(px(4.0)))
            .child(Text::new("Rem-based sizing").size(gpui::rems(1.2)))
            .child(Text::new("Px-based sizing").size(px(16.0)))
    }
}

fn row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    div().flex().flex_row().gap_4().items_center().flex_wrap().children(elements)
}
