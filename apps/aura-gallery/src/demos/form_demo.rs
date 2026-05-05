use aura_components::{Checkbox, CheckboxGroup, Form, FormItem, Input, InputNumber, Radio, RadioGroup, Rate, Select, Slider, Switch, Textarea};
use gpui::{
    div, prelude::*, App, Context, Entity, IntoElement, Render, Window,
};

pub fn render(cx: &mut App) -> Entity<FormDemo> {
    cx.new(|cx| FormDemo::new(cx))
}

pub struct FormDemo {
    switch_on: Entity<Switch>,
    switch_off: Entity<Switch>,
    switch_disabled: Entity<Switch>,
    switch_disabled_on: Entity<Switch>,
    cb_checked: Entity<Checkbox>,
    cb_unchecked: Entity<Checkbox>,
    cb_labeled: Entity<Checkbox>,
    _cb_disabled: Entity<Checkbox>,
    _cb_disabled_checked: Entity<Checkbox>,
    cb_group: Entity<CheckboxGroup>,
    radio_checked: Entity<Radio>,
    radio_unchecked: Entity<Radio>,
    radio_labeled: Entity<Radio>,
    _radio_disabled: Entity<Radio>,
    _radio_disabled_checked: Entity<Radio>,
    radio_group: Entity<RadioGroup>,
    _radio_group_disabled: Entity<RadioGroup>,
    _input_plain: Entity<Input>,
    _input_placeholder: Entity<Input>,
    input_icon: Entity<Input>,
    _input_clearable: Entity<Input>,
    _input_disabled: Entity<Input>,
    input_number: Entity<InputNumber>,
    _input_number_precision: Entity<InputNumber>,
    _textarea: Entity<Textarea>,
    textarea_limit: Entity<Textarea>,
    slider_basic: Entity<Slider>,
    _slider_step: Entity<Slider>,
    rate_basic: Entity<Rate>,
    _rate_custom: Entity<Rate>,
    select_basic: Entity<Select>,
}

impl FormDemo {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            switch_on: cx.new(|cx| Switch::new(true, cx)),
            switch_off: cx.new(|cx| Switch::new(false, cx)),
            switch_disabled: cx.new(|cx| Switch::new(false, cx).disabled(true)),
            switch_disabled_on: cx.new(|cx| Switch::new(true, cx).disabled(true)),
            cb_checked: cx.new(|cx| Checkbox::new(true, cx)),
            cb_unchecked: cx.new(|cx| Checkbox::new(false, cx)),
            cb_labeled: cx.new(|cx| Checkbox::new(false, cx).label("Label")),
            _cb_disabled: cx.new(|cx| Checkbox::new(false, cx).disabled(true)),
            _cb_disabled_checked: cx.new(|cx| Checkbox::new(true, cx).disabled(true)),
            cb_group: cx.new(|cx| CheckboxGroup::new(vec!["Option 1", "Option 2", "Option 3"], vec![0, 2], cx)),
            radio_checked: cx.new(|cx| Radio::new(true, cx)),
            radio_unchecked: cx.new(|cx| Radio::new(false, cx)),
            radio_labeled: cx.new(|cx| Radio::new(false, cx).label("Label")),
            _radio_disabled: cx.new(|cx| Radio::new(false, cx).disabled(true)),
            _radio_disabled_checked: cx.new(|cx| Radio::new(true, cx).disabled(true)),
            radio_group: cx.new(|cx| RadioGroup::new(vec!["Option A", "Option B", "Option C"], 1, cx)),
            _radio_group_disabled: cx.new(|cx| RadioGroup::new(vec!["Disabled A", "Disabled B"], 0, cx).disabled(true)),
            _input_plain: cx.new(|cx| Input::new("", cx)),
            _input_placeholder: cx.new(|cx| Input::new("", cx).placeholder("Type something...")),
            input_icon: cx.new(|cx| {
                Input::new("", cx)
                    .placeholder("Search")
                    .icon_prefix(aura_icons_lucide::IconName::Search)
                    .clearable(true)
            }),
            _input_clearable: cx.new(|cx| Input::new("Clear me", cx).clearable(true)),
            _input_disabled: cx.new(|cx| Input::new("Disabled", cx).disabled(true)),
            input_number: cx.new(|cx| InputNumber::new(10.0, cx).min(0.0).max(100.0)),
            _input_number_precision: cx.new(|cx| InputNumber::new(1.23, cx).precision(2).step(0.01)),
            _textarea: cx.new(|cx| Textarea::new("Line 1\nLine 2", cx).rows(3)),
            textarea_limit: cx.new(|cx| Textarea::new("Limited content", cx).max_length(50).rows(2)),
            slider_basic: cx.new(|cx| Slider::new(50.0, cx)),
            _slider_step: cx.new(|cx| Slider::new(20.0, cx).step(10.0)),
            rate_basic: cx.new(|cx| Rate::new(3.0, cx)),
            _rate_custom: cx.new(|cx| Rate::new(4.0, cx).max(10)),
            select_basic: cx.new(|cx| Select::new(vec!["Apple", "Banana", "Orange", "Grape", "Watermelon"], Some(1), cx)),
        }
    }
}

impl Render for FormDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _theme = &cx.global::<aura_core::Config>().theme;

        Form::new()
            .child(FormItem::new().label("Switch 开关").child(
                div().flex().flex_row().gap_4().items_center()
                    .child(self.switch_on.clone())
                    .child(self.switch_off.clone())
                    .child(self.switch_disabled.clone())
                    .child(self.switch_disabled_on.clone())
            ))
            .child(FormItem::new().label("Checkbox 多选").required(true).child(
                div().flex().flex_row().gap_4().items_center()
                    .child(self.cb_checked.clone())
                    .child(self.cb_unchecked.clone())
                    .child(self.cb_labeled.clone())
            ))
            .child(FormItem::new().label("CheckboxGroup 多选组").child(self.cb_group.clone()))
            .child(FormItem::new().label("Radio 单选").child(
                div().flex().flex_row().gap_4().items_center()
                    .child(self.radio_checked.clone())
                    .child(self.radio_unchecked.clone())
                    .child(self.radio_labeled.clone())
            ))
            .child(FormItem::new().label("RadioGroup 单选组").child(self.radio_group.clone()))
            .child(FormItem::new().label("Select 下拉选择").child(self.select_basic.clone()))
            .child(FormItem::new().label("Input 输入框").required(true).child(self.input_icon.clone()))
            .child(FormItem::new().label("InputNumber 数字输入").child(self.input_number.clone()))
            .child(FormItem::new().label("Textarea 文本域").error("This is an error message").child(self.textarea_limit.clone()))
            .child(FormItem::new().label("Slider 滑块").child(self.slider_basic.clone()))
            .child(FormItem::new().label("Rate 评分").child(self.rate_basic.clone()))
    }
}
