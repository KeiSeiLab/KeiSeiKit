//! Integration tests for kei-dna-index.
//!
//! Each test builds a minimal `agents` table in a tempfile sqlite DB,
//! then opens it read-only via the library and asserts public-API behaviour.

use kei_dna_index::{
    adjacent, cluster_by, open_read_only, precedent, split_dna, stats, AdjacencyKind, ClusterBy,
    Relationship,
};
use rusqlite::{params, Connection};
use tempfile::NamedTempFile;

fn setup() -> NamedTempFile {
    let f = NamedTempFile::new().unwrap();
    let c = Connection::open(f.path()).unwrap();
    c.execute_batch(
        "CREATE TABLE agents (\
             id TEXT PRIMARY KEY, \
             dna TEXT, \
             started_ts INTEGER NOT NULL, \
             status TEXT NOT NULL)",
    )
    .unwrap();
    drop(c);
    f
}

fn insert(path: &std::path::Path, id: &str, dna: Option<&str>, ts: i64, status: &str) {
    let c = Connection::open(path).unwrap();
    c.execute(
        "INSERT INTO agents (id, dna, started_ts, status) VALUES (?1, ?2, ?3, ?4)",
        params![id, dna, ts, status],
    )
    .unwrap();
}

#[test]
fn parse_dna_valid_format() {
    let p = split_dna("edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821::AC73A6A3-e9bf468d").unwrap();
    assert_eq!(p.role, "edit-local");
    assert_eq!(p.scope_sha, "5435F821");
    assert_eq!(p.body_sha, "AC73A6A3");
    assert_eq!(p.nonce, "e9bf468d");
}

#[test]
fn parse_dna_rejects_malformed() {
    assert!(split_dna("nope").is_err());
    assert!(split_dna("a::b::c::d-e").is_err()); // short hex
    assert!(split_dna("a::b::12345678::ZZZZZZZZ-12345678").is_err()); // non-hex
    assert!(split_dna("a::b::12345678::12345678_12345678").is_err()); // no dash
}

#[test]
fn adjacent_same_scope() {
    let f = setup();
    let p = f.path();
    insert(
        p,
        "a1",
        Some("edit::CAPS1::AAAAAAAA::11111111-22222222"),
        100,
        "merged",
    );
    insert(
        p,
        "a2",
        Some("edit::CAPS1::AAAAAAAA::33333333-44444444"),
        200,
        "running",
    );
    insert(
        p,
        "a3",
        Some("edit::CAPS1::BBBBBBBB::55555555-66666666"),
        300,
        "merged",
    );
    let conn = open_read_only(p).unwrap();
    let out = adjacent(
        &conn,
        "edit::CAPS1::AAAAAAAA::11111111-22222222",
        AdjacencyKind::Scope,
        10,
    )
    .unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].agent_id, "a2");
    assert_eq!(out[0].relationship, Relationship::SameScope);
    assert_eq!(out[0].distance, 0);
}

#[test]
fn adjacent_same_body() {
    let f = setup();
    let p = f.path();
    insert(
        p,
        "a1",
        Some("edit::CAPS1::11111111::ABCDEF01-aaaaaaaa"),
        100,
        "merged",
    );
    insert(
        p,
        "a2",
        Some("edit::CAPS2::22222222::ABCDEF01-bbbbbbbb"),
        200,
        "merged",
    );
    insert(
        p,
        "a3",
        Some("edit::CAPS1::11111111::DEADBEEF-cccccccc"),
        300,
        "merged",
    );
    let conn = open_read_only(p).unwrap();
    let out = adjacent(
        &conn,
        "edit::CAPS1::11111111::ABCDEF01-aaaaaaaa",
        AdjacencyKind::Body,
        10,
    )
    .unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].agent_id, "a2");
    assert_eq!(out[0].relationship, Relationship::SameBody);
}

