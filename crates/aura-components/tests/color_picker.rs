use aura_components::ColorPicker;

#[test]
fn normalizes_hex_colors() {
    assert_eq!(
        ColorPicker::normalize_hex("#409EFF"),
        Some("#409EFF".into())
    );
    assert_eq!(ColorPicker::normalize_hex("409eff"), Some("#409EFF".into()));
    assert_eq!(ColorPicker::normalize_hex("#abc"), Some("#AABBCC".into()));
}

#[test]
fn rejects_invalid_hex_colors() {
    assert_eq!(ColorPicker::normalize_hex("#12"), None);
    assert_eq!(ColorPicker::normalize_hex("#xyzxyz"), None);
}

#[test]
fn converts_hex_to_rgb_channels() {
    assert_eq!(ColorPicker::hex_rgb("#409EFF"), Some((64, 158, 255)));
    assert_eq!(ColorPicker::hex_rgb("#AABBCC"), Some((170, 187, 204)));
}
