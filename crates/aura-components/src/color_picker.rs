use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, Context, Hsla, IntoElement, MouseButton, Pixels, Render, SharedString, Window, div,
    prelude::*, px,
};
use std::sync::Arc;

pub struct ColorPicker {
    id: SharedString,
    value: SharedString,
    presets: Vec<SharedString>,
    disabled: bool,
    show_label: bool,
    width: Option<Pixels>,
    on_change: Option<Arc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}

impl ColorPicker {
    #[track_caller]
    pub fn new(value: impl Into<SharedString>) -> Self {
        let caller = std::panic::Location::caller();
        let value = value.into();
        Self {
            id: format!("color-picker-{caller}").into(),
            value: Self::normalize_hex(value.as_ref()).unwrap_or_else(|| "#409EFF".into()),
            presets: default_presets(),
            disabled: false,
            show_label: true,
            width: None,
            on_change: None,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn value(mut self, value: impl AsRef<str>) -> Self {
        if let Some(value) = Self::normalize_hex(value.as_ref()) {
            self.value = value;
        }
        self
    }

    pub fn presets(mut self, presets: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.presets = presets
            .into_iter()
            .filter_map(|value| {
                let value = value.into();
                Self::normalize_hex(value.as_ref())
            })
            .collect();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_label(mut self, show_label: bool) -> Self {
        self.show_label = show_label;
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn on_change(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Arc::new(f));
        self
    }

    pub fn normalize_hex(input: &str) -> Option<SharedString> {
        let trimmed = input.trim();
        let raw = trimmed.strip_prefix('#').unwrap_or(trimmed);
        let expanded = match raw.len() {
            3 => raw.chars().flat_map(|ch| [ch, ch]).collect::<String>(),
            6 => raw.to_string(),
            _ => return None,
        };
        if !expanded.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return None;
        }
        Some(format!("#{}", expanded.to_ascii_uppercase()).into())
    }

    pub fn hex_rgb(input: &str) -> Option<(u8, u8, u8)> {
        let normalized = Self::normalize_hex(input)?;
        let raw = normalized.as_ref().trim_start_matches('#');
        Some((
            u8::from_str_radix(&raw[0..2], 16).ok()?,
            u8::from_str_radix(&raw[2..4], 16).ok()?,
            u8::from_str_radix(&raw[4..6], 16).ok()?,
        ))
    }

    fn select_color(&mut self, color: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled || self.value == color {
            return;
        }
        self.value = color.clone();
        if let Some(on_change) = &self.on_change {
            on_change(color, window, cx);
        }
        cx.notify();
    }
}

impl Render for ColorPicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let selected = self.value.clone();
        let swatch_color = hex_to_hsla(&selected).unwrap_or(theme.primary.base);
        let id = self.id.clone();
        let disabled = self.disabled;

        div()
            .flex()
            .flex_col()
            .gap_3()
            .when_some(self.width, |s, width| s.w(width))
            .child(
                div()
                    .id(format!("{}-trigger", id))
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_3()
                    .py_2()
                    .rounded(px(theme.radius.md))
                    .border_1()
                    .border_color(theme.neutral.border)
                    .bg(if disabled {
                        theme.neutral.hover
                    } else {
                        theme.neutral.card
                    })
                    .text_color(if disabled {
                        theme.neutral.text_3
                    } else {
                        theme.neutral.text_1
                    })
                    .child(color_square(swatch_color, px(24.0), theme.neutral.border))
                    .when(self.show_label, |s| {
                        s.child(
                            div()
                                .text_sm()
                                .font_family("monospace")
                                .child(selected.clone()),
                        )
                    })
                    .child(
                        Icon::new(IconName::ChevronDown)
                            .size(px(14.0))
                            .color(if disabled {
                                theme.neutral.text_3
                            } else {
                                theme.neutral.icon
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .children(self.presets.iter().enumerate().map(|(index, color)| {
                        let color_value = color.clone();
                        let is_active = color == &selected;
                        let hsla = hex_to_hsla(color).unwrap_or(theme.primary.base);
                        div()
                            .id(format!("{}-preset-{}", id, index))
                            .p(px(2.0))
                            .rounded(px(theme.radius.sm))
                            .border_1()
                            .border_color(if is_active {
                                theme.primary.base
                            } else {
                                theme.neutral.border
                            })
                            .when(!disabled, |s| {
                                s.cursor_pointer()
                                    .hover(|s| s.bg(theme.neutral.hover).cursor_pointer())
                            })
                            .when(disabled, |s| s.cursor_not_allowed().opacity(0.55))
                            .child(color_square(hsla, px(24.0), theme.neutral.border))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, window, cx| {
                                    this.select_color(color_value.clone(), window, cx);
                                }),
                            )
                    })),
            )
    }
}

fn color_square(color: Hsla, size: Pixels, border: Hsla) -> impl IntoElement {
    div()
        .w(size)
        .h(size)
        .rounded(px(4.0))
        .border_1()
        .border_color(border)
        .bg(color)
}

fn hex_to_hsla(value: &str) -> Option<Hsla> {
    let (r, g, b) = ColorPicker::hex_rgb(value)?;
    Some(gpui::rgb((u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)).into())
}

fn default_presets() -> Vec<SharedString> {
    [
        "#409EFF", "#67C23A", "#E6A23C", "#F56C6C", "#909399", "#000000", "#FFFFFF", "#626AEF",
        "#13C2C2", "#722ED1", "#EB2F96", "#FA541C",
    ]
    .into_iter()
    .map(Into::into)
    .collect()
}
