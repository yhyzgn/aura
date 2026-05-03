use gpui::{prelude::*, px, App, Hsla, Rgba, Render, Window, Context, MouseButton, MouseUpEvent, Focusable, FocusHandle};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a }.into()
}

pub struct Switch {
    checked: bool,
    disabled: bool,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl Switch {
    pub fn new(checked: bool, cx: &mut Context<Self>) -> Self {
        Self { checked, disabled: false, focus_handle: cx.focus_handle(), on_change: None }
    }

    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn on_change(mut self, cb: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb)); self
    }

    fn toggle(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.checked = !self.checked;
            cx.notify();
            if let Some(ref cb) = self.on_change {
                cb(self.checked, window, cx);
            }
        }
    }
}

impl Focusable for Switch {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl Render for Switch {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;
        let w = 40.0; let h = 22.0; let thumb_sz = 16.0;
        let thumb_offset = if self.checked { w - thumb_sz - 3.0 } else { 3.0 };

        let thumb_color = if self.disabled {
            theme.neutral.text_disabled
        } else {
            rgba(255, 255, 255, 1.0)
        };
        let track_color = if self.disabled {
            theme.neutral.hover
        } else if self.checked {
            theme.primary.base
        } else {
            theme.neutral.border
        };

        gpui::div()
            .flex_none().w(px(w)).h(px(h)).rounded(px(h / 2.0))
            .bg(track_color).cursor_pointer()
            .child(
                gpui::div()
                    .absolute().left(px(thumb_offset)).top(px((h - thumb_sz) / 2.0))
                    .w(px(thumb_sz)).h(px(thumb_sz)).rounded(px(thumb_sz / 2.0))
                    .bg(thumb_color)
            )
            .on_mouse_up(MouseButton::Left, cx.listener(move |this: &mut Self, _: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>| {
                this.toggle(window, cx);
            }))
            .track_focus(&self.focus_handle(cx))
    }
}
