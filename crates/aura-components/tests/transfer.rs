use aura_components::{Transfer, TransferItem};

fn fixture() -> Vec<TransferItem> {
    vec![
        TransferItem::new("a", "Alpha"),
        TransferItem::new("b", "Beta").disabled(true),
        TransferItem::new("c", "Gamma"),
    ]
}

#[test]
fn moves_only_enabled_checked_source_items_to_target() {
    let mut target = vec!["c".into()];
    let mut checked = vec!["a".into(), "b".into()];

    let moved = Transfer::move_to_target(&fixture(), &mut target, &mut checked);

    assert_eq!(moved, vec!["a"]);
    assert_eq!(target, vec!["c", "a"]);
    assert!(checked.is_empty());
}

#[test]
fn moves_only_enabled_checked_target_items_to_source() {
    let mut target = vec!["a".into(), "b".into(), "c".into()];
    let mut checked = vec!["b".into(), "c".into()];

    let moved = Transfer::move_to_source(&fixture(), &mut target, &mut checked);

    assert_eq!(moved, vec!["c"]);
    assert_eq!(target, vec!["a", "b"]);
    assert!(checked.is_empty());
}

#[test]
fn filters_items_by_label_or_key() {
    let result = Transfer::filter_items(&fixture(), "alp");
    assert_eq!(result, vec!["a"]);

    let result = Transfer::filter_items(&fixture(), "c");
    assert_eq!(result, vec!["c"]);
}
