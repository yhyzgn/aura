include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use std::borrow::Cow;

impl aura_icons::IntoIconPath for IconName {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(format!(
            "{}/assets/svgs/{}",
            env!("CARGO_MANIFEST_DIR"),
            self.file()
        ))
    }
}
