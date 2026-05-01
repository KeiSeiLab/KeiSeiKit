//! Integration smoke test for `index-substrate`.
//!
//! Creates a synthetic mini-kit, runs `handle_index_substrate`, verifies row
//! counts, then verifies idempotency (second run → 0 new) and supersede (one
//! file modified → 1 superseded, rest unchanged).

use kei_registry::index_substrate::handle_index_substrate;
use kei_registry::{list, open_db, BlockType};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

fn mini_kit_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("mini-kit")
}

// ── Smoke: 1 run → at least 6 rows (1 per type) ──────────────────────────

#[test]
fn index_substrate_registers_all_types() {
    let tmp = tempdir().unwrap();
    let db = tmp.path().join("reg.sqlite");
    handle_index_substrate(Some(mini_kit_root()), Some(db.clone()), false).unwrap();
    let conn = open_db(&db).unwrap();
    let all = list(&conn, false, 1000).unwrap();
    // Expect at least one of each type (primitive, skill, hook, block/atom, capability, role).
    let has_primitive = all.iter().any(|b| b.block_type == BlockType::Primitive);
    let has_skill = all.iter().any(|b| b.block_type == BlockType::Skill);
    let has_hook = all.iter().any(|b| b.block_type == BlockType::Hook);
    // blocks/capabilities/roles all land as Atom
    let atom_count = all.iter().filter(|b| b.block_type == BlockType::Atom).count();
    assert!(has_primitive, "primitive registered");
    assert!(has_skill, "skill registered");
    assert!(has_hook, "hook registered");
    assert!(atom_count >= 3, "at least 3 atoms (block + capability + role), got {atom_count}");
}

// ── Idempotency: second run → 0 new rows ─────────────────────────────────

#[test]
fn index_substrate_idempotent() {
    let tmp = tempdir().unwrap();
    let db = tmp.path().join("reg.sqlite");
    handle_index_substrate(Some(mini_kit_root()), Some(db.clone()), false).unwrap();
    let count_after_first: i64 = {
        let conn = open_db(&db).unwrap();
        conn.query_row("SELECT COUNT(*) FROM blocks WHERE superseded_by IS NULL", [], |r| r.get(0)).unwrap()
    };
    handle_index_substrate(Some(mini_kit_root()), Some(db.clone()), false).unwrap();
    let count_after_second: i64 = {
        let conn = open_db(&db).unwrap();
        conn.query_row("SELECT COUNT(*) FROM blocks WHERE superseded_by IS NULL", [], |r| r.get(0)).unwrap()
    };
    assert_eq!(count_after_first, count_after_second, "second run must not add rows");
}

// ── Supersede: modify one file → 1 superseded, rest unchanged ────────────

#[test]
fn index_substrate_supersede_on_change() {
    let tmp = tempdir().unwrap();
    // Copy the mini-kit to a mutable temp location.
    let kit_tmp = tmp.path().join("kit");
    copy_mini_kit(&kit_tmp);
    let db = tmp.path().join("reg.sqlite");

    handle_index_substrate(Some(kit_tmp.clone()), Some(db.clone()), false).unwrap();
    let active_before: i64 = {
        let conn = open_db(&db).unwrap();
        conn.query_row("SELECT COUNT(*) FROM blocks WHERE superseded_by IS NULL", [], |r| r.get(0)).unwrap()
    };

    // Modify one file.
    let role_file = kit_tmp.join("_roles").join("mini-role.toml");
    let mut content = fs::read_to_string(&role_file).unwrap();
    content.push_str("\n# modified");
    fs::write(&role_file, content).unwrap();

    handle_index_substrate(Some(kit_tmp.clone()), Some(db.clone()), false).unwrap();
    let active_after: i64 = {
        let conn = open_db(&db).unwrap();
        conn.query_row("SELECT COUNT(*) FROM blocks WHERE superseded_by IS NULL", [], |r| r.get(0)).unwrap()
    };
    let superseded_count: i64 = {
        let conn = open_db(&db).unwrap();
        conn.query_row("SELECT COUNT(*) FROM blocks WHERE superseded_by IS NOT NULL", [], |r| r.get(0)).unwrap()
    };

    assert_eq!(active_after, active_before, "active count unchanged (new row replaces old)");
    assert_eq!(superseded_count, 1, "exactly one superseded row");
}

// ── Dry-run: no rows written ──────────────────────────────────────────────

#[test]
fn index_substrate_dry_run_writes_nothing() {
    let tmp = tempdir().unwrap();
    let db = tmp.path().join("reg.sqlite");
    // open_db once to create schema; then dry-run should leave it empty.
    {
        let _ = open_db(&db).unwrap();
    }
    handle_index_substrate(Some(mini_kit_root()), Some(db.clone()), true).unwrap();
    let conn = open_db(&db).unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM blocks", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0, "dry-run must not write rows");
}

// ── Helper ────────────────────────────────────────────────────────────────

fn copy_mini_kit(dest: &std::path::Path) {
    let src = mini_kit_root();
    copy_dir_all(&src, dest).expect("copy mini-kit");
}

fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}
