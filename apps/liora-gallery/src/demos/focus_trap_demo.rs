use gpui::{AnyView, App, Context, Entity, Focusable, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card, showcase_card_wide};
use liora_components::{Button, Card, FocusTrap, Input, Space, Tag, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| FocusTrapDemo {
        enabled: true,
        focused_index: 0,
        primary_input: cx.new(|cx| Input::new("", cx).placeholder("Primary field inside trap")),
        secondary_input: cx.new(|cx| Input::new("", cx).placeholder("Secondary field inside trap")),
    })
    .into()
}

struct FocusTrapDemo {
    enabled: bool,
    focused_index: usize,
    primary_input: Entity<Input>,
    secondary_input: Entity<Input>,
}

impl FocusTrapDemo {
    fn policy(&self) -> FocusTrap {
        if self.enabled {
            FocusTrap::new()
        } else {
            FocusTrap::new().disabled()
        }
    }

    fn active_field(&self) -> Entity<Input> {
        if self.focused_index % 2 == 0 {
            self.primary_input.clone()
        } else {
            self.secondary_input.clone()
        }
    }
}

impl Render for FocusTrapDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let policy = self.policy();
        let view = cx.entity().clone();
        let prev_view = view.clone();
        let next_view = view.clone();
        let toggle_view = view.clone();

        page(
            "FocusTrap 焦点策略",
            "FocusTrap 定义弹窗、抽屉、浮层等 overlay 的焦点约束策略；下方预览用两个输入框展示焦点在 trap 内循环的状态。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Trap preview",
                    "启用后，Previous/Next 会在 trap 内部字段之间循环；禁用时，策略标签会明确显示不会约束焦点。",
                    showcase_card_wide(
                        "Trap preview",
                        "Use the controls to move the highlighted focus target inside the trap policy scope.",
                        Space::new()
                            .vertical()
                            .gap_lg()
                            .child(
                                Space::new()
                                    .wrap()
                                    .gap_sm()
                                    .child(policy_badge("enabled", policy.enabled))
                                    .child(policy_badge("restore focus", policy.restore_focus))
                                    .child(policy_badge("escape closes", policy.close_on_escape)),
                            )
                            .child(
                                Card::new(
                                    Space::new()
                                        .vertical()
                                        .gap_md()
                                        .child(Text::new(format!(
                                            "Current trapped target: {}",
                                            if self.focused_index % 2 == 0 {
                                                "Primary field"
                                            } else {
                                                "Secondary field"
                                            }
                                        )))
                                        .child(self.primary_input.clone())
                                        .child(self.secondary_input.clone()),
                                )
                                .no_shadow(),
                            )
                            .child(
                                Space::new()
                                    .wrap()
                                    .gap_sm()
                                    .child(Button::new("Focus previous").on_click(
                                        move |_, window, cx| {
                                            focus_relative(&prev_view, -1, window, cx);
                                        },
                                    ))
                                    .child(Button::new("Focus next").primary().on_click(
                                        move |_, window, cx| {
                                            focus_relative(&next_view, 1, window, cx);
                                        },
                                    ))
                                    .child(Button::new(if self.enabled {
                                        "Disable trap"
                                    } else {
                                        "Enable trap"
                                    })
                                    .secondary()
                                    .on_click(move |_, window, cx| {
                                        let target = toggle_view.update(cx, |this, cx| {
                                            this.enabled = !this.enabled;
                                            cx.notify();
                                            this.active_field()
                                        });
                                        target.read(cx).focus_handle(cx).focus(window, cx);
                                    })),
                            ),
                    ),
                ))
                .child(section(
                    "策略对象",
                    "FocusTrap 保持为轻量策略对象，可被 Dialog、Drawer、Popover 等 overlay 复用。",
                    Space::new()
                        .wrap()
                        .gap_lg()
                        .child(policy_card("Default modal", FocusTrap::new()))
                        .child(policy_card(
                            "Non-closable modal",
                            FocusTrap::new().restore_focus(true).close_on_escape(false),
                        ))
                        .child(policy_card("Disabled scope", FocusTrap::new().disabled())),
                )),
        )
    }
}

fn focus_relative(view: &Entity<FocusTrapDemo>, delta: isize, window: &mut Window, cx: &mut App) {
    let target = view.update(cx, |this, cx| {
        if this.enabled {
            let next = (this.focused_index as isize + delta).rem_euclid(2);
            this.focused_index = next as usize;
        }
        cx.notify();
        this.active_field()
    });
    target.read(cx).focus_handle(cx).focus(window, cx);
}

fn policy_badge(label: &'static str, enabled: bool) -> Tag {
    if enabled {
        Tag::new(label).success().round(true)
    } else {
        Tag::new(label).warning().round(true)
    }
}

fn policy_card(label: &'static str, policy: FocusTrap) -> gpui::AnyElement {
    showcase_card(
        label,
        "Policy flags passed to overlay components.",
        Space::new()
            .vertical()
            .gap_sm()
            .child(policy_badge("enabled", policy.enabled))
            .child(Text::new(format!(
                "restore_focus: {}",
                policy.restore_focus
            )))
            .child(Text::new(format!(
                "close_on_escape: {}",
                policy.close_on_escape
            ))),
    )
    .into_any_element()
}
