use aura_components::{Text, Upload, UploadFile, UploadStatus};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| UploadDemo::new(cx)).into()
}

struct UploadDemo {
    basic: Entity<Upload>,
    drag: Entity<Upload>,
    picture: Entity<Upload>,
    limited: Entity<Upload>,
    disabled: Entity<Upload>,
}

impl UploadDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| {
                Upload::new()
                    .button_text("选择文件")
                    .tip("支持移除列表项；真实文件选择可通过 on_select 接入宿主文件选择器。")
                    .width(px(420.0))
                    .add_file(
                        UploadFile::new("spec", "产品需求说明.pdf")
                            .size(428_000)
                            .status(UploadStatus::Success),
                    )
                    .add_file(
                        UploadFile::new("draft", "设计稿-v2.fig")
                            .size(2_480_000)
                            .status(UploadStatus::Uploading)
                            .progress(68),
                    )
            }),
            drag: cx.new(|_| {
                Upload::new()
                    .drag(true)
                    .multiple(true)
                    .accept(".png, .jpg, .pdf")
                    .button_text("拖拽文件到这里上传")
                    .tip("拖拽区域用于展示上传入口，点击事件通过 on_select 暴露给业务层。")
                    .width(px(420.0))
                    .add_file(
                        UploadFile::new("error", "合同扫描件.jpg")
                            .size(820_000)
                            .status(UploadStatus::Error)
                            .description("网络中断"),
                    )
            }),
            picture: cx.new(|_| {
                Upload::new()
                    .picture_card()
                    .button_text("上传图片")
                    .width(px(420.0))
                    .files([
                        UploadFile::new("cover", "cover.png")
                            .size(512_000)
                            .status(UploadStatus::Success),
                        UploadFile::new("banner", "banner.jpg")
                            .size(1_240_000)
                            .status(UploadStatus::Uploading)
                            .progress(42),
                    ])
            }),
            limited: cx.new(|_| {
                Upload::new()
                    .limit(1)
                    .button_text("达到数量限制")
                    .tip("limit=1 时已有文件，入口自动禁用。")
                    .width(px(420.0))
                    .add_file(
                        UploadFile::new("only", "唯一附件.zip")
                            .size(5_120_000)
                            .status(UploadStatus::Ready),
                    )
            }),
            disabled: cx.new(|_| {
                Upload::new()
                    .disabled(true)
                    .button_text("禁用上传")
                    .tip("禁用状态下入口不可用。")
                    .width(px(420.0))
            }),
        }
    }
}

impl Render for UploadDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

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
                            .child("Upload 上传"),
                    )
                    .child(div().text_sm().text_color(theme.neutral.text_3).child(
                        "用于呈现上传入口和文件列表，支持拖拽样式、图片卡片列表、进度、状态、数量限制和移除回调。",
                    )),
            )
            .child(section(
                "基础文件列表",
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(self.basic.clone())
                    .child(Text::new("点击垃圾桶图标可从组件内部移除文件。").size(px(theme.font_size.sm))),
            ))
            .child(section("拖拽上传样式", self.drag.clone()))
            .child(section("图片卡片列表", self.picture.clone()))
            .child(section("数量限制", self.limited.clone()))
            .child(section("禁用状态", self.disabled.clone()))
    }
}

fn section(title: &'static str, content: impl IntoElement) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(content)
}
