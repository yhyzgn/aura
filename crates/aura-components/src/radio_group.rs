use aura_core::Config;
use gpui::{
    App, Context, FocusHandle, Focusable, Hsla, KeyBinding, MouseButton, MouseUpEvent, Pixels,
    Render, Rgba, SharedString, Window, prelude::*, px,
};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
}

gpui::actions!(radio_group, [RadioGroupUp, RadioGroupDown]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadioGroupLayout {
    #[default]
    Vertical,
    Horizontal,
    Button,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadioGroupSize {
    Large,
    #[default]
    Default,
    Small,
}

impl RadioGroupSize {
    fn height(self) -> Pixels {
        match self {
            RadioGroupSize::Large => px(38.0),
            RadioGroupSize::Default => px(32.0),
            RadioGroupSize::Small => px(24.0),
        }
    }

    fn padding_x(self) -> Pixels {
        match self {
            RadioGroupSize::Large => px(18.0),
            RadioGroupSize::Default => px(14.0),
            RadioGroupSize::Small => px(10.0),
        }
    }

    fn text_size(self, theme: &aura_theme::Theme) -> Pixels {
        match self {
            RadioGroupSize::Large => px(theme.font_size.md),
            RadioGroupSize::Default => px(theme.font_size.md),
            RadioGroupSize::Small => px(theme.font_size.sm),
        }
    }
}

pub struct RadioGroup {
    selected: usize,
    disabled: bool,
    options: Vec<SharedString>,
    layout: RadioGroupLayout,
    size: RadioGroupSize,
    stretch: bool,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl RadioGroup {
    pub fn new(
        options: Vec<impl Into<SharedString>>,
        selected: usize,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            selected,
            disabled: false,
            options: options.into_iter().map(|o| o.into()).collect(),
            layout: RadioGroupLayout::Vertical,
            size: RadioGroupSize::Default,
            stretch: false,
            focus_handle: cx.focus_handle(),
            on_change: None,
        }
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn layout(mut self, layout: RadioGroupLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.layout = RadioGroupLayout::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.layout = RadioGroupLayout::Horizontal;
        self
    }

    pub fn button(mut self) -> Self {
        self.layout = RadioGroupLayout::Button;
        self
    }

    pub fn size(mut self, size: RadioGroupSize) -> Self {
        self.size = size;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = RadioGroupSize::Large;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = RadioGroupSize::Small;
        self
    }

    pub fn stretch(mut self, stretch: bool) -> Self {
        self.stretch = stretch;
        self
    }

    pub fn block(self, block: bool) -> Self {
        self.stretch(block)
    }

    pub fn is_stretched(&self) -> bool {
        self.stretch
    }

    pub fn layout_kind(&self) -> RadioGroupLayout {
        self.layout
    }

    pub fn size_kind(&self) -> RadioGroupSize {
        self.size
    }

    pub fn on_change(mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("up", RadioGroupUp, None),
            KeyBinding::new("down", RadioGroupDown, None),
            KeyBinding::new("left", RadioGroupUp, None),
            KeyBinding::new("right", RadioGroupDown, None),
        ]);
    }

    fn up(&mut self, _: &RadioGroupUp, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled && self.selected > 0 {
            self.select(self.selected - 1, window, cx);
        }
    }

    fn down(&mut self, _: &RadioGroupDown, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled && self.selected + 1 < self.options.len() {
            self.select(self.selected + 1, window, cx);
        }
    }

    fn select(&mut self, idx: usize, window: &mut Window, cx: &mut Context<Self>) {
        if idx != self.selected {
            self.selected = idx;
            cx.notify();
            if let Some(ref cb) = self.on_change {
                cb(idx, window, cx);
            }
        }
    }

    fn render_button_group(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let radius = px(theme.radius.md);
        let height = self.size.height();
        let padding_x = self.size.padding_x();
        let text_size = self.size.text_size(&theme);

        let mut group = gpui::div()
            .flex()
            .items_center()
            .rounded(radius)
            .border_1()
            .border_color(theme.neutral.border)
            .overflow_hidden()
            .when(self.stretch, |s| s.w_full())
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::down));

        if !self.disabled {
            group = group.track_focus(&self.focus_handle);
        }

        for (idx, label) in self.options.iter().enumerate() {
            let checked = idx == self.selected;
            let is_first = idx == 0;
            let label = label.clone();
            let bg = if checked {
                theme.primary.base
            } else {
                theme.neutral.card
            };
            let text_color = if self.disabled {
                theme.neutral.text_disabled
            } else if checked {
                rgba(255, 255, 255, 1.0)
            } else {
                theme.neutral.text_1
            };
            let mut item = gpui::div()
                .h(height)
                .px(padding_x)
                .flex()
                .items_center()
                .justify_center()
                .when(self.stretch, |s| s.flex_1())
                .bg(bg)
                .text_size(text_size)
                .text_color(text_color)
                .child(label);

            if !is_first {
                item = item.border_l_1().border_color(theme.neutral.border);
            }
            if !self.disabled {
                item = item.cursor_pointer().hover(move |s| {
                    if checked {
                        s.cursor_pointer()
                    } else {
                        s.cursor_pointer().bg(theme.neutral.hover)
                    }
                });
                item = item.on_mouse_up(
                    MouseButton::Left,
                    cx.listener(
                        move |this: &mut Self,
                              _: &MouseUpEvent,
                              window: &mut Window,
                              cx: &mut Context<Self>| {
                            this.select(idx, window, cx);
                        },
                    ),
                );
            } else {
                item = item.cursor_not_allowed();
            }
            group = group.child(item);
        }

        group
    }
}

