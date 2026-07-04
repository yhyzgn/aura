#[path = "../../crates/liora-core/src/locales_codegen.rs"]
mod locales_codegen;

fn main() {
    let icon_report = liora_icons_optimizer::Optimizer::new()
        .scan_dir("src")
        .scan_dir("content/snippets")
        .bundle_auto()
        .try_run()
        .expect("Liora Docs icon optimization should complete");
    write_embedded_icon_bundle(&icon_report);

    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../packaging/icons/liora.ico");
        res.set("FileDescription", "Liora Docs");
        res.set("ProductName", "Liora");
        res.set("CompanyName", "Liora Contributors");
        res.set("OriginalFilename", "liora-docs.exe");
        let _ = res.compile();
    }

    locales_codegen::generate_locales_from_package("liora_core::Locales");
}

fn write_embedded_icon_bundle(report: &liora_icons_optimizer::OptimizationReport) {
    use std::{collections::BTreeMap, env, fs, path::PathBuf};

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR should be set"));
    let source_path = out_dir.join("embedded_icon_bundle.rs");
    let bundle_path = out_dir.join("embedded_icon_bundle.bin");
    let mut embedded_icons = BTreeMap::new();
    for icon in &report.copied {
        embedded_icons
            .entry(format!("{}/{}", icon.set, icon.file))
            .or_insert_with(|| icon.path.clone());
    }

    let mut bundle = Vec::new();
    let mut index = Vec::new();
    for (key, path) in embedded_icons {
        let start = bundle.len();
        let bytes = fs::read(&path).expect("optimized icon SVG should be readable");
        bundle.extend_from_slice(&bytes);
        index.push((key, start, bytes.len()));
    }
    fs::write(&bundle_path, bundle).expect("embedded icon bundle bytes should be written");

    let bundle_path = bundle_path.to_string_lossy().replace('\\', "\\\\");
    let mut out = String::new();
    out.push_str(&format!(
        "const BUNDLE: &[u8] = include_bytes!({bundle_path:?});\n"
    ));
    out.push_str("const INDEX: &[(&str, usize, usize)] = &[\n");
    for (key, start, len) in index {
        out.push_str(&format!("    ({key:?}, {start}, {len}),\n"));
    }
    out.push_str("];\n");
    out.push_str("pub fn load(path: &str) -> Option<&'static [u8]> {\n");
    out.push_str("    let index = INDEX.binary_search_by(|(key, _, _)| key.cmp(&path)).ok()?;\n");
    out.push_str("    let (_, start, len) = INDEX[index];\n");
    out.push_str("    Some(&BUNDLE[start..start + len])\n");
    out.push_str("}\n");
    fs::write(source_path, out).expect("embedded icon bundle source should be written");
}