#[test]
fn adjacent_role_caps() {
    let f = setup();
    let p = f.path();
    // Target role=edit caps=ABCDEFGH
    insert(
        p,
        "a1",
        Some("edit::ABCDEFGH::11111111::AAAAAAAA-aaaaaaaa"),
        100,
        "merged",
    );
    // Same role, caps 1-char different → Hamming=1
    insert(
        p,
        "a2",
        Some("edit::ABCDEFGX::22222222::BBBBBBBB-bbbbbbbb"),
        200,
        "merged",
    );
    // Same role, caps 3-char different → Hamming=3
    insert(
        p,
        "a3",
        Some("edit::ZBCZEFGZ::33333333::CCCCCCCC-cccccccc"),
        300,
        "merged",
    );
    // Different role → excluded
    insert(
        p,
        "a4",
        Some("plan::ABCDEFGH::44444444::DDDDDDDD-dddddddd"),
        400,
        "merged",
    );
    let conn = open_read_only(p).unwrap();
    let out = adjacent(
        &conn,
        "edit::ABCDEFGH::11111111::AAAAAAAA-aaaaaaaa",
        AdjacencyKind::Role,
        10,
    )
    .unwrap();
    assert_eq!(out.len(), 2);
    assert_eq!(out[0].agent_id, "a2");
    assert_eq!(out[0].distance, 1);
    assert_eq!(out[1].agent_id, "a3");
    assert_eq!(out[1].distance, 3);
}

#[test]
fn adjacent_temporal() {
    let f = setup();
    let p = f.path();
    insert(
        p,
        "a1",
        Some("edit::C1::11111111::AAAAAAAA-aaaaaaaa"),
        1000,
        "merged",
    );
    insert(
        p,
        "a2",
        Some("edit::C2::22222222::BBBBBBBB-bbbbbbbb"),
        1005,
        "merged",
    );
    insert(
        p,
        "a3",
        Some("edit::C3::33333333::CCCCCCCC-cccccccc"),
        990,
        "merged",
    );
    insert(
        p,
        "a4",
        Some("edit::C4::44444444::DDDDDDDD-dddddddd"),
        1100,
        "merged",
    );
    insert(
        p,
        "a5",
        Some("edit::C5::55555555::EEEEEEEE-eeeeeeee"),
        500,
        "merged",
    );
    let conn = open_read_only(p).unwrap();
    let out = adjacent(
        &conn,
        "edit::C1::11111111::AAAAAAAA-aaaaaaaa",
        AdjacencyKind::Temporal,
        3,
    )
    .unwrap();
    assert_eq!(out.len(), 3);
    assert_eq!(out[0].agent_id, "a2");
    assert_eq!(out[0].distance, 5);
    assert_eq!(out[1].agent_id, "a3");
    assert_eq!(out[1].distance, 10);
    assert_eq!(out[2].agent_id, "a4");
    assert_eq!(out[2].distance, 100);
}

#[test]
fn adjacent_all_kind() {
    let f = setup();
    let p = f.path();
    // Target
    insert(
        p,
        "t0",
        Some("edit::ABCDEFGH::11111111::AAAAAAAA-aaaaaaaa"),
        100,
        "merged",
    );
    // Same scope AND same role/caps (should appear once with dist 0)
    insert(
        p,
        "dup",
        Some("edit::ABCDEFGH::11111111::BBBBBBBB-bbbbbbbb"),
        200,
        "merged",
    );
    // Only temporal neighbor
    insert(
        p,
        "far",
        Some("plan::ZZZZZZZZ::99999999::CCCCCCCC-cccccccc"),
        150,
        "merged",
    );
    let conn = open_read_only(p).unwrap();
    let out = adjacent(
        &conn,
        "edit::ABCDEFGH::11111111::AAAAAAAA-aaaaaaaa",
        AdjacencyKind::All,
        10,
    )
    .unwrap();
    // Two distinct DNAs: "dup" and "far"
    assert_eq!(out.len(), 2);
    let dup = out.iter().find(|r| r.agent_id == "dup").unwrap();
    // Dup should be reported with min distance (0 from scope/body match)
    assert_eq!(dup.distance, 0);
    let far = out.iter().find(|r| r.agent_id == "far").unwrap();
    assert_eq!(far.distance, 50);
}

#[test]
fn cluster_by_scope() {
    let f = setup();
    let p = f.path();
    // scope AAAA×3
    insert(p, "a1", Some("r::c::AAAAAAAA::00000001-11111111"), 1, "m");
    insert(p, "a2", Some("r::c::AAAAAAAA::00000002-22222222"), 2, "m");
    insert(p, "a3", Some("r::c::AAAAAAAA::00000003-33333333"), 3, "m");
    // scope BBBB×2
    insert(p, "b1", Some("r::c::BBBBBBBB::00000004-44444444"), 4, "m");
    insert(p, "b2", Some("r::c::BBBBBBBB::00000005-55555555"), 5, "m");
    // scope CCCC×1 (singleton → filtered)
    insert(p, "c1", Some("r::c::CCCCCCCC::00000006-66666666"), 6, "m");
    let conn = open_read_only(p).unwrap();
    let out = cluster_by(&conn, ClusterBy::Scope).unwrap();
    assert_eq!(out.len(), 2);
    let a = out.iter().find(|c| c.key == "AAAAAAAA").unwrap();
    assert_eq!(a.members.len(), 3);
    let b = out.iter().find(|c| c.key == "BBBBBBBB").unwrap();
    assert_eq!(b.members.len(), 2);
}

