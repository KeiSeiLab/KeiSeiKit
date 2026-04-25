use kei_changelog::{parse_subject, CommitKind};

#[test]
fn feat_with_scope() {
    let (kind, scope, subj, breaking) = parse_subject("feat(blocks): 5 documentation blocks");
    assert_eq!(kind, CommitKind::Feat);
    assert_eq!(scope.as_deref(), Some("blocks"));
    assert_eq!(subj, "5 documentation blocks");
    assert!(!breaking);
}

#[test]
fn fix_no_scope() {
    let (kind, scope, _, breaking) = parse_subject("fix: off-by-one in walker");
    assert_eq!(kind, CommitKind::Fix);
    assert!(scope.is_none());
    assert!(!breaking);
}

#[test]
fn breaking_bang() {
    let (kind, _, _, breaking) = parse_subject("feat(api)!: rename endpoint");
    assert_eq!(kind, CommitKind::Feat);
    assert!(breaking);
}

#[test]
fn unknown_kind_falls_to_other() {
    let (kind, _, _, _) = parse_subject("nonsense: whatever");
    match kind {
        CommitKind::Other(raw) => assert_eq!(raw, "nonsense"),
        other => panic!("expected Other, got {other:?}"),
    }
}

#[test]
fn non_conventional_subject() {
    let (kind, scope, subj, _) = parse_subject("just a plain message");
    match kind {
        CommitKind::Other(raw) => assert_eq!(raw, "_"),
        other => panic!("expected Other('_'), got {other:?}"),
    }
    assert!(scope.is_none());
    assert_eq!(subj, "just a plain message");
}

#[test]
fn checkpoint_kind() {
    let (kind, _, _, _) = parse_subject("checkpoint: before big refactor");
    assert_eq!(kind, CommitKind::Checkpoint);
}
