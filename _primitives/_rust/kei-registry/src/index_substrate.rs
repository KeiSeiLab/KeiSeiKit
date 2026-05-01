//! `index-substrate` subcommand — bulk registration of all kit substrate dirs.
//!
//! Constructor Pattern: this cube owns the multi-type substrate walk. It
//! delegates per-type scanning to existing scanner adapters (Primitive,
//! Skill, Hook inside kit/hooks, Atom, plus BlockMd, Capability, Role).
//! Each found artefact is forwarded to `registry::register` with idempotency.
//! Output: per-type count table printed as JSON.

use anyhow::Result;
use rusqlite::Connection;
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::handlers::Outcome;
use crate::paths::resolve_db;
use crate::scanners::atom::AtomScanner;
use crate::scanners::block_md::BlockMdScanner;
use crate::scanners::capability::CapabilityScanner;
use crate::scanners::hook::HookScanner;
use crate::scanners::primitive::PrimitiveScanner;
use crate::scanners::role::RoleScanner;
use crate::scanners::skill::SkillScanner;
use crate::scanners::{Found, Scanner};
use crate::store::open_db;

/// Per-type counts for `index-substrate` output.
#[derive(Debug, Default, Clone, Serialize)]
pub struct IndexCounts {
    pub registered: usize,
    pub skipped: usize,
    pub superseded: usize,
}

/// Top-level handler for `index-substrate`.
pub fn handle_index_substrate(
    kit_root: Option<PathBuf>,
    db: Option<PathBuf>,
    dry_run: bool,
) -> Result<Outcome> {
    let root = resolve_kit_root(kit_root);
    let conn = open_db(resolve_db(db))?;
    let mut by_type: BTreeMap<String, IndexCounts> = BTreeMap::new();
    let mut total = IndexCounts::default();

    // Hooks scanner root is kit/hooks for index-substrate (kit-internal hooks).
    let hooks_root = root.join("hooks");

    scan_and_register(&PrimitiveScanner, &root, &conn, "primitives", dry_run, &mut by_type, &mut total)?;
    scan_and_register(&SkillScanner, &root, &conn, "skills", dry_run, &mut by_type, &mut total)?;
    scan_and_register(&HookScanner, &hooks_root, &conn, "hooks", dry_run, &mut by_type, &mut total)?;
    scan_and_register(&AtomScanner, &root, &conn, "atoms", dry_run, &mut by_type, &mut total)?;
    scan_and_register(&BlockMdScanner, &root, &conn, "blocks", dry_run, &mut by_type, &mut total)?;
    scan_and_register(&CapabilityScanner, &root, &conn, "capabilities", dry_run, &mut by_type, &mut total)?;
    scan_and_register(&RoleScanner, &root, &conn, "roles", dry_run, &mut by_type, &mut total)?;

    print_summary(&root, total, by_type, dry_run);
    Ok(Outcome::Ok)
}

fn resolve_kit_root(kit_root: Option<PathBuf>) -> PathBuf {
    let raw = kit_root
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    raw.canonicalize().unwrap_or(raw)
}

fn scan_and_register<S: Scanner>(
    scanner: &S,
    scan_root: &Path,
    conn: &Connection,
    label: &str,
    dry_run: bool,
    by_type: &mut BTreeMap<String, IndexCounts>,
    total: &mut IndexCounts,
) -> Result<()> {
    let found = scanner.scan(scan_root)?;
    let counts = by_type.entry(label.to_string()).or_default();
    for f in &found {
        if dry_run {
            counts.registered += 1;
            total.registered += 1;
        } else {
            register_one(conn, f, counts, total)?;
        }
    }
    Ok(())
}

fn register_one(
    conn: &Connection,
    f: &Found,
    counts: &mut IndexCounts,
    total: &mut IndexCounts,
) -> Result<()> {
    let pre_existing = crate::registry::find_by_path(conn, &f.path)?;
    let block = crate::registry::register(conn, f.block_type, &f.name, &f.path, &f.body, &f.caps)?;
    match pre_existing {
        Some(prev) if prev.body_sha == block.body_sha => {
            counts.skipped += 1;
            total.skipped += 1;
        }
        Some(_) => {
            counts.superseded += 1;
            counts.registered += 1;
            total.superseded += 1;
            total.registered += 1;
        }
        None => {
            counts.registered += 1;
            total.registered += 1;
        }
    }
    Ok(())
}

fn print_summary(root: &Path, total: IndexCounts, by_type: BTreeMap<String, IndexCounts>, dry_run: bool) {
    let body = json!({
        "kit_root": root.display().to_string(),
        "dry_run": dry_run,
        "total": {
            "registered": total.registered,
            "skipped": total.skipped,
            "superseded": total.superseded,
        },
        "by_type": by_type,
    });
    println!("{}", serde_json::to_string_pretty(&body).unwrap_or_default());
}