#[test]
fn cluster_filters_single_member_groups() {
    let f = setup();
    let p = f.path();
    // All scopes unique → no clusters
    insert(p, "a", Some("r::c::AAAAAAAA::11111111-11111111"), 1, "m");
    insert(p, "b", Some("r::c::BBBBBBBB::22222222-22222222"), 2, "m");
    insert(p, "c", Some("r::c::CCCCCCCC::33333333-33333333"), 3, "m");
    let conn = open_read_only(p).unwrap();
    let out = cluster_by(&conn, ClusterBy::Scope).unwrap();
    assert!(out.is_empty());
}

#[test]
fn precedent_finds_merged_only() {
    let f = setup();
    let p = f.path();
    insert(
        p,
        "a1",
        Some("edit::C::11111111::DEADBEEF-11111111"),
        1,
        "merged",
    );
    insert(
        p,
        "a2",
        Some("plan::C::22222222::DEADBEEF-22222222"),
        2,
        "failed",
    );
    insert(
        p,
        "a3",
        Some("edit::C::33333333::DEADBEEF-33333333"),
        3,
        "merged",
    );
    insert(
        p,
        "a4",
        Some("edit::C::44444444::CAFEBABE-44444444"),
        4,
        "merged",
    );
    let conn = open_read_only(p).unwrap();
    let merged = precedent(&conn, "DEADBEEF", Some("merged")).unwrap();
    assert_eq!(merged.len(), 2);
    assert!(merged.iter().all(|r| r.status == "merged"));
    let all = precedent(&conn, "DEADBEEF", None).unwrap();
    assert_eq!(all.len(), 3);
    let all_explicit = precedent(&conn, "DEADBEEF", Some("all")).unwrap();
    assert_eq!(all_explicit.len(), 3);
}

#[test]
fn stats_aggregates() {
    let f = setup();
    let p = f.path();
    // 4 DNAs, 2 unique scopes, 3 unique bodies, 1 scope-cluster, 1 body-cluster
    insert(p, "a1", Some("r::c::AAAAAAAA::b0d10001-11111111"), 1, "m");
    insert(p, "a2", Some("r::c::AAAAAAAA::b0d10002-22222222"), 2, "m");
    insert(p, "a3", Some("r::c::BBBBBBBB::b0d10001-33333333"), 3, "m");
    insert(p, "a4", Some("r::c::BBBBBBBB::b0d10003-44444444"), 4, "m");
    let conn = open_read_only(p).unwrap();
    let s = stats(&conn).unwrap();
    assert_eq!(s.total_dnas, 4);
    assert_eq!(s.unique_scopes, 2);
    assert_eq!(s.unique_bodies, 3);
    assert_eq!(s.clusters_scope, 2); // AAAA×2 + BBBB×2
    assert_eq!(s.clusters_body, 1); // BODY0001×2
    assert!(s.avg_cluster_size > 1.0);
}

#[test]
fn empty_ledger_returns_empty() {
    let f = setup();
    let p = f.path();
    let conn = open_read_only(p).unwrap();
    let s = stats(&conn).unwrap();
    assert_eq!(s.total_dnas, 0);
    assert_eq!(s.unique_scopes, 0);
    assert_eq!(s.clusters_scope, 0);
    assert_eq!(s.avg_cluster_size, 0.0);
    assert!(cluster_by(&conn, ClusterBy::Scope).unwrap().is_empty());
    assert!(precedent(&conn, "DEADBEEF", None).unwrap().is_empty());
}

#[test]
fn malformed_dna_skipped_silently() {
    let f = setup();
    let p = f.path();
    insert(p, "good", Some("r::c::AAAAAAAA::BBBBBBBB-cccccccc"), 1, "m");
    insert(p, "bad1", Some("totally-wrong"), 2, "m");
    insert(p, "bad2", Some("r::c::short::xx-yy"), 3, "m");
    insert(p, "nullrow", None, 4, "m");
    let conn = open_read_only(p).unwrap();
    let s = stats(&conn).unwrap();
    // Only 1 well-formed DNA survives the loader
    assert_eq!(s.total_dnas, 1);
}
