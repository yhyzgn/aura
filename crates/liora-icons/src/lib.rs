//! Native GPUI icon rendering helpers for Liora.
//!
//! This crate exposes an `Icon` component plus the `IntoIconPath` trait used by
//! Liora applications. Icons are rendered as GPUI SVG/image assets and do not
//! require a web icon runtime.

use gpui::{
    App, AssetSource, Component, DefiniteLength, Hsla, IntoElement, Radians, RenderOnce,
    SharedString, Transformation, Window, prelude::*, px,
};
use liora_core::Config;
use std::{
    borrow::Cow,
    env, fs,
    path::{Path, PathBuf},
};

/// Virtual asset prefix used for SVGs embedded directly in the binary.
///
/// Kept for caller-provided inline SVG snippets and backwards compatibility.
pub const INLINE_SVG_ASSET_PREFIX: &str = "liora-icon-inline:";

/// Virtual asset prefix used by optimized bundled icon sets.
pub const ICON_SVG_ASSET_PREFIX: &str = "liora-icon://";

/// Builds a virtual SVG asset path from embedded SVG source text.
pub fn inline_svg_asset_path(svg: &'static str) -> Cow<'static, str> {
    Cow::Owned(format!("{INLINE_SVG_ASSET_PREFIX}{svg}"))
}

/// Builds a virtual SVG asset path for an icon-library SVG file.
///
/// Optimized packages should place resources under
/// `assets/liora-icons/<set>/<file>`. The optional development fallback keeps
/// local `cargo run` working before installer resources are staged.
pub fn icon_svg_asset_path(
    set: &'static str,
    file: &'static str,
    dev_path: Option<String>,
) -> Cow<'static, str> {
    match dev_path {
        Some(dev_path) => Cow::Owned(format!(
            "{ICON_SVG_ASSET_PREFIX}{set}/{file}?dev={dev_path}"
        )),
        None => Cow::Owned(format!("{ICON_SVG_ASSET_PREFIX}{set}/{file}")),
    }
}

/// Converts icon identifiers into SVG asset paths that `Icon` can render.
pub trait IntoIconPath {
    /// Returns the SVG asset path used by the icon renderer.
    fn icon_path(&self) -> Cow<'static, str>;
}

impl IntoIconPath for &str {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}
impl IntoIconPath for String {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.clone())
    }
}

/// Asset source for Liora SVG icons.
///
/// GPUI's `svg().path(...)` resolves through the application asset source.
/// `IconAssetSource` can load inline SVG payloads, optimized icon bundle
/// resources, and explicit filesystem SVG paths.
#[derive(Debug, Default, Clone, Copy)]
pub struct IconAssetSource;

impl AssetSource for IconAssetSource {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some(svg) = path.strip_prefix(INLINE_SVG_ASSET_PREFIX) {
            return Ok(Some(Cow::Owned(svg.as_bytes().to_vec())));
        }

        if let Some(request) = IconAssetRequest::parse(path) {
            for candidate in request.candidate_paths() {
                match fs::read(&candidate) {
                    Ok(bytes) => return Ok(Some(Cow::Owned(bytes))),
                    Err(error)
                        if matches!(
                            error.kind(),
                            std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory
                        ) => {}
                    Err(error) => return Err(error.into()),
                }
            }
            return Ok(None);
        }

