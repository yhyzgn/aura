use std::{fmt, path::PathBuf};

use crate::known_apps;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    MissingPath { label: String, path: PathBuf },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPath { label, path } => {
                write!(f, "missing {label}: {}", path.display())
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationReport {
    pub errors: Vec<ValidationError>,
}

impl ValidationReport {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    fn require_path(&mut self, label: impl Into<String>, path: PathBuf) {
        if !path.exists() {
            self.errors.push(ValidationError::MissingPath {
                label: label.into(),
                path,
            });
        }
    }
}

pub fn validate_packaging_layout(root: impl Into<PathBuf>) -> ValidationReport {
    let root = root.into();
    let mut report = ValidationReport::default();

    report.require_path("packaging directory", root.join("packaging"));
    report.require_path("packaging icons directory", root.join("packaging/icons"));
    report.require_path("packaging linux directory", root.join("packaging/linux"));
    report.require_path("packaging macos directory", root.join("packaging/macos"));
    report.require_path(
        "packaging windows directory",
        root.join("packaging/windows"),
    );

    for app in known_apps() {
        let metadata = app.metadata();
        report.require_path(
            format!("{} packager config", metadata.binary),
            metadata.packager_config_path(&root),
        );
        report.require_path(
            format!("{} linux desktop entry", metadata.binary),
            metadata.linux_desktop_path(&root),
        );
        report.require_path(
            format!("{} linux metainfo", metadata.binary),
            metadata.linux_metainfo_path(&root),
        );
        report.require_path(
            format!("{} png icon", metadata.binary),
            metadata.icon_png_path(&root),
        );
        report.require_path(
            format!("{} icns icon", metadata.binary),
            metadata.icon_icns_path(&root),
        );
        report.require_path(
            format!("{} ico icon", metadata.binary),
            metadata.icon_ico_path(&root),
        );
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_layout_reports_missing_paths() {
        let report = validate_packaging_layout("target/definitely-missing-aura-packaging-layout");
        assert!(!report.is_ok());
        assert!(
            report
                .errors
                .iter()
                .any(|error| error.to_string().contains("packaging directory"))
        );
    }
}
