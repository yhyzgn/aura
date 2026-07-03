//! Bundled Lucide icon names for Liora's native GPUI icon component.
//!
//! The generated `IconName` enum maps each synchronized Lucide SVG asset to a
//! strongly typed Rust variant. Each variant can be passed directly to
//! `liora_icons::Icon::new(...)` or converted into a GPUI element.

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use std::borrow::Cow;

impl IconName {
    /// Returns the virtual path for this optimized SVG asset.
    pub fn svg_path(&self) -> String {
        liora_icons::icon_svg_asset_path(
            self.set(),
            self.file(),
            Some(format!(
                "{}/assets/svgs/{}",
                env!("CARGO_MANIFEST_DIR"),
                self.file()
            )),
        )
        .into_owned()
    }
}

impl liora_icons::IntoIconPath for IconName {
    fn icon_path(&self) -> Cow<'static, str> {
        liora_icons::icon_svg_asset_path(
            self.set(),
            self.file(),
            Some(format!(
                "{}/assets/svgs/{}",
                env!("CARGO_MANIFEST_DIR"),
                self.file()
            )),
        )
    }
}

impl gpui::IntoElement for IconName {
    type Element = gpui::Component<liora_icons::Icon>;
    fn into_element(self) -> Self::Element {
        liora_icons::Icon::new(self).into_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::AssetSource;
    use liora_icons::IntoIconPath;

    #[test]
    fn icon_paths_are_virtual_and_load_through_asset_source() {
        let icon = IconName::all()
            .first()
            .copied()
            .expect("icon set should not be empty");
        let path = icon.icon_path();
        assert!(path.starts_with(liora_icons::ICON_SVG_ASSET_PREFIX));

        let bytes = liora_icons::IconAssetSource
            .load(&path)
            .expect("virtual icon loading should not error")
            .expect("virtual icon should resolve through IconAssetSource");
        let svg = std::str::from_utf8(&bytes).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox") || svg.contains("viewbox"));
    }
}
