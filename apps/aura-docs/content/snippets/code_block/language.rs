//! Explicit language selection for syntax highlighting.

use aura_components::CodeBlock;

fn rust_code_block() -> CodeBlock {
    CodeBlock::new(r#"fn main() { println!(\"Aura\"); }"#).language("rust")
}

fn main() {
    let _ = rust_code_block();
}
