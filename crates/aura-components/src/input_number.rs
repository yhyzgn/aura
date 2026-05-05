use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{prelude::*, px, App, Render, Window, Context, Focusable, FocusHandle, Entity, MouseButton};
use crate::Input;

pub struct InputNumber {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    precision: usize,
    disabled: bool,
    input: Entity<Input>,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(f64, &mut Window, &mut App) + 'static>>,
}

impl InputNumber {
    pub fn new(value: f64, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            Input::new(format!("{:.0}", value), cx)
        });

        let focus_handle = cx.focus_handle();
        
        Self {
            value,
            min: f64::MIN,
            max: f64::MAX,
            step: 1.0,
            precision: 0,
            disabled: false,
            input,
            focus_handle,
            on_change: None,
        }
    }

    pub fn min(mut self, min: f64) -> Self { self.min = min; self }
    pub fn max(mut self, max: f64) -> Self { self.max = max; self }
    pub fn step(mut self, step: f64) -> Self { self.step = step; self }
    pub fn precision(mut self, p: usize) -> Self { 
        self.precision = p; 
        self
    }
    pub fn disabled(mut self, d: bool, cx: &mut Context<Self>) -> Self {
        self.disabled = d;
        self.input.update(cx, |input, cx| { input.set_disabled(d, cx); });
        self
    }

    pub fn on_change(mut self, cb: impl Fn(f64, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    fn set_value(&mut self, val: f64, window: &mut Window, cx: &mut Context<Self>) {
        let val = val.clamp(self.min, self.max);
        if (val - self.value).abs() > f64::EPSILON || self.value == 0.0 {
            self.value = val;
            let formatted = format!("{:.*}", self.precision, self.value);
            self.input.update(cx, |input, cx| {
                input.set_value(formatted, cx);
            });
            if let Some(ref cb) = self.on_change {
                cb(self.value, window, cx);
            }
            cx.notify();
        }
    }

    fn increment(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.set_value(self.value + self.step, window, cx);
        }
    }

    fn decrement(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.set_value(self.value - self.step, window, cx);
        }
    }
}

impl Focusable for InputNumber {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl Render for InputNumber {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let icon_sz = 12.0;

        let mut row = gpui::div()
            .flex().flex_row().items_center()
            .h(px(34.0)); // Match Input height

        // Decrement button
        let mut dec_btn = gpui::div()
            .flex().items_center().justify_center()
            .w(px(32.0)).h_full()
            .bg(theme.neutral.hover)
            .border_color(theme.neutral.border).border_r_1();

        if !self.disabled {
            dec_btn = dec_btn.cursor_pointer().hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    this.decrement(window, cx);
                }));
        } else {
            dec_btn = dec_btn.cursor_not_allowed().opacity(0.5);
        }
        
        row = row.child(dec_btn.child(Icon::new(IconName::Minus).size(px(icon_sz)).color(theme.neutral.text_1)));
        row = row.child(gpui::div().flex_1().child(self.input.clone()));

        // Increment button
        let mut inc_btn = gpui::div()
            .flex().items_center().justify_center()
            .w(px(32.0)).h_full()
            .bg(theme.neutral.hover)
            .border_color(theme.neutral.border).border_l_1();

        if !self.disabled {
            inc_btn = inc_btn.cursor_pointer().hover(|s| s.bg(theme.neutral.border))
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    this.increment(window, cx);
                }));
        } else {
            inc_btn = inc_btn.cursor_not_allowed().opacity(0.5);
        }

        row = row.child(inc_btn.child(Icon::new(IconName::Plus).size(px(icon_sz)).color(theme.neutral.text_1)));
        
        row
    }
}
