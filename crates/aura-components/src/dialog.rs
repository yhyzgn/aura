use aura_core::{Config};
use gpui::{
    prelude::*, px, App, Context, IntoElement, Render, Window,
    div, AnyElement, MouseButton, actions, SharedString,
};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use std::sync::Arc;

actions!(dialog, [Close]);

pub struct Dialog {
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<DialogView>) -> AnyElement + 'static>,
    close_on_click_outside: bool,
}

pub struct DialogView {
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    close_on_click_outside: bool,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl DialogView {
    fn new(
        title: SharedString,
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        close_on_click_outside: bool,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            title,
            content,
            close_on_click_outside,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for DialogView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let title = self.title.clone();
        let content_fn = self.content.clone();
        let on_close = self.on_close.clone();
        let close_on_click_outside = self.close_on_click_outside;

        div()
            .absolute()
            .size_full()
            .bg(gpui::rgba(0x00000066))
            .flex().items_center().justify_center()
            .when(close_on_click_outside, |s| {
                s.on_mouse_down(MouseButton::Left, {
                    let on_close = on_close.clone();
                    move |_, window, cx| {
                        on_close(window, cx);
                    }
                })
            })
            .on_action(cx.listener({
                let on_close = on_close.clone();
                move |_, _action: &Close, window, cx| {
                    on_close(window, cx);
                }
            }))
            .child(
                div()
                    .w(px(400.0))
                    .bg(theme.neutral.card)
                    .rounded(px(theme.radius.md))
                    .shadow_xl()
                    .on_mouse_down(MouseButton::Left, |_, _, _| {}) // Consume
                    .child(
                        div().p_4().border_b_1().border_color(theme.neutral.border).flex().justify_between().items_center()
                            .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
                            .child(
                                div().id("close-btn").cursor_pointer().child(Icon::new(IconName::X).size(px(16.0)).color(theme.neutral.icon))
                                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                        on_close(window, cx);
                                    })
                            )
                    )
                    .child(div().p_4().child(content_fn(_window, cx)))
            )
    }
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            title: SharedString::default(),
            content: Arc::new(|_, _| div().child("Dialog Content").into_any_element()),
            close_on_click_outside: true,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    pub fn close_on_click_outside(mut self, c: bool) -> Self {
        self.close_on_click_outside = c;
        self
    }

    pub fn content<F, E>(mut self, f: F) -> Self 
    where 
        F: Fn(&mut Window, &mut Context<DialogView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }

    pub fn show(self, cx: &mut App) {
        let title = self.title;
        let content = self.content;
        let close_on_click_outside = self.close_on_click_outside;
        
        let view = cx.new(|_cx| DialogView::new(
            title,
            content,
            close_on_click_outside,
            |_window, _cx| {
                aura_core::clear_active_modal(_cx);
            }
        ));
        
        aura_core::set_active_modal(view.into(), cx);
    }
}
