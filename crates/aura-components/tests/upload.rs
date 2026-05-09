use aura_components::{Upload, UploadFile};

#[test]
fn detects_when_upload_can_accept_more_files() {
    assert!(Upload::can_accept_more_len(0, Some(1), false));
    assert!(!Upload::can_accept_more_len(1, Some(1), false));
    assert!(!Upload::can_accept_more_len(0, None, true));
}

#[test]
fn clamps_upload_progress_to_percent() {
    let file = UploadFile::new("large", "large.bin").progress(142);

    assert_eq!(file.progress, 100);
}
