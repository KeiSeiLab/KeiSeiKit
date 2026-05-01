//! CLI command handlers.
//!
//! Constructor Pattern: this cube wires CLI args to library calls. Each
//! handler is a thin adapter. The common parts (db path resolve,
//! id-or-DNA lookup) live in sibling cubes (`paths.rs`, `lookup.rs`).
//! Schema-version mismatch surfaces as exit 3, not-found 2, IO 1.

use anyhow::{Context, Result};
use serde_json::json;
use std::path::PathBuf;
use std::str::FromStr;

use crate::block::{Block, BlockType};
use crate::cli::Command;
use crate::encyclopedia::{render_json, render_markdown, to_entries};
use crate::index_substrate;
use crate::lookup::lookup_block;
use crate::paths::resolve_db;
use crate::registry::{list, list_by_type};
use crate::scan_orchestrator;
use crate::scanners::hook::HookScanner;
use crate::scanners::Scanner;
use crate::store::open_db;

/// Exit-code outcome. `Ok` for success, plus typed not-found variant.
#[derive(Debug)]
pub enum Outcome {
    Ok,
    NotFound(String),
}

/// Dispatch one parsed Command. Returns Outcome → main maps to exit code.
pub fn dispatch(cmd: Command) -> Result<Outcome> {
    match cmd {
        Command::Init { db } => handle_init(db),
        Command::Scan {
            kit_root,
            rules_root,
            hooks_root,
            db,
            types,
        } => scan_orchestrator::handle_scan(kit_root, rules_root, hooks_root, db, types),
        Command::Register {
            block_type,
            path,
            name,
            caps,
            db,
        } => handle_register(block_type, path, name, caps, db),
        Command::List {
            block_type,
            db,
            limit,
            include_superseded,
        } => handle_list(block_type, db, limit, include_superseded),
        Command::Get { target, db } => handle_get(target, db),
        Command::Related { target, depth, db } => handle_related(target, depth, db),
        Command::Diff { a, b, db } => handle_diff(a, b, db),
        Command::Stats { db } => handle_stats(db),
        Command::Encyclopedia {
            registry_db,
            output,
            format,
            block_type,
        } => handle_encyclopedia(registry_db, output, format, block_type),
        Command::RegisterSkill { path, name, db } => handle_register_skill(path, name, db),
        Command::RegisterHook { path, name, db } => handle_register_hook(path, name, db),
        Command::IndexSubstrate { kit_root, db, dry_run } => {
            index_substrate::handle_index_substrate(Some(kit_root), db, dry_run)
        }
        Command::Status {
            db,
            git_repo,
            ledger_db,
            format,
        } => handle_status(db, git_repo, ledger_db, format),
        Command::Secrets {
            env_files,
            scan_root,
            format,
        } => crate::secrets_handler::handle_secrets(env_files, scan_root, format),
    }
}

fn handle_status(
    db: Option<PathBuf>,
    git_repo: PathBuf,
    ledger_db: Option<PathBuf>,
    format: String,
) -> Result<Outcome> {
    let db_path = resolve_db(db);
    let conn = open_db(&db_path)?;
    let ledger = ledger_db.unwrap_or_else(crate::status::default_ledger_path);
    let snap = crate::status::compute_status(&conn, Some(&git_repo), Some(&ledger))?;
    match format.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&snap)?);
        }
        "ascii" | "" => {
            print!("{}", crate::status::render_ascii(&snap));
        }
        other => anyhow::bail!("unknown --format '{other}' (use ascii or json)"),
    }
    Ok(Outcome::Ok)
}

fn handle_init(db: Option<PathBuf>) -> Result<Outcome> {
    let path = resolve_db(db);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create parent dir {}", parent.display()))?;
    }
    let _conn = open_db(&path)?;
    println!(
        "{}",
        json!({
            "ok": true,
            "db": path.to_string_lossy(),
            "schema_version": crate::store::SCHEMA_VERSION,
        })
    );
    Ok(Outcome::Ok)
}

fn handle_register(
    type_str: String,
    path: PathBuf,
    name: Option<String>,
    caps: Option<String>,
    db: Option<PathBuf>,
) -> Result<Outcome> {
    let block_type = BlockType::from_str(&type_str).map_err(anyhow::Error::msg)?;
    let canonical = path
        .canonicalize()
        .with_context(|| format!("canonicalize {}", path.display()))?;
    let body = std::fs::read(&canonical)?;
    let final_name = name.unwrap_or_else(|| auto_name_from_path(&canonical));
    let final_caps = caps.unwrap_or_default();
    let conn = open_db(resolve_db(db))?;
    let block = crate::registry::register(
        &conn,
        block_type,
        &final_name,
        &canonical.to_string_lossy(),
        &body,
        &final_caps,
    )?;
    println!("{}", serde_json::to_string_pretty(&block)?);
    Ok(Outcome::Ok)
}