impl Focusable for RadioGroup {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RadioGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.layout == RadioGroupLayout::Button {
            return self.render_button_group(cx).into_any_element();
        }

        let theme = &cx.global::<Config>().theme;
        let focused = self.focus_handle.is_focused(_window);
        let sz = 16.0;
        let inner_sz = 8.0;

        let mut col = gpui::div()
            .flex()
            .when(self.layout == RadioGroupLayout::Vertical, |s| {
                s.flex_col().gap_2()
            })
            .when(self.layout == RadioGroupLayout::Horizontal, |s| {
                s.flex_row().gap_4().items_center()
            })
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::down));

        if !self.disabled {
            col = col.track_focus(&self.focus_handle);
            col = col.on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    window.focus(&this.focus_handle, cx);
                }),
            );
        }

        for (idx, label) in self.options.iter().enumerate() {
            let checked = idx == self.selected;
            let (border_color, dot_color) = if self.disabled {
                (theme.neutral.border, theme.neutral.text_disabled)
            } else if checked {
                (theme.primary.base, theme.primary.base)
            } else {
                (
                    if focused && checked {
                        theme.primary.base
                    } else {
                        theme.neutral.border
                    },
                    rgba(0, 0, 0, 0.0),
                )
            };

            let label_text = label.clone();
            let mut row = gpui::div().flex().flex_row().items_center().gap_2();

            if !self.disabled {
                row = row.cursor_pointer();
            } else {
                row = row.cursor_not_allowed();
            }

            let circle = gpui::div()
                .flex_none()
                .w(px(sz))
                .h(px(sz))
                .rounded(px(sz / 2.0))
                .border_1()
                .border_color(border_color)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    gpui::div()
                        .w(px(inner_sz))
                        .h(px(inner_sz))
                        .rounded(px(inner_sz / 2.0))
                        .bg(dot_color),
                );

            row = row.child(circle);
            row = row.child(
                gpui::div()
                    .text_size(px(theme.font_size.md))
                    .text_color(theme.neutral.text_1)
                    .child(label_text),
            );

            if !self.disabled {
                row = row.on_mouse_up(
                    MouseButton::Left,
                    cx.listener(
                        move |this: &mut Self,
                              _: &MouseUpEvent,
                              window: &mut Window,
                              cx: &mut Context<Self>| {
                            this.select(idx, window, cx);
                        },
                    ),
                );
            }

            col = col.child(row);
        }

        col.into_any_element()
    }
}
