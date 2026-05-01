//! `scan` subcommand orchestrator.
//!
//! Constructor Pattern: this cube owns the multi-scanner walk. It opens
//! the SQLite store, dispatches to each `scanners::*` adapter, and merges
//! `Found` rows into idempotent `register()` calls. The output JSON
//! summarises counts so users can see at a glance what the kit has.

use anyhow::Result;
use rusqlite::Connection;
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::block::BlockType;
use crate::handlers::Outcome;
use crate::paths::{resolve_db, resolve_hooks_root, resolve_kit_root, resolve_rules_root};
use crate::scanners::atom::AtomScanner;
use crate::scanners::hook::HookScanner;
use crate::scanners::primitive::PrimitiveScanner;
use crate::scanners::rule::RuleScanner;
use crate::scanners::skill::SkillScanner;
use crate::scanners::{Found, Scanner};
use crate::store::open_db;

/// Per-type scan counters.
#[derive(Debug, Default, Clone, Serialize)]
pub struct ScanCounts {
    pub registered: usize,
    pub skipped: usize,
    pub superseded: usize,
}

/// Top-level handler. Resolves roots, runs scanners, registers results,
/// prints summary JSON, returns Outcome::Ok.
pub fn handle_scan(
    kit_root: Option<PathBuf>,
    rules_root: Option<PathBuf>,
    hooks_root: Option<PathBuf>,
    db: Option<PathBuf>,
    types: Option<String>,
) -> Result<Outcome> {
    let kit = resolve_kit_root(kit_root);
    let rules = resolve_rules_root(rules_root);
    let hooks = resolve_hooks_root(hooks_root);
    let conn = open_db(resolve_db(db))?;
    let allowed = parse_types_filter(types.as_deref())?;
    let mut by_type: BTreeMap<String, ScanCounts> = BTreeMap::new();
    let total = run_all_scanners(&conn, &kit, &rules, &hooks, &allowed, &mut by_type)?;
    print_summary(total, by_type);
    Ok(Outcome::Ok)
}

fn print_summary(total: ScanCounts, by_type: BTreeMap<String, ScanCounts>) {
    let body = json!({
        "registered": total.registered,
        "skipped": total.skipped,
        "superseded": total.superseded,
        "by_type": by_type,
    });
    println!("{}", serde_json::to_string_pretty(&body).unwrap_or_default());
}

fn run_all_scanners(
    conn: &Connection,
    kit: &Path,
    rules: &Path,
    hooks: &Path,
    allowed: &[BlockType],
    by_type: &mut BTreeMap<String, ScanCounts>,
) -> Result<ScanCounts> {
    let mut total = ScanCounts::default();
    if allowed.contains(&BlockType::Primitive) {
        run_one(&PrimitiveScanner, kit, conn, &mut total, by_type)?;
    }
    if allowed.contains(&BlockType::Skill) {
        run_one(&SkillScanner, kit, conn, &mut total, by_type)?;
    }
    if allowed.contains(&BlockType::Rule) {
        run_one(&RuleScanner, rules, conn, &mut total, by_type)?;
    }
    if allowed.contains(&BlockType::Hook) {
        run_one(&HookScanner, hooks, conn, &mut total, by_type)?;
    }
    if allowed.contains(&BlockType::Atom) {
        run_one(&AtomScanner, kit, conn, &mut total, by_type)?;
    }
    Ok(total)
}

fn run_one<S: Scanner>(
    scanner: &S,
    root: &Path,
    conn: &Connection,
    total: &mut ScanCounts,
    by_type: &mut BTreeMap<String, ScanCounts>,
) -> Result<()> {
    let found = scanner.scan(root)?;
    for f in found {
        let counts_for_type = by_type.entry(f.block_type.to_string()).or_default();
        register_one(conn, &f, counts_for_type, total)?;
    }
    Ok(())
}

fn register_one(
    conn: &Connection,
    f: &Found,
    counts_for_type: &mut ScanCounts,
    total: &mut ScanCounts,
) -> Result<()> {
    let pre_existing = crate::registry::find_by_path(conn, &f.path)?;
    let block = crate::registry::register(conn, f.block_type, &f.name, &f.path, &f.body, &f.caps)?;
    classify_outcome(pre_existing.as_ref(), &block, counts_for_type, total);
    Ok(())
}

fn classify_outcome(
    pre_existing: Option<&crate::block::Block>,
    block: &crate::block::Block,
    counts_for_type: &mut ScanCounts,
    total: &mut ScanCounts,
) {
    match pre_existing {
        Some(prev) if prev.body_sha == block.body_sha => {
            counts_for_type.skipped += 1;
            total.skipped += 1;
        }
        Some(_) => {
            counts_for_type.superseded += 1;
            counts_for_type.registered += 1;
            total.superseded += 1;
            total.registered += 1;
        }
        None => {
            counts_for_type.registered += 1;
            total.registered += 1;
        }
    }
}

fn parse_types_filter(types: Option<&str>) -> Result<Vec<BlockType>> {
    match types {
        None => Ok(BlockType::all().to_vec()),
        Some(s) => s
            .split(',')
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| BlockType::from_str(t).map_err(anyhow::Error::msg))
            .collect(),
    }
}
