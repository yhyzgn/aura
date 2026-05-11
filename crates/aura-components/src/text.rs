use aura_core::Config;
use gpui::{
    App, Component, FontStyle, FontWeight, Hsla, IntoElement, Pixels, RenderOnce, SharedString,
    StrikethroughStyle, TextRun, TextStyle, UnderlineStyle, Window, div, prelude::*, px,
};

#[derive(Clone)]
pub struct Text {
    pub(crate) content: SharedString,
    pub(crate) color: Option<Hsla>,
    pub(crate) bg: Option<Hsla>,
    pub(crate) size: Option<Pixels>,
    pub(crate) weight: Option<FontWeight>,
    pub(crate) style: Option<FontStyle>,
    pub(crate) underline: bool,
    pub(crate) strikethrough: bool,
    pub(crate) font_family: Option<SharedString>,
    pub(crate) wrap: bool,
    pub(crate) fill_width_on_wrap: bool,
}

impl Text {
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            color: None,
            bg: None,
            size: None,
            weight: None,
            style: None,
            underline: false,
            strikethrough: false,
            font_family: None,
            wrap: true,
            fill_width_on_wrap: false,
        }
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn bg(mut self, bg: Hsla) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn bold(mut self) -> Self {
        self.weight = Some(FontWeight::BOLD);
        self
    }

    pub fn font_style(mut self, style: FontStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn italic(mut self) -> Self {
        self.style = Some(FontStyle::Italic);
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub fn font_family(mut self, family: impl Into<SharedString>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// Enable normal whitespace wrapping and let the text take the parent width.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self.fill_width_on_wrap = true;
        self
    }

    /// Alias for [`Text::wrap`].
    pub fn auto_wrap(self) -> Self {
        self.wrap()
    }

    /// Keep the text on a single line.
    pub fn nowrap(mut self) -> Self {
        self.wrap = false;
        self.fill_width_on_wrap = false;
        self
    }

    /// Convenience for "code" style
    pub fn code_style(mut self, theme: &aura_theme::Theme) -> Self {
        self.font_family = Some("Monospace".into());
        self.bg = Some(theme.neutral.hover);
        self.text_color(theme.danger.base)
    }

    pub(crate) fn apply_to_text_style(&self, mut style: TextStyle) -> TextStyle {
        if let Some(color) = self.color {
            style.color = color;
        }

        if let Some(bg) = self.bg {
            style.background_color = Some(bg);
        }

        if let Some(weight) = self.weight {
            style.font_weight = weight;
        }

        if let Some(font_style) = self.style {
            style.font_style = font_style;
        }

        if let Some(family) = self.font_family.clone() {
            style.font_family = family;
        }

        if self.underline {
            style.underline = Some(UnderlineStyle {
                thickness: px(1.0),
                color: self.color,
                ..Default::default()
            });
        }

        if self.strikethrough {
            style.strikethrough = Some(StrikethroughStyle {
                thickness: px(1.0),
                color: self.color,
            });
        }

        style
    }

    pub(crate) fn to_text_run(&self, default_style: &TextStyle) -> TextRun {
        self.apply_to_text_style(default_style.clone())
            .to_run(self.content.len())
    }
}

impl RenderOnce for Text {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        let font_size = self.size.unwrap_or_else(|| px(theme.font_size.md));
        let mut el = div()
            .child(self.content.clone())
            .text_size(font_size)
            .line_height(font_size * 1.6)
            .text_color(self.color.unwrap_or(theme.neutral.text_2));

        if self.wrap {
            el = el.whitespace_normal();
            if self.fill_width_on_wrap {
                el = el.w_full().flex_shrink();
            }
        } else {
            el = el.whitespace_nowrap();
        }

        if let Some(bg) = self.bg {
            el = el.bg(bg).px_1().rounded(px(2.0));
        }

        if let Some(weight) = self.weight {
            el = el.font_weight(weight);
        }

        if let Some(style) = self.style {
            // In some GPUI versions, it's .italic(), in others it's .font_style(style)
            // If .font_style failed, let's try matching on style
            if style == FontStyle::Italic {
                el = el.italic();
            }
        }

        if self.underline {
            el = el.underline();
        }

        if self.strikethrough {
            el = el.line_through();
        }

        if let Some(family) = self.font_family {
            el = el.font_family(family);
        }

        el
    }
}

impl IntoElement for Text {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
