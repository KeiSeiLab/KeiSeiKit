//! Integration tests for kei-diff.
//!
//! Core property: `apply(old, diff(old, new)) == new` for every fixture.
//! Plus edge cases on pointer escaping, array edits, apply errors, and
//! the RFC 6902 wire format.

use kei_diff::{apply, diff, ApplyError, Op, Patch};
use serde_json::{json, Value};

fn rt(old: Value, new: Value) {
    let patch = diff(&old, &new);
    let applied = apply(&old, &patch).expect("apply failed");
    assert_eq!(
        applied, new,
        "round-trip failed\n  old  = {old}\n  new  = {new}\n  patch = {}",
        serde_json::to_string(&patch).unwrap()
    );
}

#[test]
fn equal_values_produce_empty_patch() {
    let patch = diff(&json!({"a": 1}), &json!({"a": 1}));
    assert!(patch.is_empty());
    rt(json!({"a": 1}), json!({"a": 1}));
}

#[test]
fn scalar_replace() {
    rt(json!(1), json!(2));
    rt(json!("a"), json!("b"));
    rt(json!(true), json!(false));
}

#[test]
fn type_change_emits_replace() {
    let old = json!("hello");
    let new = json!(42);
    let patch = diff(&old, &new);
    assert_eq!(patch.len(), 1);
    assert!(matches!(patch.0[0], Op::Replace { .. }));
    rt(old, new);
}

#[test]
fn object_add_key() {
    let old = json!({"a": 1});
    let new = json!({"a": 1, "b": 2});
    let patch = diff(&old, &new);
    assert_eq!(patch.len(), 1);
    assert!(matches!(&patch.0[0], Op::Add { path, .. } if path == "/b"));
    rt(old, new);
}

#[test]
fn object_remove_key() {
    let old = json!({"a": 1, "b": 2});
    let new = json!({"a": 1});
    let patch = diff(&old, &new);
    assert_eq!(patch.len(), 1);
    assert!(matches!(&patch.0[0], Op::Remove { path } if path == "/b"));
    rt(old, new);
}

#[test]
fn object_replace_value() {
    let old = json!({"a": 1});
    let new = json!({"a": 2});
    let patch = diff(&old, &new);
    assert_eq!(patch.len(), 1);
    assert!(matches!(&patch.0[0], Op::Replace { path, .. } if path == "/a"));
    rt(old, new);
}

#[test]
fn nested_object_replace() {
    let old = json!({"a": {"b": {"c": 1}}});
    let new = json!({"a": {"b": {"c": 2}}});
    let patch = diff(&old, &new);
    assert_eq!(patch.len(), 1);
    assert!(matches!(&patch.0[0], Op::Replace { path, .. } if path == "/a/b/c"));
    rt(old, new);
}

#[test]
fn array_append() {
    let old = json!([1, 2]);
    let new = json!([1, 2, 3, 4]);
    let patch = diff(&old, &new);
    assert_eq!(patch.len(), 2);
    assert!(matches!(&patch.0[0], Op::Add { path, .. } if path == "/2"));
    assert!(matches!(&patch.0[1], Op::Add { path, .. } if path == "/3"));
    rt(old, new);
}

#[test]
fn array_truncate() {
    let old = json!([1, 2, 3, 4]);
    let new = json!([1, 2]);
    let patch = diff(&old, &new);
    // Expect removals highest-first (/3 then /2) so indices stay valid.
    assert_eq!(patch.len(), 2);
    assert!(matches!(&patch.0[0], Op::Remove { path } if path == "/3"));
    assert!(matches!(&patch.0[1], Op::Remove { path } if path == "/2"));
    rt(old, new);
}

#[test]
fn array_element_replace() {
    let old = json!([1, 2, 3]);
    let new = json!([1, 99, 3]);
    let patch = diff(&old, &new);
    assert_eq!(patch.len(), 1);
    assert!(matches!(&patch.0[0], Op::Replace { path, .. } if path == "/1"));
    rt(old, new);
}

#[test]
fn nested_array_inside_object() {
    let old = json!({"xs": [1, 2, 3], "y": "z"});
    let new = json!({"xs": [1, 7, 3, 4], "y": "z"});
    rt(old, new);
}

#[test]
fn deeply_nested_mixed() {
    let old = json!({
        "meta": {"ts": 100, "tags": ["a", "b"]},
        "items": [{"id": 1}, {"id": 2}],
    });
    let new = json!({
        "meta": {"ts": 200, "tags": ["a", "b", "c"]},
        "items": [{"id": 1}, {"id": 3}, {"id": 4}],
        "extra": true,
    });
    rt(old, new);
}

