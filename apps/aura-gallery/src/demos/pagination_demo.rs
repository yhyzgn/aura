use aura_components::{Card, Pagination};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| PaginationDemo {
        basic: cx.new(|_| {
            Pagination::new(50)
                .id("pagination-demo-basic")
                .on_change(|page, _, _| println!("Page changed to: {}", page))
        }),
        background: cx.new(|_| {
            Pagination::new(100)
                .id("pagination-demo-background")
                .background(true)
                .on_change(|page, _, _| println!("Page changed to: {}", page))
        }),
        page_sizes: cx.new(|_| {
            Pagination::new(400)
                .id("pagination-demo-page-sizes")
                .page_size(20)
                .page_sizes(vec![10, 20, 50, 100])
                .background(true)
                .layout("total, sizes, prev, pager, next, jumper")
                .on_change(|page, _, _| println!("Page changed to: {}", page))
                .on_page_size_change(|size, _, _| println!("Page size changed to: {}", size))
        }),
    })
    .into()
}

struct PaginationDemo {
    basic: Entity<Pagination>,
    background: Entity<Pagination>,
    page_sizes: Entity<Pagination>,
}

impl Render for PaginationDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .id("pagination-scroll")
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Pagination 分页"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("当数据量过多时，使用分页分解数据。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(Card::new(self.basic.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("带有背景色的分页"),
                    )
                    .child(Card::new(self.background.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("附加功能 (Total, Sizes, Jumper)"),
                    )
                    .child(Card::new(self.page_sizes.clone())),
            )
    }
}
