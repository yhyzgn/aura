use aura_components::{Button, Card, Space, Tag, Text};
use aura_tray::{TrayCommand, TrayMenuItemSpec, default_aura_tray_menu};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TrayDemo {
        active_icon: "default",
        auto_show: true,
    })
    .into()
}

struct TrayDemo {
    active_icon: &'static str,
    auto_show: bool,
}

impl Render for TrayDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        page(
            "Tray 系统托盘",
            "aura-tray 封装 tray-icon/muda，提供进程常驻、动态图标、CheckBox 菜单和任意层级子菜单配置。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "托盘状态预览",
                    "Demo 不直接创建系统托盘，避免干扰普通组件预览；这里展示同一份配置 DSL 的交互效果。",
                    Space::new()
                        .vertical()
                        .gap_lg()
                        .child(
                            Card::new(
                                Space::new()
                                    .vertical()
                                    .gap_md()
                                    .child(
                                        Space::new()
                                            .gap_md()
                                            .align_center()
                                            .child(active_icon_tag(self.active_icon))
                                            .child(
                                                Space::new()
                                                    .vertical()
                                                    .gap_xs()
                                                    .child(Text::new("Aura Gallery 正在后台运行").bold())
                                                    .child(Text::new(format!(
                                                        "当前图标：{} · 自动显示：{}",
                                                        self.active_icon,
                                                        if self.auto_show { "开启" } else { "关闭" }
                                                    ))),
                                            ),
                                    )
                                    .child(
                                        Space::new()
                                            .gap_md()
                                            .wrap()
                                            .child(icon_button("默认", "default", entity.clone()))
                                            .child(icon_button("同步中", "syncing", entity.clone()))
                                            .child(icon_button("错误", "error", entity.clone()))
                                            .child(toggle_auto_show_button(self.auto_show, entity.clone())),
                                    ),
                            )
                            .no_shadow(),
                        )
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(Tag::new("QuitMode::Explicit").warning())
                                .child(Tag::new("TrayIcon::set_icon").success())
                                .child(Tag::new("CheckMenuItem").info())
                                .child(Tag::new("N-level Submenu").info()),
                        ),
                ))
                .child(section(
                    "丰富菜单配置",
                    "同一套 TrayMenuItemSpec 支持普通动作、CheckBox、动态图标命令、二级/三级/更多层级菜单和分隔线。",
                    menu_preview(&default_aura_tray_menu(), 0),
                ))
                .child(section(
                    "运行时命令",
                    "菜单事件会被映射为稳定的 TrayCommand，GPUI 主窗口只需要处理 Show/Hide/Toggle/Quit/SetIcon/Custom。",
                    command_table(),
                )),
        )
    }
}

fn active_icon_tag(name: &str) -> Tag {
    match name {
        "syncing" => Tag::new("↻ syncing").warning().large(),
        "error" => Tag::new("! error").danger().large(),
        _ => Tag::new("A default").success().large(),
    }
}

fn icon_button(label: &'static str, icon: &'static str, entity: Entity<TrayDemo>) -> Button {
    Button::new(label).primary().on_click(move |_, _, cx| {
        let _ = entity.update(cx, |demo, cx| {
            demo.active_icon = icon;
            cx.notify();
        });
    })
}

fn toggle_auto_show_button(auto_show: bool, entity: Entity<TrayDemo>) -> Button {
    Button::new(if auto_show {
        "关闭 Auto Show"
    } else {
        "开启 Auto Show"
    })
    .on_click(move |_, _, cx| {
        let _ = entity.update(cx, |demo, cx| {
            demo.auto_show = !demo.auto_show;
            cx.notify();
        });
    })
}

fn menu_preview(specs: &[TrayMenuItemSpec], depth: usize) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .children(specs.iter().map(|spec| render_menu_item(spec, depth)))
}

fn render_menu_item(spec: &TrayMenuItemSpec, depth: usize) -> gpui::AnyElement {
    match spec {
        TrayMenuItemSpec::Action { label, command, .. } => menu_line(
            depth,
            "Action",
            label,
            format!("id = {}", command.id()),
            false,
        ),
        TrayMenuItemSpec::Check {
            label,
            command,
            checked,
            ..
        } => menu_line(
            depth,
            if *checked {
                "☑ CheckBox"
            } else {
                "☐ CheckBox"
            },
            label,
            format!("id = {}", command.id()),
            false,
        ),
        TrayMenuItemSpec::Submenu {
            label, children, ..
        } => Space::new()
            .vertical()
            .gap_sm()
            .child(menu_line(
                depth,
                "Submenu",
                label,
                format!("{} children", children.len()),
                true,
            ))
            .child(menu_preview(children, depth + 1))
            .into_any_element(),
        TrayMenuItemSpec::Separator => {
            Text::new(format!("{}────────", indent(depth))).into_any_element()
        }
    }
}

fn menu_line(
    depth: usize,
    kind: &'static str,
    label: &str,
    detail: String,
    submenu: bool,
) -> gpui::AnyElement {
    Card::new(
        Space::new()
            .vertical()
            .gap_xs()
            .child(
                Space::new()
                    .gap_sm()
                    .child(Tag::new(kind).small())
                    .child(Text::new(format!(
                        "{}{}{}",
                        indent(depth),
                        label,
                        if submenu { " ›" } else { "" }
                    ))),
            )
            .child(Text::new(detail)),
    )
    .no_shadow()
    .into_any_element()
}

fn command_table() -> impl IntoElement {
    let commands = [
        TrayCommand::Show,
        TrayCommand::Hide,
        TrayCommand::Toggle,
        TrayCommand::SetIcon("syncing".into()),
        TrayCommand::Custom("deep-action".into()),
        TrayCommand::Quit,
    ];

    Space::new()
        .vertical()
        .gap_sm()
        .children(commands.map(|command| {
            Card::new(Text::new(format!("{command:?}  →  {}", command.id())).nowrap()).no_shadow()
        }))
}

fn indent(depth: usize) -> String {
    "  ".repeat(depth)
}
