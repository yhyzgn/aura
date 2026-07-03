//! Build-time icon bundle optimizer for Liora applications.
//!
//! Add this crate as a build dependency and call the builder from the host
//! application's build script:
//!
//! ```ignore
//! fn main() {
//!     liora_icons_optimizer::Optimizer::new()
//!         .bundle_auto()
//!         .run();
//! }
//! ```
//!
//! The optimizer scans the host crate plus local Liora dependency sources for
//! strongly typed `IconName::...` usages, then copies only those SVG files into
//! `target/liora/icons/assets/liora-icons` for packaging.

use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};
use thiserror::Error;

const ICON_SETS: &[IconSet] = &[
    IconSet::new("lucide", "liora-icons-lucide", "icons_lucide"),
    IconSet::new("antd", "liora-icons-antd", "icons_antd"),
    IconSet::new("ionic", "liora-icons-ionic", "icons_ionic"),
    IconSet::new("tabler", "liora-icons-tabler", "icons_tabler"),
    IconSet::new("carbon", "liora-icons-carbon", "icons_carbon"),
    IconSet::new("material", "liora-icons-material", "icons_material"),
];

#[derive(Debug, Clone, Copy)]
struct IconSet {
    id: &'static str,
    package: &'static str,
    facade_module: &'static str,
}

impl IconSet {
    const fn new(id: &'static str, package: &'static str, facade_module: &'static str) -> Self {
        Self {
            id,
            package,
            facade_module,
        }
    }
}

/// Controls build-time icon resource optimization for the host application.
#[derive(Debug, Clone)]
pub struct Optimizer {
    manifest_dir: PathBuf,
    workspace_root: Option<PathBuf>,
    scan_dirs: Vec<PathBuf>,
    asset_out_dir: Option<PathBuf>,
    report_file: Option<PathBuf>,
    enabled: bool,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Optimizer {
    /// Creates an optimizer using Cargo build-script environment defaults.
    pub fn new() -> Self {
        let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        Self {
            manifest_dir,
            workspace_root: None,
            scan_dirs: Vec::new(),
            asset_out_dir: None,
            report_file: None,
            enabled: false,
        }
    }

    /// Overrides the host package directory. Mostly useful for tests.
    pub fn manifest_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.manifest_dir = path.into();
        self
    }

    /// Overrides the workspace root used for target-relative outputs.
    pub fn workspace_root(mut self, path: impl Into<PathBuf>) -> Self {
        self.workspace_root = Some(path.into());
        self
    }

