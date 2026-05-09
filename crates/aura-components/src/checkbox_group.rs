use crate::{Checkbox, CheckboxChanged};
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Hsla, MouseButton, MouseUpEvent, Pixels, Render,
    Rgba, SharedString, Window, prelude::*, px,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxGroupLayout {
    #[default]
    Vertical,
    Horizontal,
    Button,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxGroupSize {
    Large,
    #[default]
    Default,
    Small,
}

impl CheckboxGroupSize {
    fn height(self) -> Pixels {
        match self {
            CheckboxGroupSize::Large => px(38.0),
            CheckboxGroupSize::Default => px(32.0),
            CheckboxGroupSize::Small => px(24.0),
        }
    }

    fn padding_x(self) -> Pixels {
        match self {
            CheckboxGroupSize::Large => px(18.0),
            CheckboxGroupSize::Default => px(14.0),
            CheckboxGroupSize::Small => px(10.0),
        }
    }

    fn text_size(self, theme: &aura_theme::Theme) -> Pixels {
        match self {
            CheckboxGroupSize::Large => px(theme.font_size.md),
            CheckboxGroupSize::Default => px(theme.font_size.md),
            CheckboxGroupSize::Small => px(theme.font_size.sm),
        }
    }
}

pub struct CheckboxGroup {
    selected: Vec<usize>,
    disabled: bool,
    focus_handle: FocusHandle,
    options: Vec<SharedString>,
    checkboxes: Vec<Entity<Checkbox>>,
    layout: CheckboxGroupLayout,
    size: CheckboxGroupSize,
    stretch: bool,
    on_change: Option<Box<dyn Fn(Vec<usize>, &mut Window, &mut App) + 'static>>,
}

impl CheckboxGroup {
    pub fn new(
        options: Vec<impl Into<SharedString>>,
        selected: Vec<usize>,
        cx: &mut Context<Self>,
    ) -> Self {
        let options: Vec<SharedString> = options.into_iter().map(|o| o.into()).collect();
        let mut checkboxes = Vec::new();

        for (i, label) in options.iter().enumerate() {
            let is_checked = selected.contains(&i);
            let checkbox = cx.new(|cx| Checkbox::new(is_checked, cx).label(label.clone()));

            // Subscribe to each checkbox's change
            cx.subscribe(
                &checkbox,
                move |this, _checkbox, event: &CheckboxChanged, cx| {
                    this.update_selection(i, event.0, cx);
                },
            )
            .detach();

            checkboxes.push(checkbox);
        }

        Self {
            selected,
            disabled: false,
            focus_handle: cx.focus_handle(),
            options,
            checkboxes,
            layout: CheckboxGroupLayout::Vertical,
            size: CheckboxGroupSize::Default,
            stretch: false,
            on_change: None,
        }
    }

    pub fn disabled(mut self, d: bool, cx: &mut Context<Self>) -> Self {
        self.disabled = d;
        for cb in &self.checkboxes {
            cb.update(cx, |cb, cx| {
                cb.set_disabled(d, cx);
            });
        }
        self
    }

    pub fn on_change(mut self, cb: impl Fn(Vec<usize>, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    pub fn layout(mut self, layout: CheckboxGroupLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.layout = CheckboxGroupLayout::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.layout = CheckboxGroupLayout::Horizontal;
        self
    }

    pub fn button(mut self) -> Self {
        self.layout = CheckboxGroupLayout::Button;
        self
    }

    pub fn size(mut self, size: CheckboxGroupSize) -> Self {
        self.size = size;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = CheckboxGroupSize::Large;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = CheckboxGroupSize::Small;
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

    pub fn layout_kind(&self) -> CheckboxGroupLayout {
        self.layout
    }

    pub fn size_kind(&self) -> CheckboxGroupSize {
        self.size
    }

    pub fn register_key_bindings(_cx: &mut App) {}

    fn update_selection(&mut self, idx: usize, checked: bool, cx: &mut Context<Self>) {
        if checked {
            if !self.selected.contains(&idx) {
                self.selected.push(idx);
                self.selected.sort();
            }
        } else {
            self.selected.retain(|&i| i != idx);
        }
        cx.notify();
    }

    fn toggle_idx(&mut self, idx: usize, cx: &mut Context<Self>) {
        if self.disabled || idx >= self.options.len() {
            return;
        }
        let checked = !self.selected.contains(&idx);
        self.update_selection(idx, checked, cx);
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
            .when(!self.stretch, |s| s.self_start());

        if !self.disabled {
            group = group.track_focus(&self.focus_handle);
        }

        for (idx, label) in self.options.iter().enumerate() {
            let checked = self.selected.contains(&idx);
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
                .gap_2()
                .bg(bg)
                .text_size(text_size)
                .text_color(text_color);

            if checked {
                item = item.child(Icon::new(IconName::Check).size(px(12.0)).color(text_color));
            }
            item = item.child(label);

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
                              _: &mut Window,
                              cx: &mut Context<Self>| {
                            this.toggle_idx(idx, cx);
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

impl Focusable for CheckboxGroup {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CheckboxGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.layout == CheckboxGroupLayout::Button {
            return self.render_button_group(cx).into_any_element();
        }

        let mut col = gpui::div()
            .flex()
            .when(self.layout == CheckboxGroupLayout::Vertical, |s| {
                s.flex_col().gap_2()
            })
            .when(self.layout == CheckboxGroupLayout::Horizontal, |s| {
                s.flex_row().gap_4().items_center()
            });

        if !self.disabled {
            col = col.track_focus(&self.focus_handle);
        }

        for cb_entity in &self.checkboxes {
            col = col.child(cb_entity.clone());
        }

        col.into_any_element()
    }
}
