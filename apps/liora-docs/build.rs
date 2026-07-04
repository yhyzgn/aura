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
    let dest = out_dir.join("embedded_icon_bundle.rs");
    let mut embedded_icons = BTreeMap::new();
    for icon in &report.copied {
        embedded_icons
            .entry(format!("{}/{}", icon.set, icon.file))
            .or_insert_with(|| icon.path.clone());
    }

    let mut out = String::new();
    out.push_str("pub fn load(path: &str) -> Option<&'static [u8]> {\n");
    out.push_str("    match path {\n");
    for (key, path) in embedded_icons {
        let path = path.to_string_lossy().replace('\\', "\\\\");
        out.push_str(&format!(
            "        {key:?} => Some(include_bytes!({path:?}).as_slice()),\n"
        ));
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    fs::write(dest, out).expect("embedded icon bundle should be written");
}