    /// Adds a source directory to scan. Relative paths are resolved against the
    /// host package directory.
    pub fn scan_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.scan_dirs.push(path.into());
        self
    }

    /// Overrides the copied SVG output directory.
    pub fn asset_out_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.asset_out_dir = Some(path.into());
        self
    }

    /// Overrides the markdown report path.
    pub fn report_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.report_file = Some(path.into());
        self
    }

    /// Enables automatic bundle generation.
    pub fn bundle_auto(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// Runs the optimizer and exits the build script with a Cargo-friendly error
    /// if optimization fails.
    pub fn run(self) {
        if let Err(error) = self.try_run() {
            println!("cargo:error=liora icon bundle optimization failed: {error}");
            std::process::exit(1);
        }
    }

    /// Runs the optimizer and returns detailed errors to the caller.
    pub fn try_run(self) -> Result<OptimizationReport, OptimizerError> {
        if !self.enabled {
            return Ok(OptimizationReport::default());
        }

        let metadata = CargoMetadata::load(&self.manifest_dir)?;
        let workspace_root = self
            .workspace_root
            .clone()
            .or_else(|| metadata.workspace_root.clone())
            .unwrap_or_else(|| self.manifest_dir.clone());

        let package_name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "app".to_string());
        let asset_out_dir = self.asset_out_dir.clone().unwrap_or_else(|| {
            workspace_root
                .join("target")
                .join("liora")
                .join("icons")
                .join(&package_name)
                .join("assets")
                .join("liora-icons")
        });
        let report_file = self.report_file.clone().unwrap_or_else(|| {
            workspace_root
                .join("target")
                .join("liora")
                .join("icons")
                .join("liora_icon_bundle_report.md")
        });

        let icon_packages = metadata.icon_packages();
        let mut icon_catalogs = HashMap::new();
        for set in ICON_SETS {
            if let Some(package_dir) = icon_packages.get(set.package) {
                let svg_dir = package_dir.join("assets").join("svgs");
                icon_catalogs.insert(set.id, IconCatalog::load(set, svg_dir)?);
            }
        }

        let mut source_dirs = self.resolved_scan_dirs();
        source_dirs.extend(metadata.liora_source_dirs(&self.manifest_dir));
        source_dirs.sort();
        source_dirs.dedup();

        for dir in &source_dirs {
            println!("cargo:rerun-if-changed={}", dir.display());
        }

        let scanner = SourceScanner::new();
        let mut used = BTreeSet::new();
        let mut scanned_files = 0usize;
        for dir in &source_dirs {
            scanned_files += scanner.scan_dir(dir, &mut used)?;
        }

        let mut copied = Vec::new();
        if asset_out_dir.exists() {
            fs::remove_dir_all(&asset_out_dir).map_err(|source| OptimizerError::Io {
                path: asset_out_dir.clone(),
                source,
            })?;
        }
        fs::create_dir_all(&asset_out_dir).map_err(|source| OptimizerError::Io {
            path: asset_out_dir.clone(),
            source,
        })?;

        let mut missing = Vec::new();
        for icon in &used {
            let Some(catalog) = icon_catalogs.get(icon.set.as_str()) else {
                missing.push(format!(
                    "{}.{} (icon set package not found)",
                    icon.set, icon.variant
                ));
                continue;
            };
            if icon.variant == "*" {
                for (variant, source_svg) in &catalog.variant_to_file {
                    copy_icon_svg(
                        &asset_out_dir,
                        catalog,
                        &icon.set,
                        variant,
                        source_svg,
                        &mut copied,
                    )?;
                }
                continue;
            }
            let Some(source_svg) = catalog.variant_to_file.get(icon.variant.as_str()) else {
                missing.push(format!("{}.{} (variant not found)", icon.set, icon.variant));
                continue;
            };
            copy_icon_svg(
                &asset_out_dir,
                catalog,
                &icon.set,
                &icon.variant,
                source_svg,
                &mut copied,
            )?;
        }

        let report = OptimizationReport {
            scanned_files,
            source_dirs,
            asset_out_dir,
            copied,
            missing,
        };
        write_report(&report_file, &report)?;
        println!("cargo:rerun-if-changed={}", report_file.display());
        Ok(report)
    }

    fn resolved_scan_dirs(&self) -> Vec<PathBuf> {
        let dirs = if self.scan_dirs.is_empty() {
            vec![PathBuf::from("src")]
        } else {
            self.scan_dirs.clone()
        };
        dirs.into_iter()
            .map(|path| {
                if path.is_absolute() {
                    path
                } else {
                    self.manifest_dir.join(path)
                }
            })
            .filter(|path| path.is_dir())
            .collect()
    }
}

#[derive(Debug, Error)]
pub enum OptimizerError {
    #[error("failed to run cargo metadata in {manifest_dir}: {source}")]
    CargoMetadataIo {
        manifest_dir: PathBuf,
        source: io::Error,
    },
    #[error("cargo metadata failed in {manifest_dir}: {stderr}")]
    CargoMetadataFailed {
        manifest_dir: PathBuf,
        stderr: String,
    },
    #[error("failed to parse cargo metadata: {0}")]
    CargoMetadataJson(#[from] serde_json::Error),
    #[error("filesystem error at {path}: {source}")]
    Io { path: PathBuf, source: io::Error },
}

/// Summary produced by [`Optimizer`].
#[derive(Debug, Clone, Default)]
pub struct OptimizationReport {
    pub scanned_files: usize,
    pub source_dirs: Vec<PathBuf>,
    pub asset_out_dir: PathBuf,
    pub copied: Vec<CopiedIcon>,
    pub missing: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CopiedIcon {
    pub set: String,
    pub variant: String,
    pub file: String,
    pub path: PathBuf,
}

#[derive(Debug)]
struct CargoMetadata {
    workspace_root: Option<PathBuf>,
    packages: Vec<MetadataPackage>,
    reachable_package_ids: BTreeSet<String>,
}

#[derive(Debug)]
struct MetadataPackage {
    id: String,
    name: String,
    manifest_path: PathBuf,
}

impl CargoMetadata {
    fn load(manifest_dir: &Path) -> Result<Self, OptimizerError> {
        let output = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
            .args(["metadata", "--format-version", "1"])
            .current_dir(manifest_dir)
            .output()
            .map_err(|source| OptimizerError::CargoMetadataIo {
                manifest_dir: manifest_dir.to_path_buf(),
                source,
            })?;
        if !output.status.success() {
            return Err(OptimizerError::CargoMetadataFailed {
                manifest_dir: manifest_dir.to_path_buf(),
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            });
        }
        let value: Value = serde_json::from_slice(&output.stdout)?;
        let workspace_root = value
            .get("workspace_root")
            .and_then(Value::as_str)
            .map(PathBuf::from);
        let mut packages = Vec::new();
        if let Some(items) = value.get("packages").and_then(Value::as_array) {
            for item in items {
                let Some(id) = item.get("id").and_then(Value::as_str) else {
                    continue;
                };
                let Some(name) = item.get("name").and_then(Value::as_str) else {
                    continue;
                };
                let Some(manifest_path) = item.get("manifest_path").and_then(Value::as_str) else {
                    continue;
                };
                packages.push(MetadataPackage {
                    id: id.to_string(),
                    name: name.to_string(),
                    manifest_path: PathBuf::from(manifest_path),
                });
            }
        }
        let reachable_package_ids = reachable_package_ids(&value, manifest_dir, &packages);
        Ok(Self {
            workspace_root,
            packages,
            reachable_package_ids,
        })
    }