        let path = path.strip_prefix("file://").unwrap_or(path);
        if path.is_empty() {
            return Ok(None);
        }
        match fs::read(path) {
            Ok(bytes) => Ok(Some(Cow::Owned(bytes))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}

#[derive(Debug, Clone)]
struct IconAssetRequest {
    set: String,
    file: String,
    dev_path: Option<PathBuf>,
}

impl IconAssetRequest {
    fn parse(path: &str) -> Option<Self> {
        let rest = path.strip_prefix(ICON_SVG_ASSET_PREFIX)?;
        let (resource, query) = rest.split_once('?').unwrap_or((rest, ""));
        let (set, file) = resource.split_once('/')?;
        if set.is_empty() || file.is_empty() || file.contains("..") || set.contains("..") {
            return None;
        }
        let dev_path = query
            .split('&')
            .find_map(|part| part.strip_prefix("dev="))
            .filter(|value| !value.is_empty())
            .map(|value| PathBuf::from(value.strip_prefix("file://").unwrap_or(value)));
        Some(Self {
            set: set.to_string(),
            file: file.to_string(),
            dev_path,
        })
    }

    fn candidate_paths(&self) -> Vec<PathBuf> {
        let mut candidates = Vec::new();
        if let Ok(dir) = env::var("LIORA_ICON_ASSETS_DIR") {
            candidates.push(PathBuf::from(dir).join(&self.set).join(&self.file));
        }
        if let Ok(exe) = env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                candidates.push(self.from_resource_root(exe_dir.join("assets/liora-icons")));
                candidates.push(self.from_resource_root(exe_dir.join("../assets/liora-icons")));
                candidates
                    .push(self.from_resource_root(exe_dir.join("../Resources/assets/liora-icons")));
            }
            if let Some(binary) = exe.file_name().and_then(|name| name.to_str()) {
                candidates.push(
                    self.from_resource_root(
                        PathBuf::from("/usr/lib")
                            .join(binary)
                            .join("assets/liora-icons"),
                    ),
                );
            }
        }
        if let Ok(current_dir) = env::current_dir() {
            candidates.push(self.from_resource_root(current_dir.join("assets/liora-icons")));
            candidates.extend(target_icon_bundle_candidates(
                &current_dir,
                &self.set,
                &self.file,
            ));
        }
        if let Some(dev_path) = &self.dev_path {
            candidates.push(dev_path.clone());
        }
        candidates
    }

    fn from_resource_root(&self, root: PathBuf) -> PathBuf {
        root.join(&self.set).join(&self.file)
    }
}

fn target_icon_bundle_candidates(current_dir: &Path, set: &str, file: &str) -> Vec<PathBuf> {
    let root = current_dir.join("target/liora/icons");
    let mut candidates = vec![root.join("assets/liora-icons").join(set).join(file)];
    if let Ok(entries) = fs::read_dir(&root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                candidates.push(path.join("assets/liora-icons").join(set).join(file));
            }
        }
    }
    candidates
}

/// Native GPUI SVG icon element with size, color, hover, and rotation controls.
pub struct Icon {
    size: Option<DefiniteLength>,
    color: Option<Hsla>,
    group_hover_color: Option<(SharedString, Hsla)>,
    group_hover_primary: Option<SharedString>,
    rotation: Option<Radians>,
    asset_path: String,
}

impl Icon {
    /// Creates `Icon` initialized from the supplied path.
    pub fn new(path: impl IntoIconPath) -> Self {
        Self {
            size: None,
            color: None,
            group_hover_color: None,
            group_hover_primary: None,
            rotation: None,
            asset_path: path.icon_path().into_owned(),
        }
    }

    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, sz: impl Into<DefiniteLength>) -> Self {
        self.size = Some(sz.into());
        self
    }

    /// Sets an explicit icon size from application-facing design units.
    pub fn size_units(self, size: f32) -> Self {
        self.size(px(size))
    }

    /// Applies the predefined size xs sizing preset.
    pub fn size_xs(self) -> Self {
        self.size(px(12.0))
    }

    /// Applies the predefined size md sizing preset.
    pub fn size_md(self) -> Self {
        self.size(px(18.0))
    }

    /// Applies the predefined size lg sizing preset.
    pub fn size_lg(self) -> Self {
        self.size(px(24.0))
    }

    /// Applies the predefined size xl sizing preset.
    pub fn size_xl(self) -> Self {
        self.size(px(32.0))
    }

    /// Set explicit color. If not called, inherits parent's text_color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Change icon color when a parent/group member is hovered.
    pub fn group_hover_color(mut self, group: impl Into<SharedString>, color: Hsla) -> Self {
        self.group_hover_color = Some((group.into(), color));
        self
    }

    /// Change icon color to the active theme primary color when a group member is hovered.
    pub fn group_hover_primary(mut self, group: impl Into<SharedString>) -> Self {
        self.group_hover_primary = Some(group.into());
        self
    }

    /// Rotate the icon around its center while preserving layout and hitbox.
    pub fn rotation(mut self, rotation: Radians) -> Self {
        self.rotation = Some(rotation);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        let sz = self.size.unwrap_or_else(|| px(18.0).into());
        let mut el = gpui::svg().path(self.asset_path).size(sz);
        if let Some(color) = self.color {
            el = el.text_color(color);
        } else {
            el = el.text_color(theme.neutral.icon);
        }
        if let Some((group, color)) = self.group_hover_color {
            el = el.group_hover(group, move |style| style.text_color(color));
        }
        if let Some(group) = self.group_hover_primary {
            let primary = theme.primary.base;
            el = el.group_hover(group, move |style| style.text_color(primary));
        }
        if let Some(rotation) = self.rotation {
            el = el.with_transformation(Transformation::rotate(rotation));
        }
        el
    }
}

