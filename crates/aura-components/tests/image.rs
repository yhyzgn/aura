use aura_components::{Image, ImageFit};

#[test]
fn image_defaults_to_contain_fit() {
    let image = Image::new("https://example.com/image.png");
    assert_eq!(image.fit_kind(), ImageFit::Contain);
}

#[test]
fn image_builder_tracks_dimensions_and_fit() {
    let image = Image::new("https://example.com/image.png")
        .square(gpui::px(88.0))
        .cover();

    assert_eq!(image.fit_kind(), ImageFit::Cover);
    assert_eq!(
        image.dimensions(),
        (Some(gpui::px(88.0)), Some(gpui::px(88.0)))
    );
}

#[test]
fn image_empty_has_no_dimensions_until_configured() {
    let image = Image::empty().fill();
    assert_eq!(image.fit_kind(), ImageFit::Fill);
    assert_eq!(image.dimensions(), (None, None));
}

#[test]
fn image_supports_local_file_sources() {
    let image = Image::local("apps/aura-gallery/assets/local.jpeg");
    assert!(image.source().is_some_and(|source| source.is_file()));
}

#[test]
fn image_supports_remote_url_sources() {
    let image = Image::new("https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg");
    assert!(image.source().is_some_and(|source| source.is_url()));
}
