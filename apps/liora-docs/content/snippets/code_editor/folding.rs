use gpui::{App, AppContext, Entity};
use liora_components::{CodeEditor, CodeLanguage, CodeTheme};

const SOURCE: &str = r#"pub struct Workspace {
    name: String,
}

impl Workspace {
    pub fn open(path: &str) -> Self {
        Self {
            name: path.to_string(),
        }
    }

    pub fn render(&self) {
        println!("{}", self.name);
    }
}

mod tests {
    #[test]
    fn opens_workspace() {
        let workspace = Workspace::open("liora");
        assert_eq!(workspace.name, "liora");
    }
}
"#;

pub fn code_editor_folding(cx: &mut App) -> Entity<CodeEditor> {
    cx.new(|cx| {
        CodeEditor::new(SOURCE, cx)
            .language(CodeLanguage::Rust)
            .theme(CodeTheme::OneDark)
            .rows(8)
            .indent_guides(true)
            .fold_range(3, 9, "impl block")
            .fold_range(13, 17, "test module")
            .code_folding(true)
    })
}
