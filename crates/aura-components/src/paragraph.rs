use crate::Text;
use aura_core::Config;
use gpui::{
    App, Component, IntoElement, RenderOnce, SharedString, StyledText, TextRun, TextStyle,
    WhiteSpace, Window, div, prelude::*, px,
};

pub struct Paragraph {
    children: Vec<Text>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn with_text(text: impl Into<SharedString>) -> Self {
        Self {
            children: vec![Text::new(text)],
        }
    }

    pub fn child(mut self, child: Text) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = Text>) -> Self {
        self.children.extend(children);
        self
    }

    fn default_text_style(theme: &aura_theme::Theme) -> TextStyle {
        let font_size = px(theme.font_size.md);
        let mut style = TextStyle::default();
        style.color = theme.neutral.text_2;
        style.font_size = font_size.into();
        style.line_height = px(theme.font_size.md * 1.6).into();
        style.white_space = WhiteSpace::Normal;
        style.text_overflow = None;
        style.line_clamp = None;
        style
    }

    fn styled_text_parts(self, theme: &aura_theme::Theme) -> (SharedString, Vec<TextRun>) {
        let default_style = Self::default_text_style(theme);
        let mut full_text = String::new();
        let mut runs = Vec::new();

        for segment in self.children {
            if segment.content.is_empty() {
                continue;
            }

            full_text.push_str(segment.content.as_ref());
            runs.push(segment.to_text_run(&default_style));
        }

        (full_text.into(), runs)
    }
}

impl RenderOnce for Paragraph {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let (full_text, runs) = self.styled_text_parts(theme);
        let font_size = px(theme.font_size.md);

        div()
            .w_full()
            .text_size(font_size)
            .line_height(font_size * 1.6)
            .text_color(theme.neutral.text_2)
            .whitespace_normal()
            .child(StyledText::new(full_text).with_runs(runs))
    }
}

impl IntoElement for Paragraph {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{FontStyle, FontWeight};

    #[test]
    fn paragraph_composes_segments_into_one_styled_text_run_list() {
        let theme = aura_theme::Theme::light();
        let (text, runs) = Paragraph::new()
            .child(Text::new("Hello ").bold())
            .child(Text::new("世界").italic())
            .styled_text_parts(&theme);

        assert_eq!(text.as_ref(), "Hello 世界");
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0].len, "Hello ".len());
        assert_eq!(runs[1].len, "世界".len());
        assert_eq!(runs[0].font.weight, FontWeight::BOLD);
        assert_eq!(runs[1].font.style, FontStyle::Italic);
    }

    #[test]
    fn text_segments_map_inline_code_style_to_text_runs() {
        let theme = aura_theme::Theme::light();
        let default_style = Paragraph::default_text_style(&theme);
        let run = Text::new("code")
            .code_style(&theme)
            .bold()
            .underline()
            .to_text_run(&default_style);

        assert_eq!(run.len, "code".len());
        assert_eq!(run.font.family.as_ref(), "Monospace");
        assert_eq!(run.font.weight, FontWeight::BOLD);
        assert_eq!(run.color, theme.danger.base);
        assert_eq!(run.background_color, Some(theme.neutral.hover));
        assert!(run.underline.is_some());
    }

    #[test]
    fn paragraph_default_style_keeps_native_wrapping_without_truncation() {
        let theme = aura_theme::Theme::light();
        let style = Paragraph::default_text_style(&theme);

        assert_eq!(style.white_space, WhiteSpace::Normal);
        assert!(style.text_overflow.is_none());
        assert!(style.line_clamp.is_none());
    }
}
