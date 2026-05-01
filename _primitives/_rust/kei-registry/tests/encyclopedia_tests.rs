//! Tests for the encyclopedia subcommand rendering.
//!
//! Six test cases: mixed-type markdown, JSON roundtrip, --type filter,
//! empty registry, supersede chain rendering, and section-count accuracy.

use kei_registry::encyclopedia::{render_json, render_markdown, to_entries, EncyclopediaEntry};
use kei_registry::{open_db, register, BlockType};
use tempfile::tempdir;

// ── helpers ────────────────────────────────────────────────────────────────

fn seed_db(conn: &rusqlite::Connection) {
    register(conn, BlockType::Primitive, "kei-cache", "/tmp/kei-cache/Cargo.toml", b"body-p1", "").unwrap();
    register(conn, BlockType::Skill, "deploy-kit", "/tmp/skills/deploy-kit/SKILL.md", b"body-s1", "deploy").unwrap();
    register(conn, BlockType::Rule, "code-style-1", "/tmp/rules/code-style.md", b"body-r1", "").unwrap();
    register(conn, BlockType::Hook, "disk-guard", "/tmp/hooks/disk-guard.sh", b"body-h1", "PreToolUse:Bash").unwrap();
    register(conn, BlockType::Atom, "atom-one", "/tmp/atoms/atom-one.md", b"body-a1", "").unwrap();
}

// ── test 1: markdown contains all 5 names and correct counts table ─────────

#[test]
fn markdown_contains_all_names_and_counts() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    seed_db(&conn);

    let active = kei_registry::list(&conn, false, 1000).unwrap();
    let all = kei_registry::list(&conn, true, 1000).unwrap();
    let entries = to_entries(&active);
    let md = render_markdown(&entries, &all);

    assert!(md.contains("kei-cache"), "primitive name present");
    assert!(md.contains("deploy-kit"), "skill name present");
    assert!(md.contains("code-style-1"), "rule name present");
    assert!(md.contains("disk-guard"), "hook name present");
    assert!(md.contains("atom-one"), "atom name present");

    assert!(md.contains("Total blocks: 5"), "total count in header");
    assert!(md.contains("| primitive | 1 |"), "primitive count row");
    assert!(md.contains("| skill | 1 |"), "skill count row");
    assert!(md.contains("| rule | 1 |"), "rule count row");
    assert!(md.contains("| hook | 1 |"), "hook count row");
    assert!(md.contains("| atom | 1 |"), "atom count row");
}

// ── test 2: JSON roundtrip ─────────────────────────────────────────────────

#[test]
fn json_roundtrips_through_serde() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    seed_db(&conn);

    let active = kei_registry::list(&conn, false, 1000).unwrap();
    let entries = to_entries(&active);
    let json_str = render_json(&entries).unwrap();

    let parsed: kei_registry::encyclopedia::Encyclopedia =
        serde_json::from_str(&json_str).expect("valid JSON");
    assert_eq!(parsed.total_blocks, 5);
    assert_eq!(parsed.blocks.len(), 5);
    assert!(parsed.counts.contains_key("primitive"));
    assert!(parsed.counts.contains_key("hook"));
}

// ── test 3: --type filter returns only matching rows ──────────────────────

#[test]
fn type_filter_returns_only_matching_type() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();
    seed_db(&conn);

    let rule_blocks = kei_registry::list_by_type(&conn, BlockType::Rule).unwrap();
    let all = kei_registry::list(&conn, true, 1000).unwrap();
    let entries = to_entries(&rule_blocks);
    let md = render_markdown(&entries, &all);

    assert!(md.contains("code-style-1"), "rule name present");
    assert!(!md.contains("kei-cache"), "primitive absent from rule-filtered output");
    assert!(!md.contains("deploy-kit"), "skill absent from rule-filtered output");
    // Counts table reflects only the filtered type.
    assert!(md.contains("Total blocks: 1"), "total = 1 for rule filter");
}

// ── test 4: empty registry renders without panic ──────────────────────────

