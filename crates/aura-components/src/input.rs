use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{prelude::*, px, App, FocusHandle, Focusable, Render, SharedString, Window, Context, MouseButton, MouseUpEvent};

pub struct Input {
    value: SharedString,
    placeholder: SharedString,
    disabled: bool,
    clearable: bool,
    icon_prefix: Option<IconName>,
    icon_suffix: Option<IconName>,
    focus_handle: FocusHandle,
}

impl Input {
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            value: value.into(), placeholder: SharedString::default(), disabled: false,
            clearable: false, icon_prefix: None, icon_suffix: None,
            focus_handle: cx.focus_handle(),
        }
    }
    pub fn placeholder(mut self, p: impl Into<SharedString>) -> Self { self.placeholder = p.into(); self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn clearable(mut self, c: bool) -> Self { self.clearable = c; self }
    pub fn icon_prefix(mut self, icon: IconName) -> Self { self.icon_prefix = Some(icon); self }
    pub fn icon_suffix(mut self, icon: IconName) -> Self { self.icon_suffix = Some(icon); self }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.value = SharedString::default(); cx.notify();
    }
}

impl Focusable for Input {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl Render for Input {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let h = 34.0; let icon_sz = 16.0;

        let (bg, text_c, border_c) = if self.disabled {
            (theme.neutral.hover, theme.neutral.text_disabled, theme.neutral.border)
        } else {
            (theme.neutral.card, theme.neutral.text_1, theme.neutral.border)
        };

        let display = if self.value.is_empty() { self.placeholder.clone() } else { self.value.clone() };
        let is_placeholder = self.value.is_empty();
        let ph_color = theme.neutral.text_3;

        let mut row = gpui::div()
            .flex().flex_row().items_center().gap_2()
            .h(px(h)).px(px(12.0)).rounded(px(theme.radius.md))
            .bg(bg).border_1().border_color(border_c).text_size(px(theme.font_size.md));

        if let Some(icon) = self.icon_prefix {
            row = row.child(Icon::new(icon).size(px(icon_sz)).color(theme.neutral.icon));
        }

        row = row.child(
            gpui::div().flex_1().h_full().flex().items_center()
                .text_color(if is_placeholder { ph_color } else { text_c })
                .child(display)
        );

        if self.clearable && !self.value.is_empty() && !self.disabled {
            row = row.child(
                gpui::div().cursor_pointer().flex_none()
                    .child(Icon::new(IconName::X).size(px(14.0)).color(theme.neutral.icon))
                    .on_mouse_up(MouseButton::Left, cx.listener(move |this: &mut Self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>| {
                        this.clear(cx);
                    }))
            );
        }

        if let Some(icon) = self.icon_suffix {
            row = row.child(Icon::new(icon).size(px(icon_sz)).color(theme.neutral.icon));
        }

        row
    }
}
