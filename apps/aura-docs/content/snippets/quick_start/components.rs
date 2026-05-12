use aura_components::{Button, CodeBlock, Space, Title};

Space::new()
    .vertical()
    .gap_lg()
    .child(Title::new("Aura UI").h2())
    .child(Button::new("Primary").primary())
    .child(CodeBlock::new("cargo run -p aura-docs").shell());
