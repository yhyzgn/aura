//! Compile-check harness for all authored Rust snippets used by Aura Docs.
//!
//! This binary is not shown in the docs UI. It imports each snippet as a module
//! so `cargo check -p aura-docs --bin check_snippets` catches syntax errors,
//! missing imports, and stale public APIs in documentation examples.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[path = "../../content/snippets/architecture/render_pipeline.rs"]
mod architecture_render_pipeline;
#[path = "../../content/snippets/authoring/code_block.rs"]
mod authoring_code_block;
#[path = "../../content/snippets/button/rounded.rs"]
mod button_rounded;
#[path = "../../content/snippets/button/secondary.rs"]
mod button_secondary;
#[path = "../../content/snippets/button/sizes.rs"]
mod button_sizes;
#[path = "../../content/snippets/button/states.rs"]
mod button_states;
#[path = "../../content/snippets/button/text.rs"]
mod button_text;
#[path = "../../content/snippets/button/types.rs"]
mod button_types;
#[path = "../../content/snippets/code_block/basic.rs"]
mod code_block_basic;
#[path = "../../content/snippets/code_block/inline.rs"]
mod code_block_inline;
#[path = "../../content/snippets/code_block/language.rs"]
mod code_block_language;
#[path = "../../content/snippets/code_block/theme.rs"]
mod code_block_theme;
#[path = "../../content/snippets/input/affix.rs"]
mod input_affix;
#[path = "../../content/snippets/input/basic.rs"]
mod input_basic;
#[path = "../../content/snippets/input/password.rs"]
mod input_password;
#[path = "../../content/snippets/input/states.rs"]
mod input_states;
#[path = "../../content/snippets/live_demo/button.rs"]
mod live_demo_button;
#[path = "../../content/snippets/markdown/state_machine.rs"]
mod markdown_state_machine;
#[path = "../../content/snippets/message/formatting.rs"]
mod message_formatting;
#[path = "../../content/snippets/message/types.rs"]
mod message_types;
#[path = "../../content/snippets/quick_start/components.rs"]
mod quick_start_components;
#[path = "../../content/snippets/quick_start/init.rs"]
mod quick_start_init;
#[path = "../../content/snippets/switch/basic.rs"]
mod switch_basic;
#[path = "../../content/snippets/switch/callback.rs"]
mod switch_callback;
#[path = "../../content/snippets/switch/disabled.rs"]
mod switch_disabled;
#[path = "../../content/snippets/typography/paragraph.rs"]
mod typography_paragraph;

fn main() {}
