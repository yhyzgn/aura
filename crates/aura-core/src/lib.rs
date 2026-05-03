use gpui::{App, Context, Global, Hsla, prelude::*};

pub use aura_theme::Theme;

pub struct Config {
    pub theme: Theme,
    pub z_index_base: u32,
}

impl Global for Config {}

pub fn init_aura(cx: &mut App, theme: Theme) {
    cx.set_global(Config {
        theme,
        z_index_base: 1000,
    });
}

pub fn aura_theme<'a, V>(cx: &'a Context<'a, V>) -> &'a Theme {
    &cx.global::<Config>().theme
}

pub trait ContextExt {
    fn aura(&self) -> &Theme;
}

impl<V> ContextExt for Context<'_, V> {
    fn aura(&self) -> &Theme {
        &self.global::<Config>().theme
    }
}

pub trait ElementExt: IntoElement + Sized {
    fn size(self, _size: aura_theme::ButtonSize) -> Self {
        self
    }

    fn variant(self, _variant: aura_theme::ButtonVariant) -> Self {
        self
    }

    fn disabled(self, _disabled: bool) -> Self {
        self
    }

    fn loading(self, _loading: bool) -> Self {
        self
    }
}

impl<E: IntoElement> ElementExt for E {}

pub fn z_index_popup<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 100
}

pub fn z_index_modal<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 200
}

pub fn z_index_notification<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 300
}

pub fn z_index_tooltip<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 400
}

pub fn hex_color(hex: u32) -> Hsla {
    gpui::rgb(hex).into()
}