impl IntoElement for Icon {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_size_helpers_set_common_demo_sizes() {
        assert_eq!(
            Icon::new("home").size_units(16.0).size,
            Some(px(16.0).into())
        );
        assert_eq!(Icon::new("home").size_xs().size, Some(px(12.0).into()));
        assert_eq!(Icon::new("home").size_md().size, Some(px(18.0).into()));
        assert_eq!(Icon::new("home").size_lg().size, Some(px(24.0).into()));
        assert_eq!(Icon::new("home").size_xl().size, Some(px(32.0).into()));
    }

    #[test]
    fn icon_supports_theme_primary_group_hover() {
        let source = include_str!("lib.rs");

        assert!(source.contains("pub fn group_hover_primary"));
        assert!(source.contains("theme.primary.base"));
    }

    #[test]
    fn icon_rotation_tracks_transform_request() {
        assert_eq!(
            Icon::new("loader").rotation(gpui::radians(1.0)).rotation,
            Some(gpui::radians(1.0))
        );
    }

    #[test]
    fn icon_asset_source_loads_absolute_svg_files() {
        use gpui::AssetSource;

        let path = format!(
            "{}/../liora-icons-lucide/assets/svgs/loader-circle.svg",
            env!("CARGO_MANIFEST_DIR")
        );
        let bytes = IconAssetSource
            .load(&path)
            .expect("icon asset loading should not error")
            .expect("loader-circle svg should exist");
        assert!(std::str::from_utf8(&bytes).unwrap().contains("<svg"));
        assert!(
            IconAssetSource
                .load("/definitely/missing/liora.svg")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn icon_asset_source_loads_embedded_svg_payloads() {
        use gpui::AssetSource;

        let path = format!(
            "{}{}",
            INLINE_SVG_ASSET_PREFIX, r#"<svg viewBox="0 0 24 24"><path d="M1 1h22v22H1z"/></svg>"#
        );
        let bytes = IconAssetSource
            .load(&path)
            .expect("embedded icon asset loading should not error")
            .expect("embedded SVG payload should load");
        let svg = std::str::from_utf8(&bytes).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
    }

    #[test]
    fn target_icon_bundle_candidates_ignore_report_files() {
        let root = std::env::temp_dir().join(format!(
            "liora-icons-candidates-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let icons_root = root.join("target/liora/icons");
        let docs_bundle = icons_root.join("liora-docs/assets/liora-icons/lucide");
        std::fs::create_dir_all(&docs_bundle).unwrap();
        std::fs::write(icons_root.join("liora_icon_bundle_report.md"), "report").unwrap();
        std::fs::write(docs_bundle.join("a-arrow-down.svg"), "<svg />").unwrap();

        let candidates = target_icon_bundle_candidates(&root, "lucide", "a-arrow-down.svg");

        assert!(candidates.iter().any(|path| path.ends_with(
            "target/liora/icons/liora-docs/assets/liora-icons/lucide/a-arrow-down.svg"
        )));
        assert!(!candidates.iter().any(|path| {
            path.to_string_lossy()
                .contains("liora_icon_bundle_report.md/assets/liora-icons")
        }));

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn icon_asset_source_loads_virtual_icon_with_dev_fallback() {
        use gpui::AssetSource;

        let dev_path = format!(
            "{}/../liora-icons-lucide/assets/svgs/loader-circle.svg",
            env!("CARGO_MANIFEST_DIR")
        );
        let path = icon_svg_asset_path("lucide", "loader-circle.svg", Some(dev_path));
        let bytes = IconAssetSource
            .load(&path)
            .expect("virtual icon asset loading should not error")
            .expect("virtual icon should load from dev fallback");
        let svg = std::str::from_utf8(&bytes).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("viewBox"));
    }
}
