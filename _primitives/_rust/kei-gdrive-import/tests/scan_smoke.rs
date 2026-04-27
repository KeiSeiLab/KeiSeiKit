use std::path::PathBuf;

use kei_gdrive_import::scan_tree;

fn fixtures_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("fixtures");
    p
}

#[test]
fn scan_tree_emits_at_least_six_classifications() {
    let entries = scan_tree(&fixtures_root()).expect("scan_tree");
    assert!(
        entries.len() >= 6,
        "expected at least 6 fixture dirs, got {}: {:?}",
        entries.len(),
        entries.iter().map(|c| &c.path).collect::<Vec<_>>()
    );
}

#[test]
fn scan_tree_output_round_trips_through_serde_json() {
    let entries = scan_tree(&fixtures_root()).expect("scan_tree");
    let s = serde_json::to_string(&entries).expect("serialize");
    let v: serde_json::Value = serde_json::from_str(&s).expect("re-parse");
    assert!(v.is_array());
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), entries.len());
}
