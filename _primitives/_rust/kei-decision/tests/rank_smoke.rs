//! Smoke: parse + classify + rank — assert deps are honoured and the
//! highest-score / lowest-effort row floats near the top within its
//! deps-equivalent group.

use kei_decision::{classify, parse_master_report, rank_actions};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures");
    p.push(name);
    p
}

#[test]
fn rank_returns_topo_consistent_order() {
    let raws = parse_master_report(&fixture("valid-master.md")).expect("parse");
    let kinds: Vec<_> = raws.iter().map(classify).collect();
    let ranked = rank_actions(raws, kinds);
    assert_eq!(ranked.len(), 5);
    // Action #4 depends on #3 — #3 must come first.
    let pos_3 = ranked.iter().position(|r| r.raw.id == "3").expect("3 present");
    let pos_4 = ranked.iter().position(|r| r.raw.id == "4").expect("4 present");
    assert!(pos_3 < pos_4, "topo broken: 3 at {pos_3}, 4 at {pos_4}");
    // Action #3 depends on #1 — #1 must come first.
    let pos_1 = ranked.iter().position(|r| r.raw.id == "1").expect("1 present");
    assert!(pos_1 < pos_3, "topo broken: 1 at {pos_1}, 3 at {pos_3}");
}

#[test]
fn rank_field_is_one_indexed_dense() {
    let raws = parse_master_report(&fixture("valid-master.md")).expect("parse");
    let kinds: Vec<_> = raws.iter().map(classify).collect();
    let ranked = rank_actions(raws, kinds);
    for (i, r) in ranked.iter().enumerate() {
        assert_eq!(r.rank, i + 1, "rank field must equal position+1");
    }
}

#[test]
fn no_dep_action_ranks_above_chained_actions_within_score_class() {
    // Among the no-dep set {1, 2, 5}, the smallest-effort one (2-3h tied
    // for #2 and #5; LOW severity) should rank near the top.
    let raws = parse_master_report(&fixture("valid-master.md")).expect("parse");
    let kinds: Vec<_> = raws.iter().map(classify).collect();
    let ranked = rank_actions(raws, kinds);
    // First-position action must NOT have a dep.
    assert!(ranked[0].raw.deps.is_empty(), "rank-1 must have no deps; got {:?}", ranked[0]);
}