fn auto_name_from_path(canonical: &std::path::Path) -> String {
    canonical
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn handle_list(
    type_str: Option<String>,
    db: Option<PathBuf>,
    limit: i64,
    include_superseded: bool,
) -> Result<Outcome> {
    let conn = open_db(resolve_db(db))?;
    let blocks: Vec<Block> = match type_str {
        Some(t) => {
            let bt = BlockType::from_str(&t).map_err(anyhow::Error::msg)?;
            list_by_type(&conn, bt)?
        }
        None => list(&conn, include_superseded, limit)?,
    };
    println!("{}", serde_json::to_string_pretty(&blocks)?);
    Ok(Outcome::Ok)
}

fn handle_get(target: String, db: Option<PathBuf>) -> Result<Outcome> {
    let conn = open_db(resolve_db(db))?;
    match lookup_block(&conn, &target)? {
        Some(b) => {
            println!("{}", serde_json::to_string_pretty(&b)?);
            Ok(Outcome::Ok)
        }
        None => Ok(Outcome::NotFound(target)),
    }
}

fn handle_related(target: String, depth: u32, db: Option<PathBuf>) -> Result<Outcome> {
    let conn = open_db(resolve_db(db))?;
    let root = match lookup_block(&conn, &target)? {
        Some(b) => b,
        None => return Ok(Outcome::NotFound(target)),
    };
    let related = crate::related::find_related(&conn, &root, depth)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({"root": root, "related": related}))?
    );
    Ok(Outcome::Ok)
}

fn handle_diff(a: String, b: String, db: Option<PathBuf>) -> Result<Outcome> {
    let conn = open_db(resolve_db(db))?;
    let (block_a, block_b) = match (lookup_block(&conn, &a)?, lookup_block(&conn, &b)?) {
        (Some(x), Some(y)) => (x, y),
        (None, _) => return Ok(Outcome::NotFound(a)),
        (_, None) => return Ok(Outcome::NotFound(b)),
    };
    let diff = crate::diff::diff_blocks(&block_a, &block_b);
    println!("{}", serde_json::to_string_pretty(&diff)?);
    Ok(Outcome::Ok)
}

fn handle_stats(db: Option<PathBuf>) -> Result<Outcome> {
    let conn = open_db(resolve_db(db))?;
    let stats = crate::stats::compute_stats(&conn)?;
    println!("{}", serde_json::to_string_pretty(&stats)?);
    Ok(Outcome::Ok)
}

fn handle_register_skill(
    path: PathBuf,
    name: Option<String>,
    db: Option<PathBuf>,
) -> Result<Outcome> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("canonicalize skill dir {}", path.display()))?;
    if !canonical.is_dir() {
        anyhow::bail!("{} is not a directory", canonical.display());
    }
    let one = crate::scanners::skill::scan_one_skill(&canonical)?;
    let found: Vec<_> = match one {
        Some(f) => vec![f],
        None => anyhow::bail!("no SKILL.md found under {}", canonical.display()),
    };
    let conn = open_db(resolve_db(db))?;
    let mut results = Vec::new();
    for mut f in found {
        if let Some(ref n) = name {
            f.name = n.clone();
        }
        let block = crate::registry::register(&conn, f.block_type, &f.name, &f.path, &f.body, &f.caps)?;
        results.push(block);
    }
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(Outcome::Ok)
}

fn handle_register_hook(
    path: PathBuf,
    name: Option<String>,
    db: Option<PathBuf>,
) -> Result<Outcome> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("canonicalize hook file {}", path.display()))?;
    if !canonical.is_file() {
        anyhow::bail!("{} is not a file", canonical.display());
    }
    let parent = canonical.parent().unwrap_or(&canonical);
    let found = HookScanner.scan(parent)?;
    let stem = canonical
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    let hook_path = canonical.to_string_lossy().to_string();
    let target = found.into_iter().find(|f| f.path == hook_path);
    let mut f = match target {
        Some(f) => f,
        None => anyhow::bail!("{} not recognised as a .sh hook", canonical.display()),
    };
    if let Some(ref n) = name {
        f.name = n.clone();
    } else {
        f.name = stem.to_string();
    }
    let conn = open_db(resolve_db(db))?;
    let block = crate::registry::register(&conn, f.block_type, &f.name, &f.path, &f.body, &f.caps)?;
    println!("{}", serde_json::to_string_pretty(&block)?);
    Ok(Outcome::Ok)
}

fn handle_encyclopedia(
    registry_db: Option<PathBuf>,
    output: Option<PathBuf>,
    format: String,
    type_filter: Option<String>,
) -> Result<Outcome> {
    let db_path = resolve_db(registry_db);
    let conn = open_db(db_path)?;

    // Fetch active rows (optionally filtered) + all rows for supersede chains.
    let active_blocks = match &type_filter {
        Some(t) => {
            let bt = BlockType::from_str(t).map_err(anyhow::Error::msg)?;
            list_by_type(&conn, bt)?
        }
        None => list(&conn, false, i64::MAX)?,
    };
    let all_blocks = list(&conn, true, i64::MAX)?;

    let entries = to_entries(&active_blocks);

    let rendered = match format.as_str() {
        "json" => render_json(&entries)?,
        _ => render_markdown(&entries, &all_blocks),
    };

    match output {
        Some(path) => {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, &rendered)?;
            eprintln!("kei-registry: encyclopedia written to {}", path.display());
        }
        None => print!("{rendered}"),
    }
    Ok(Outcome::Ok)
}