    fn icon_packages(&self) -> HashMap<String, PathBuf> {
        self.packages
            .iter()
            .filter(|package| ICON_SETS.iter().any(|set| set.package == package.name))
            .filter_map(|package| {
                package
                    .manifest_path
                    .parent()
                    .map(|dir| (package.name.clone(), dir.to_path_buf()))
            })
            .collect()
    }

    fn liora_source_dirs(&self, root_manifest_dir: &Path) -> Vec<PathBuf> {
        self.packages
            .iter()
            .filter(|package| self.reachable_package_ids.contains(&package.id))
            .filter(|package| package.name.starts_with("liora-") || package.name == "liora")
            .filter(|package| {
                package.name != "liora-icons-optimizer"
                    && package.name != "liora-locales-codegen"
                    && package.name != "liora-icons"
                    && !package.name.starts_with("liora-icons-")
            })
            .filter_map(|package| package.manifest_path.parent())
            .filter(|dir| *dir != root_manifest_dir)
            .map(|dir| dir.join("src"))
            .filter(|path| path.is_dir())
            .collect()
    }
}

fn reachable_package_ids(
    metadata: &Value,
    manifest_dir: &Path,
    packages: &[MetadataPackage],
) -> BTreeSet<String> {
    let root_manifest = manifest_dir.join("Cargo.toml");
    let Some(root) = packages
        .iter()
        .find(|package| package.manifest_path == root_manifest)
    else {
        return packages.iter().map(|package| package.id.clone()).collect();
    };
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(nodes) = metadata
        .get("resolve")
        .and_then(|resolve| resolve.get("nodes"))
        .and_then(Value::as_array)
    {
        for node in nodes {
            let Some(id) = node.get("id").and_then(Value::as_str) else {
                continue;
            };
            let deps = node
                .get("dependencies")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>();
            edges.insert(id.to_string(), deps);
        }
    }
    let mut reachable = BTreeSet::new();
    let mut stack = vec![root.id.clone()];
    while let Some(id) = stack.pop() {
        if !reachable.insert(id.clone()) {
            continue;
        }
        if let Some(deps) = edges.get(&id) {
            stack.extend(deps.iter().cloned());
        }
    }
    reachable
}

#[derive(Debug)]
struct IconCatalog {
    svg_dir: PathBuf,
    variant_to_file: BTreeMap<String, String>,
}

impl IconCatalog {
    fn load(set: &IconSet, svg_dir: PathBuf) -> Result<Self, OptimizerError> {
        let mut variant_to_file = BTreeMap::new();
        if svg_dir.is_dir() {
            let entries = fs::read_dir(&svg_dir).map_err(|source| OptimizerError::Io {
                path: svg_dir.clone(),
                source,
            })?;
            for entry in entries {
                let entry = entry.map_err(|source| OptimizerError::Io {
                    path: svg_dir.clone(),
                    source,
                })?;
                let path = entry.path();
                if path.extension().is_some_and(|extension| extension == "svg") {
                    let Some(stem) = path.file_stem().and_then(|value| value.to_str()) else {
                        continue;
                    };
                    let Some(file) = path.file_name().and_then(|value| value.to_str()) else {
                        continue;
                    };
                    variant_to_file.insert(to_pascal_case(stem), file.to_string());
                }
            }
        } else {
            println!(
                "cargo:warning=Liora icon optimizer could not find {} SVG directory: {}",
                set.package,
                svg_dir.display()
            );
        }
        Ok(Self {
            svg_dir,
            variant_to_file,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct UsedIcon {
    set: String,
    variant: String,
}

#[derive(Debug)]
struct SourceScanner;

impl SourceScanner {
    fn new() -> Self {
        Self
    }

    fn scan_dir(&self, dir: &Path, used: &mut BTreeSet<UsedIcon>) -> Result<usize, OptimizerError> {
        if !dir.is_dir() {
            return Ok(0);
        }
        let mut count = 0;
        self.walk_rs_files(dir, &mut |path| {
            count += 1;
            let text = fs::read_to_string(path).map_err(|source| OptimizerError::Io {
                path: path.to_path_buf(),
                source,
            })?;
            self.scan_text(&text, used);
            Ok(())
        })?;
        Ok(count)
    }

    fn walk_rs_files(
        &self,
        dir: &Path,
        visit: &mut impl FnMut(&Path) -> Result<(), OptimizerError>,
    ) -> Result<(), OptimizerError> {
        for entry in fs::read_dir(dir).map_err(|source| OptimizerError::Io {
            path: dir.to_path_buf(),
            source,
        })? {
            let entry = entry.map_err(|source| OptimizerError::Io {
                path: dir.to_path_buf(),
                source,
            })?;
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or_default();
                if matches!(name, "target" | ".git") {
                    continue;
                }
                self.walk_rs_files(&path, visit)?;
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                visit(&path)?;
            }
        }
        Ok(())
    }

    fn scan_text(&self, text: &str, used: &mut BTreeSet<UsedIcon>) {
        for set in ICON_SETS {
            let aliases = find_aliases(text, set);

            for variant in find_qualified_variants(text, set) {
                used.insert(UsedIcon {
                    set: set.id.to_string(),
                    variant,
                });
            }
            if has_qualified_all_request(text, set) {
                used.insert(UsedIcon {
                    set: set.id.to_string(),
                    variant: "*".to_string(),
                });
            }

            for alias in aliases {
                for variant in find_alias_variants(text, &alias) {
                    used.insert(UsedIcon {
                        set: set.id.to_string(),
                        variant,
                    });
                }
                if has_alias_all_request(text, &alias) {
                    used.insert(UsedIcon {
                        set: set.id.to_string(),
                        variant: "*".to_string(),
                    });
                }
            }
        }
    }
}

fn find_aliases(text: &str, set: &IconSet) -> BTreeSet<String> {
    let mut aliases = BTreeSet::new();
    let needles = [
        format!("use {}::IconName", set.package.replace('-', "_")),
        format!("use liora::{}::IconName", set.facade_module),
    ];
    for needle in needles {
        let mut search_start = 0;
        while let Some(pos) = text[search_start..].find(&needle) {
            let start = search_start + pos + needle.len();
            let rest = &text[start..];
            let alias = if let Some(after_as) = rest.strip_prefix(" as ") {
                read_ident(after_as).map(str::to_string)
            } else {
                Some("IconName".to_string())
            };
            if let Some(alias) = alias {
                aliases.insert(alias);
            }
            search_start = start;
        }
    }
    aliases
}

fn find_qualified_variants(text: &str, set: &IconSet) -> BTreeSet<String> {
    let prefixes = [
        format!("{}::IconName::", set.package.replace('-', "_")),
        format!("liora::{}::IconName::", set.facade_module),
    ];
    let mut variants = BTreeSet::new();
    for prefix in prefixes {
        collect_after_prefix(text, &prefix, &mut variants);
    }
    variants
}

fn find_alias_variants(text: &str, alias: &str) -> BTreeSet<String> {
    let mut variants = BTreeSet::new();
    collect_after_prefix(text, &format!("{alias}::"), &mut variants);
    variants
}

fn has_qualified_all_request(text: &str, set: &IconSet) -> bool {
    [
        format!("{}::IconName::all", set.package.replace('-', "_")),
        format!("liora::{}::IconName::all", set.facade_module),
    ]
    .into_iter()
    .any(|prefix| text.contains(&prefix))
}

fn has_alias_all_request(text: &str, alias: &str) -> bool {
    text.contains(&format!("{alias}::all"))
}

fn collect_after_prefix(text: &str, prefix: &str, variants: &mut BTreeSet<String>) {
    let mut search_start = 0;
    while let Some(pos) = text[search_start..].find(prefix) {
        let start = search_start + pos + prefix.len();
        if let Some(variant) = read_ident(&text[start..]) {
            if variant.chars().next().is_some_and(char::is_uppercase) {
                variants.insert(variant.to_string());
            }
        }
        search_start = start;
    }
}

fn read_ident(text: &str) -> Option<&str> {
    let end = text
        .char_indices()
        .take_while(|(_, ch)| ch.is_ascii_alphanumeric() || *ch == '_')
        .map(|(index, ch)| index + ch.len_utf8())
        .last()?;
    Some(&text[..end])
}

fn copy_icon_svg(
    asset_out_dir: &Path,
    catalog: &IconCatalog,
    set: &str,
    variant: &str,
    source_svg: &str,
    copied: &mut Vec<CopiedIcon>,
) -> Result<(), OptimizerError> {
    let source = catalog.svg_dir.join(source_svg);
    let dest_dir = asset_out_dir.join(set);
    fs::create_dir_all(&dest_dir).map_err(|source| OptimizerError::Io {
        path: dest_dir.clone(),
        source,
    })?;
    let dest = dest_dir.join(source_svg);
    fs::copy(&source, &dest).map_err(|source| OptimizerError::Io {
        path: dest.clone(),
        source,
    })?;
    copied.push(CopiedIcon {
        set: set.to_string(),
        variant: variant.to_string(),
        file: source_svg.to_string(),
        path: dest,
    });
    Ok(())
}

fn write_report(path: &Path, report: &OptimizationReport) -> Result<(), OptimizerError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| OptimizerError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    let mut out = String::new();
    out.push_str("# Liora icon bundle optimization report\n\n");
    out.push_str(&format!(
        "- Scanned Rust files: `{}`\n",
        report.scanned_files
    ));
    out.push_str(&format!("- Copied SVG icons: `{}`\n", report.copied.len()));
    out.push_str(&format!(
        "- Output directory: `{}`\n",
        report.asset_out_dir.display()
    ));
    out.push_str("\n## Scan roots\n\n");
    for dir in &report.source_dirs {
        out.push_str(&format!("- `{}`\n", dir.display()));
    }
    out.push_str("\n## Included icons\n\n");
    if report.copied.is_empty() {
        out.push_str("No icons were discovered.\n");
    } else {
        out.push_str("| Set | Variant | File |\n| --- | --- | --- |\n");
        for icon in &report.copied {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` |\n",
                icon.set, icon.variant, icon.file
            ));
        }
    }
    if !report.missing.is_empty() {
        out.push_str("\n## Missing icons\n\n");
        for item in &report.missing {
            out.push_str(&format!("- `{item}`\n"));
        }
    }
    fs::write(path, out).map_err(|source| OptimizerError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn to_pascal_case(value: &str) -> String {
    let mut out = String::new();
    for part in value.split(|ch: char| !ch.is_ascii_alphanumeric()) {
        if part.is_empty() {
            continue;
        }
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    if out.is_empty() {
        out.push_str("Icon");
    }
    if out.as_bytes().first().is_some_and(u8::is_ascii_digit) {
        out.insert(0, 'I');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_finds_qualified_and_aliased_icons() {
        let text = r#"
            use liora_icons_lucide::IconName;
            use liora_icons_antd::IconName as AntdIconName;
            use liora::icons_tabler::IconName as TablerIconName;
            fn demo() {
                Icon::new(IconName::Search);
                Icon::new(AntdIconName::HomeOutlined);
                Icon::new(liora_icons_lucide::IconName::LoaderCircle);
                Icon::new(liora::icons_material::IconName::CheckCircleOutlined);
                Icon::new(TablerIconName::HomeFilled);
                let _ = liora_icons_carbon::IconName::all();
            }
        "#;
        let mut used = BTreeSet::new();
        SourceScanner::new().scan_text(text, &mut used);
        let names = used
            .into_iter()
            .map(|icon| format!("{}.{}", icon.set, icon.variant))
            .collect::<Vec<_>>();
        assert!(names.contains(&"lucide.Search".to_string()));
        assert!(names.contains(&"lucide.LoaderCircle".to_string()));
        assert!(names.contains(&"antd.HomeOutlined".to_string()));
        assert!(names.contains(&"tabler.HomeFilled".to_string()));
        assert!(names.contains(&"material.CheckCircleOutlined".to_string()));
        assert!(names.contains(&"carbon.*".to_string()));
    }

    #[test]
    fn pascal_case_matches_icon_build_scripts() {
        assert_eq!(to_pascal_case("loader-circle"), "LoaderCircle");
        assert_eq!(to_pascal_case("3d-rotation"), "I3dRotation");
        assert_eq!(to_pascal_case("a-i-enabled-e-d-t"), "AIEnabledEDT");
    }
}
