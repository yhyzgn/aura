use std::path::PathBuf;

use gpui::{App, AppContext, Entity};
use liora_components::{CodeEditor, CodeEditorConfig, CodeLanguage};

const SOURCE: &str = r#"fn configured_editor() {
    println!("settings came from editor.toml");
}
"#;

pub fn code_editor_from_config_file(cx: &mut App) -> Entity<CodeEditor> {
    let config_path = PathBuf::from("assets/editor.toml");

    cx.new(move |cx| {
        let mut editor = CodeEditor::new(SOURCE, cx).language(CodeLanguage::Rust);

        if let Ok(config) = CodeEditorConfig::load_from_path(&config_path) {
            editor = editor.config(config);
        }

        editor
    })
}

pub fn reload_editor_config(
    editor: &Entity<CodeEditor>,
    config_path: &PathBuf,
    cx: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = CodeEditorConfig::load_from_path(config_path)?;
    editor.update(cx, |editor, cx| editor.set_config(config, cx));
    Ok(())
}
