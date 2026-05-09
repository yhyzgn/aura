use aura_core::{Config, push_portal};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, Bounds, Context, Element, ElementId, Hsla, IntoElement, MouseButton, Pixels, Render,
    SharedString, Window, div, prelude::*, px,
};
use std::sync::Arc;

pub struct ColorPicker {
    id: SharedString,
    value: SharedString,
    presets: Vec<SharedString>,
    disabled: bool,
    show_label: bool,
    width: Option<Pixels>,
    is_open: bool,
    last_bounds: Option<Bounds<Pixels>>,
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
            is_open: false,
            last_bounds: None,
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

    pub fn rainbow_palette() -> Vec<SharedString> {
        [
            "#FF0000", "#FF3B00", "#FF7A00", "#FFB800", "#FFFF00", "#B8FF00", "#7AFF00", "#3BFF00",
            "#00FF00", "#00FF7A", "#00FFFF", "#00B8FF", "#007AFF", "#003BFF", "#0000FF", "#3B00FF",
            "#7A00FF", "#B800FF", "#FF00FF", "#FF00B8", "#FF007A", "#FF003B", "#FFFFFF", "#000000",
            "#F2F3F5", "#C0C4CC", "#909399", "#606266", "#303133", "#1F2D3D",
        ]
        .into_iter()
        .map(Into::into)
        .collect()
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
        self.is_open = false;
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
        let entity = cx.entity().clone();

        if self.is_open && !disabled {
            let bounds = self.last_bounds;
            let panel_id = id.clone();
            let theme_portal = theme.clone();
            let selected_for_panel = selected.clone();
            let presets = self.presets.clone();
            let entity_for_portal = entity.clone();

            push_portal(
                move |_window, _cx| {
                    let (top, left) = if let Some(bounds) = bounds {
                        (bounds.bottom() + px(6.0), bounds.left())
                    } else {
                        (px(100.0), px(100.0))
                    };
                    let close_entity = entity_for_portal.clone();
                    let panel = render_color_panel(
                        panel_id.clone(),
                        selected_for_panel.clone(),
                        presets.clone(),
                        theme_portal.clone(),
                        entity_for_portal.clone(),
                    )
                    .absolute()
                    .top(top)
                    .left(left);

                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .bg(gpui::transparent_black())
                        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                            close_entity.update(cx, |picker, cx| {
                                picker.is_open = false;
                                cx.notify();
                            });
                        })
                        .child(panel)
                        .into_any_element()
                },
                cx,
            );
        }

        div()
            .flex()
            .items_center()
            .gap_2()
            .when_some(self.width, |s, width| s.w(width))
            .child(
                div()
                    .id(format!("{}-trigger", id))
                    .relative()
                    .w(px(40.0))
                    .h(px(40.0))
                    .rounded(px(theme.radius.md))
                    .border_1()
                    .border_color(if self.is_open {
                        theme.primary.base
                    } else {
                        theme.neutral.border
                    })
                    .bg(if disabled {
                        theme.neutral.hover
                    } else {
                        swatch_color
                    })
                    .when(!disabled, |s| {
                        s.cursor_pointer()
                            .hover(|s| s.cursor_pointer().border_color(theme.primary.base))
                    })
                    .when(disabled, |s| s.cursor_not_allowed().opacity(0.55))
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .child(BoundsCapturer {
                                picker: entity.clone(),
                            }),
                    )
                    .child(
                        div()
                            .absolute()
                            .right(px(2.0))
                            .bottom(px(2.0))
                            .rounded(px(3.0))
                            .bg(theme.neutral.card.opacity(0.88))
                            .child(
                                Icon::new(IconName::ChevronDown)
                                    .size(px(12.0))
                                    .color(theme.neutral.icon),
                            ),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if !this.disabled {
                                this.is_open = !this.is_open;
                                cx.notify();
                            }
                        }),
                    ),
            )
            .when(self.show_label, |s| {
                s.child(
                    div()
                        .text_sm()
                        .font_family("monospace")
                        .text_color(if disabled {
                            theme.neutral.text_3
                        } else {
                            theme.neutral.text_1
                        })
                        .child(selected),
                )
            })
    }
}

fn render_color_panel(
    id: SharedString,
    selected: SharedString,
    presets: Vec<SharedString>,
    theme: aura_theme::Theme,
    picker: gpui::Entity<ColorPicker>,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(format!("{}-panel", id))
        .occlude()
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .w(px(252.0))
        .p_3()
        .flex()
        .flex_col()
        .gap_3()
        .rounded(px(theme.radius.lg))
        .border_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .shadow_lg()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(theme.neutral.text_1)
                .child("Rainbow"),
        )
        .child(color_grid(
            format!("{}-rainbow", id),
            ColorPicker::rainbow_palette(),
            selected.clone(),
            theme.clone(),
            picker.clone(),
            6,
            px(28.0),
        ))
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(theme.neutral.text_1)
                .child("Presets"),
        )
        .child(color_grid(
            format!("{}-preset", id),
            presets,
            selected,
            theme,
            picker,
            6,
            px(28.0),
        ))
}

fn color_grid(
    id: String,
    colors: Vec<SharedString>,
    selected: SharedString,
    theme: aura_theme::Theme,
    picker: gpui::Entity<ColorPicker>,
    columns: usize,
    size: Pixels,
) -> impl IntoElement {
    div()
        .flex()
        .flex_wrap()
        .gap_2()
        .max_w(px(columns as f32 * (f32::from(size) + 8.0)))
        .children(colors.into_iter().enumerate().map(move |(index, color)| {
            let hsla = hex_to_hsla(&color).unwrap_or(theme.primary.base);
            let active = color == selected;
            let picker = picker.clone();
            div()
                .id(format!("{}-{}", id, index))
                .w(size)
                .h(size)
                .p(px(2.0))
                .rounded(px(theme.radius.sm))
                .border_1()
                .border_color(if active {
                    theme.primary.base
                } else {
                    theme.neutral.border
                })
                .cursor_pointer()
                .hover(|s| s.cursor_pointer().bg(theme.neutral.hover))
                .child(color_square(
                    hsla,
                    px(f32::from(size) - 6.0),
                    theme.neutral.border,
                ))
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    picker.update(cx, |picker, cx| {
                        picker.select_color(color.clone(), window, cx);
                    });
                    cx.stop_propagation();
                })
        }))
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

struct BoundsCapturer {
    picker: gpui::Entity<ColorPicker>,
}

impl IntoElement for BoundsCapturer {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for BoundsCapturer {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, ()) {
        let mut style = gpui::Style::default();
        style.size.width = gpui::relative(1.0).into();
        style.size.height = gpui::relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        cx: &mut App,
    ) {
        self.picker.update(cx, |picker, _| {
            picker.last_bounds = Some(bounds);
        });
    }

    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        _window: &mut Window,
        _: &mut App,
    ) {
    }
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
