//! Classifier sanity — keyword → ActionKind.

use kei_decision::{classify, ActionKind, RawAction};

fn raw(title: &str) -> RawAction {
    RawAction {
        id: "1".to_string(),
        title: title.to_string(),
        severity: "MEDIUM".to_string(),
        effort: "1h".to_string(),
        deps: vec![],
        source_line: 1,
    }
}

#[test]
fn refactor_keyword_matches_refactor() {
    assert_eq!(classify(&raw("Refactor 4 hooks to call kei-leak-matrix")), ActionKind::Refactor);
}

#[test]
fn migrate_keyword_matches_migrate() {
    assert_eq!(classify(&raw("Migrate /research to kei-fork + kei-ledger")), ActionKind::Migrate);
}

#[test]
fn new_primitive_keyword_matches_new_primitive() {
    assert_eq!(classify(&raw("Build new primitive vault-ingester ~80 LOC")), ActionKind::NewPrimitive);
    assert_eq!(classify(&raw("Add new crate kei-foo")), ActionKind::NewPrimitive);
}

#[test]
fn decompose_keyword_matches_decompose() {
    assert_eq!(classify(&raw("Decompose new-agent 486-LOC monolith into 8 phase files")), ActionKind::Decompose);
}

#[test]
fn doc_keyword_matches_doc() {
    assert_eq!(classify(&raw("Document wikilink translator conversions")), ActionKind::Doc);
}

#[test]
fn ambiguous_falls_through_to_unknown() {
    assert_eq!(classify(&raw("Talk to user about plan")), ActionKind::Unknown);
}

#[test]
fn slug_strings_are_stable() {
    assert_eq!(ActionKind::Refactor.slug(), "refactor");
    assert_eq!(ActionKind::NewPrimitive.slug(), "new-primitive");
    assert_eq!(ActionKind::Doc.slug(), "doc");
}