#[test]
fn empty_registry_renders_without_panic() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();

    let active = kei_registry::list(&conn, false, 1000).unwrap();
    let all = kei_registry::list(&conn, true, 1000).unwrap();
    let entries = to_entries(&active);
    let md = render_markdown(&entries, &all);

    assert!(md.contains("Total blocks: 0"), "zero total shown");
    assert!(!md.is_empty(), "output is not blank");

    let json_str = render_json(&entries).unwrap();
    let parsed: kei_registry::encyclopedia::Encyclopedia =
        serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.total_blocks, 0);
    assert!(parsed.blocks.is_empty());
}

// ── test 5: supersede chain renders correctly ─────────────────────────────

#[test]
fn supersede_chain_renders_in_markdown() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();

    let path = "/tmp/evolving-block.md";
    let _v1 = register(&conn, BlockType::Rule, "evolving", path, b"body v1", "").unwrap();
    let _v2 = register(&conn, BlockType::Rule, "evolving", path, b"body v2 changed", "").unwrap();

    let active = kei_registry::list(&conn, false, 1000).unwrap();
    let all = kei_registry::list(&conn, true, 1000).unwrap();
    let entries = to_entries(&active);
    let md = render_markdown(&entries, &all);

    // Active section: only 1 row (the newest).
    assert!(md.contains("Total blocks: 1"), "only active counted");
    // Supersede chain section must appear with 2 versions.
    assert!(md.contains("Supersede chains"), "chain section present");
    assert!(md.contains("`evolving`"), "name in chain section");
    assert!(md.contains("2 versions"), "two versions noted");
}

// ── test 6: section headers only appear for non-empty types ───────────────

#[test]
fn section_headers_only_for_non_empty_types() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();

    // Insert only a primitive and a hook.
    register(&conn, BlockType::Primitive, "kei-log", "/tmp/kei-log/Cargo.toml", b"body", "").unwrap();
    register(&conn, BlockType::Hook, "stop-verify", "/tmp/hooks/stop-verify.sh", b"body", "Stop").unwrap();

    let active = kei_registry::list(&conn, false, 1000).unwrap();
    let all = kei_registry::list(&conn, true, 1000).unwrap();
    let entries = to_entries(&active);
    let md = render_markdown(&entries, &all);

    assert!(md.contains("## Primitive"), "primitive section present");
    assert!(md.contains("## Hook"), "hook section present");
    // Types with zero rows must NOT appear.
    assert!(!md.contains("## Skill"), "skill section absent");
    assert!(!md.contains("## Rule"), "rule section absent");
    assert!(!md.contains("## Atom"), "atom section absent");
}

// ── test 7: to_entries ordering is stable (alpha by type then name) ────────

#[test]
fn entries_sorted_by_type_then_name() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("r.sqlite")).unwrap();

    register(&conn, BlockType::Atom, "z-atom", "/tmp/z.md", b"z", "").unwrap();
    register(&conn, BlockType::Atom, "a-atom", "/tmp/a.md", b"a", "").unwrap();
    register(&conn, BlockType::Primitive, "mid-prim", "/tmp/mid/Cargo.toml", b"m", "").unwrap();

    let active = kei_registry::list(&conn, false, 1000).unwrap();
    let entries = to_entries(&active);

    // "atom" sorts before "primitive" alphabetically.
    let types: Vec<&str> = entries.iter().map(|e| e.block_type.as_str()).collect();
    let prim_idx = types.iter().position(|&t| t == "primitive").unwrap();
    let first_atom = types.iter().position(|&t| t == "atom").unwrap();
    assert!(first_atom < prim_idx, "atom before primitive (alphabetical)");

    // Within atom, a-atom before z-atom.
    let atom_names: Vec<&str> = entries
        .iter()
        .filter(|e| e.block_type == "atom")
        .map(|e| e.name.as_str())
        .collect();
    assert_eq!(atom_names, vec!["a-atom", "z-atom"]);
}

// suppress unused import warning in certain build configs
#[allow(dead_code)]
fn _use_entry_type(_: EncyclopediaEntry) {}