#[test]
fn pointer_escapes_slash_and_tilde() {
    let old = json!({"a/b": 1, "c~d": 2});
    let new = json!({"a/b": 9, "c~d": 2});
    let patch = diff(&old, &new);
    assert_eq!(patch.len(), 1);
    let expected_path = "/a~1b";
    match &patch.0[0] {
        Op::Replace { path, .. } => assert_eq!(path, expected_path),
        other => panic!("expected Replace, got {other:?}"),
    }
    rt(old, new);

    let old2 = json!({"c~d": 1});
    let new2 = json!({"c~d": 2});
    let p2 = diff(&old2, &new2);
    match &p2.0[0] {
        Op::Replace { path, .. } => assert_eq!(path, "/c~0d"),
        other => panic!("expected Replace, got {other:?}"),
    }
    rt(old2, new2);
}

#[test]
fn apply_missing_path_errors() {
    let doc = json!({"a": 1});
    let patch = Patch(vec![Op::Replace {
        path: "/nope".into(),
        value: json!(9),
    }]);
    let err = apply(&doc, &patch).unwrap_err();
    assert!(matches!(err, ApplyError::MissingTarget(_)));
}

#[test]
fn apply_remove_missing_errors() {
    let doc = json!({"a": 1});
    let patch = Patch(vec![Op::Remove { path: "/ghost".into() }]);
    assert!(matches!(
        apply(&doc, &patch).unwrap_err(),
        ApplyError::MissingTarget(_)
    ));
}

#[test]
fn apply_replace_root() {
    let doc = json!({"a": 1});
    let patch = Patch(vec![Op::Replace {
        path: "".into(),
        value: json!([1, 2, 3]),
    }]);
    let out = apply(&doc, &patch).unwrap();
    assert_eq!(out, json!([1, 2, 3]));
}

#[test]
fn empty_patch_is_identity() {
    let doc = json!({"a": [1, 2, {"b": true}]});
    let out = apply(&doc, &Patch::new()).unwrap();
    assert_eq!(out, doc);
}

#[test]
fn apply_add_on_root_errors() {
    let doc = json!({"a": 1});
    let patch = Patch(vec![Op::Add {
        path: "".into(),
        value: json!(2),
    }]);
    assert!(matches!(
        apply(&doc, &patch).unwrap_err(),
        ApplyError::CannotAddToRoot
    ));
}

#[test]
fn apply_remove_root_errors() {
    let doc = json!([1, 2]);
    let patch = Patch(vec![Op::Remove { path: "".into() }]);
    assert!(matches!(
        apply(&doc, &patch).unwrap_err(),
        ApplyError::CannotRemoveRoot
    ));
}

#[test]
fn wire_format_matches_rfc_6902() {
    let patch = Patch(vec![
        Op::Add {
            path: "/x".into(),
            value: json!(1),
        },
        Op::Remove { path: "/y".into() },
        Op::Replace {
            path: "/z".into(),
            value: json!("hi"),
        },
    ]);
    let txt = serde_json::to_string(&patch).unwrap();
    let parsed: Value = serde_json::from_str(&txt).unwrap();
    assert_eq!(
        parsed,
        json!([
            {"op": "add", "path": "/x", "value": 1},
            {"op": "remove", "path": "/y"},
            {"op": "replace", "path": "/z", "value": "hi"},
        ])
    );
}

#[test]
fn patch_roundtrip_through_serde() {
    let p1 = Patch(vec![
        Op::Add {
            path: "/a".into(),
            value: json!({"nested": [1, 2]}),
        },
        Op::Remove { path: "/b/0".into() },
    ]);
    let txt = serde_json::to_string(&p1).unwrap();
    let p2: Patch = serde_json::from_str(&txt).unwrap();
    assert_eq!(p1, p2);
}

#[test]
fn array_of_objects_element_replace() {
    let old = json!([{"id": 1, "v": "a"}, {"id": 2, "v": "b"}]);
    let new = json!([{"id": 1, "v": "a"}, {"id": 2, "v": "z"}]);
    let patch = diff(&old, &new);
    // Should be a single deep replace at /1/v
    assert_eq!(patch.len(), 1);
    assert!(matches!(&patch.0[0], Op::Replace { path, .. } if path == "/1/v"));
    rt(old, new);
}

#[test]
fn null_to_value_is_replace() {
    rt(json!(null), json!({"x": 1}));
    rt(json!({"x": null}), json!({"x": 1}));
    rt(json!({"x": 1}), json!({"x": null}));
}
