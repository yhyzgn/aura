use aura_components::{
    Button, ButtonSize, ButtonVariant, Table, TableAlign, TableColumn, TableRow, TableSortOrder,
    TableSortState, Text,
};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, SharedString, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TableDemo {
        sort_key: None,
        sort_order: None,
    })
    .into()
}

struct TableDemo {
    sort_key: Option<SharedString>,
    sort_order: Option<TableSortOrder>,
}

impl Render for TableDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let view = cx.entity().clone();
        let sort_key = self.sort_key.clone();
        let sort_order = self.sort_order;

        let mut sortable_table = Table::new(sortable_columns(theme))
            .rows(sorted_rows(sort_key.as_ref(), sort_order))
            .stripe(true)
            .border(true)
            .on_sort_change(move |state: TableSortState, _, cx| {
                view.update(cx, |this, cx| {
                    this.sort_key = state.order.map(|_| state.key.clone());
                    this.sort_order = state.order;
                    cx.notify();
                });
            });

        if let Some(key) = sort_key {
            sortable_table = sortable_table.sort(key, sort_order);
        }

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Table 表格"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于展示多条结构化数据，支持基础表格、斑马纹、边框、固定表头、加载、空状态、自定义表头和开发者启用的三态排序。"),
                    ),
            )
            .child(section(
                "自定义表头与可排序列",
                sortable_table.into_any_element(),
            ))
            .child(section(
                "基础用法",
                Table::new(basic_columns()).rows(basic_rows()).into_any_element(),
            ))
            .child(section(
                "斑马纹与边框",
                Table::new(basic_columns())
                    .rows(basic_rows())
                    .stripe(true)
                    .border(true)
                    .into_any_element(),
            ))
            .child(section(
                "固定表头",
                Table::new(basic_columns())
                    .rows(long_rows())
                    .stripe(true)
                    .fixed_header(true)
                    .height(px(260.0))
                    .into_any_element(),
            ))
            .child(section(
                "加载状态",
                Table::new(basic_columns())
                    .rows(basic_rows())
                    .loading(true)
                    .into_any_element(),
            ))
            .child(section(
                "空数据",
                Table::new(basic_columns())
                    .empty_text("暂无订单数据")
                    .into_any_element(),
            ))
    }
}

fn section(title: &'static str, content: gpui::AnyElement) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(content)
}

fn basic_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("date", "日期").width(px(120.0)),
        TableColumn::new("name", "姓名").width(px(120.0)),
        TableColumn::new("address", "地址").min_width(px(260.0)),
        TableColumn::new("status", "状态")
            .width(px(120.0))
            .align(TableAlign::Center),
        TableColumn::new("action", "操作")
            .width(px(120.0))
            .align(TableAlign::Right),
    ]
}

fn sortable_columns(theme: &aura_theme::Theme) -> Vec<TableColumn> {
    vec![
        TableColumn::new("date", "日期").width(px(120.0)).sortable(),
        TableColumn::new("name", "姓名")
            .header(
                Text::new("客户")
                    .bold()
                    .text_color(theme.primary.base)
                    .nowrap(),
            )
            .width(px(120.0))
            .sortable(),
        TableColumn::new("address", "地址").min_width(px(260.0)),
        TableColumn::new("status", "状态")
            .width(px(120.0))
            .align(TableAlign::Center)
            .sortable(),
        TableColumn::new("action", "操作")
            .width(px(120.0))
            .align(TableAlign::Right),
    ]
}

#[derive(Clone)]
struct OrderRecord {
    date: &'static str,
    name: &'static str,
    address: &'static str,
    status: &'static str,
}

fn records() -> Vec<OrderRecord> {
    vec![
        OrderRecord {
            date: "2016-05-03",
            name: "Tom",
            address: "上海市普陀区金沙江路 1518 弄",
            status: "已完成",
        },
        OrderRecord {
            date: "2016-05-02",
            name: "Jack",
            address: "上海市普陀区金沙江路 1517 弄",
            status: "进行中",
        },
        OrderRecord {
            date: "2016-05-04",
            name: "Alice",
            address: "上海市普陀区金沙江路 1519 弄",
            status: "已完成",
        },
        OrderRecord {
            date: "2016-05-01",
            name: "Bob",
            address: "上海市普陀区金沙江路 1516 弄",
            status: "待处理",
        },
    ]
}

fn basic_rows() -> Vec<TableRow> {
    records().into_iter().map(record_row).collect()
}

fn sorted_rows(
    sort_key: Option<&SharedString>,
    sort_order: Option<TableSortOrder>,
) -> Vec<TableRow> {
    let mut records = records();

    if let (Some(key), Some(order)) = (sort_key, sort_order) {
        records.sort_by(|a, b| field_value(a, key).cmp(field_value(b, key)));
        if order == TableSortOrder::Descending {
            records.reverse();
        }
    }

    records.into_iter().map(record_row).collect()
}

fn field_value<'a>(record: &'a OrderRecord, key: &SharedString) -> &'a str {
    match key.as_ref() {
        "date" => record.date,
        "name" => record.name,
        "status" => record.status,
        "address" => record.address,
        _ => "",
    }
}

fn long_rows() -> Vec<TableRow> {
    (1..=16)
        .map(|i| {
            row(
                match i % 4 {
                    0 => "2016-05-04",
                    1 => "2016-05-01",
                    2 => "2016-05-02",
                    _ => "2016-05-03",
                },
                match i % 4 {
                    0 => "Tom",
                    1 => "Jack",
                    2 => "Alice",
                    _ => "Bob",
                },
                "上海市普陀区金沙江路 1518 弄",
                if i % 3 == 0 { "待处理" } else { "已完成" },
            )
        })
        .collect()
}

fn record_row(record: OrderRecord) -> TableRow {
    row(record.date, record.name, record.address, record.status)
}

fn row(
    date: &'static str,
    name: &'static str,
    address: &'static str,
    status: &'static str,
) -> TableRow {
    TableRow::new()
        .cell("date", date)
        .cell("name", name)
        .cell("address", address)
        .cell(
            "status",
            div()
                .px_2()
                .py_1()
                .rounded(px(999.0))
                .bg(match status {
                    "已完成" => gpui::green().opacity(0.12),
                    "进行中" => gpui::blue().opacity(0.12),
                    _ => gpui::yellow().opacity(0.18),
                })
                .text_color(match status {
                    "已完成" => gpui::green(),
                    "进行中" => gpui::blue(),
                    _ => gpui::yellow(),
                })
                .text_xs()
                .child(status),
        )
        .cell(
            "action",
            Button::new("查看")
                .variant(ButtonVariant::Primary)
                .size(ButtonSize::Small),
        )
}
