use aura_components::{CodeBlock, CodeHighlighter, CodeTheme};

CodeBlock::new("cargo run -p aura-docs")
    .shell()
    .auto_theme(); // 默认：跟随 Aura 全局主题

CodeBlock::new(r#"fn main() { println!(\"Aura\"); }"#)
    .rust()
    .github_dark_theme();

CodeBlock::new("[package]\nname = \"aura\"")
    .toml()
    .highlighter(CodeHighlighter::Syntect)
    .theme(CodeTheme::Nord)
    .selectable(true);
