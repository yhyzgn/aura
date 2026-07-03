#[path = "../../build-support/locales_codegen.rs"]
mod locales_codegen;

fn main() {
    locales_codegen::generate_locales_from_package("crate::locales::Locales");
}
