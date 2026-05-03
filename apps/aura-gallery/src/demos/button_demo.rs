use gpui::{div, prelude::*, px, AnyElement, Window, Context};
use aura_components::AuraButton;
use aura_core::AuraContextExt;
use crate::Gallery;

pub fn render(_window: &mut Window, cx: &mut Context<Gallery>) -> AnyElement {
    // Create all button entities first (mutably borrows cx)
    let types = row7(
        cx,
        AuraButton::new("Default"),
        AuraButton::new("Tertiary").tertiary(),
        AuraButton::new("Primary").primary(),
        AuraButton::new("Info").info(),
        AuraButton::new("Success").success(),
        AuraButton::new("Warning").warning(),
        AuraButton::new("Error").danger(),
    );
    let secondary = row7(
        cx,
        AuraButton::new("Default").secondary(),
        AuraButton::new("Tertiary").tertiary().secondary(),
        AuraButton::new("Primary").primary().secondary(),
        AuraButton::new("Info").info().secondary(),
        AuraButton::new("Success").success().secondary(),
        AuraButton::new("Warning").warning().secondary(),
        AuraButton::new("Error").danger().secondary(),
    );
    let secondary_nb = row6(
        cx,
        AuraButton::new("Default").secondary().border(false),
        AuraButton::new("Primary").primary().secondary().border(false),
        AuraButton::new("Info").info().secondary().border(false),
        AuraButton::new("Success").success().secondary().border(false),
        AuraButton::new("Warning").warning().secondary().border(false),
        AuraButton::new("Error").danger().secondary().border(false),
    );
    let sizes = row3(
        cx,
        AuraButton::new("Small").primary().small(),
        AuraButton::new("Default").primary(),
        AuraButton::new("Large").primary().large(),
    );
    let states = row3(
        cx,
        AuraButton::new("Disabled").primary().disabled(true),
        AuraButton::new("Loading").primary().loading(true),
        AuraButton::new("Secondary Disabled").primary().secondary().disabled(true),
    );
    let rounded = row4(
        cx,
        AuraButton::new("4px").primary().rounded(4.0),
        AuraButton::new("12px").primary().rounded(12.0),
        AuraButton::new("20px").primary().rounded(20.0),
        AuraButton::new("Pill").primary().rounded(9999.0),
    );

    // Now read theme (immutable borrow is fine since we're done mutating cx)
    let theme = cx.aura();
    div()
        .flex().flex_col().gap_3()
        .child(section_header(theme, "Types 按钮类型"))
        .child(demo_row(types))
        .child(section_header(theme, "Secondary 次要按钮"))
        .child(demo_row(secondary))
        .child(section_header(theme, "Secondary · no border"))
        .child(demo_row(secondary_nb))
        .child(section_header(theme, "Sizes 尺寸"))
        .child(demo_row(sizes))
        .child(section_header(theme, "States 状态"))
        .child(demo_row(states))
        .child(section_header(theme, "Rounded 圆角"))
        .child(demo_row(rounded))
        .into_any_element()
}

fn row7(cx: &mut Context<Gallery>, a0: AuraButton, a1: AuraButton, a2: AuraButton, a3: AuraButton, a4: AuraButton, a5: AuraButton, a6: AuraButton) -> Vec<AnyElement> {
    vec![btn(cx, a0), btn(cx, a1), btn(cx, a2), btn(cx, a3), btn(cx, a4), btn(cx, a5), btn(cx, a6)]
}

fn row6(cx: &mut Context<Gallery>, a0: AuraButton, a1: AuraButton, a2: AuraButton, a3: AuraButton, a4: AuraButton, a5: AuraButton) -> Vec<AnyElement> {
    vec![btn(cx, a0), btn(cx, a1), btn(cx, a2), btn(cx, a3), btn(cx, a4), btn(cx, a5)]
}

fn row4(cx: &mut Context<Gallery>, a0: AuraButton, a1: AuraButton, a2: AuraButton, a3: AuraButton) -> Vec<AnyElement> {
    vec![btn(cx, a0), btn(cx, a1), btn(cx, a2), btn(cx, a3)]
}

fn row3(cx: &mut Context<Gallery>, a0: AuraButton, a1: AuraButton, a2: AuraButton) -> Vec<AnyElement> {
    vec![btn(cx, a0), btn(cx, a1), btn(cx, a2)]
}

fn btn(cx: &mut Context<Gallery>, config: AuraButton) -> AnyElement {
    cx.new(|_| config).into_any_element()
}

fn section_header(theme: &aura_theme::AuraTheme, label: impl IntoElement) -> impl IntoElement {
    div()
        .text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD).mt_2().child(label)
}

fn demo_row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    div().flex().flex_row().gap_2().items_center().flex_wrap().children(elements)
}
