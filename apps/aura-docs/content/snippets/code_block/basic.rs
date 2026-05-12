//! Basic CodeBlock with copy support.

use aura_components::CodeBlock;

fn basic_code_block() -> CodeBlock {
    CodeBlock::new("cargo run -p aura-docs")
        .shell()
        .copyable(true)
}

fn main() {
    let _ = basic_code_block();
}
