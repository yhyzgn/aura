use std::{fmt::Write as _, path::PathBuf};

use crate::{Checksum, PackageFormat, Platform};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageArtifact {
    pub app: String,
    pub version: String,
    pub platform: Platform,
    pub format: PackageFormat,
    pub path: PathBuf,
    pub checksum: Checksum,
    pub signed: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageManifest {
    pub artifacts: Vec<PackageArtifact>,
}

impl PackageManifest {
    pub fn push(&mut self, artifact: PackageArtifact) {
        self.artifacts.push(artifact);
    }

    pub fn to_json_pretty(&self) -> String {
        let mut out = String::from("{\n  \"artifacts\": [");
        for (idx, artifact) in self.artifacts.iter().enumerate() {
            if idx > 0 {
                out.push(',');
            }
            write!(
                out,
                "\n    {{\n      \"app\": \"{}\",\n      \"version\": \"{}\",\n      \"platform\": \"{}\",\n      \"format\": \"{}\",\n      \"path\": \"{}\",\n      \"checksum\": {{ \"algorithm\": \"{}\", \"hex\": \"{}\" }},\n      \"signed\": {}\n    }}",
                escape(&artifact.app),
                escape(&artifact.version),
                artifact.platform.as_str(),
                artifact.format.as_str(),
                escape(&artifact.path.display().to_string()),
                artifact.checksum.algorithm,
                artifact.checksum.hex,
                artifact.signed
            )
            .expect("write to string");
        }
        out.push_str("\n  ]\n}\n");
        out
    }
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_serializes_artifact_fields() {
        let mut manifest = PackageManifest::default();
        manifest.push(PackageArtifact {
            app: "aura-gallery".into(),
            version: "0.1.0".into(),
            platform: Platform::Linux,
            format: PackageFormat::AppImage,
            path: "target/packages/aura-gallery.AppImage".into(),
            checksum: Checksum {
                algorithm: "sha256",
                hex: "abc".into(),
            },
            signed: false,
        });
        let json = manifest.to_json_pretty();
        assert!(json.contains("\"app\": \"aura-gallery\""));
        assert!(json.contains("\"format\": \"appimage\""));
        assert!(json.contains("\"signed\": false"));
    }
}
