//! `decompose-rules` CLI subcommand implementation.
//!
//! Walks `<rules-dir>/*.md`, `specialty/*.md`, and `projects/*.md`
//! (depth ≤ 2), parses each rule file into `RuleFragment`s, writes each
//! fragment body to `<frags-dir>/<rule>__<section>.md` (a real file), and
//! registers each fragment in `kei-registry` with that real path.
//!
//! Path convention: `<frags-dir>/<rule-slug>__<section-slug>.md`
//! Double-underscore separates slugs (shell-safe; `::` is not a valid path
//! component). This ensures `_assembler` can `fs::read_to_string` the path.
//!
//! Constructor Pattern: this cube owns the walk + write + register loop.
//! Parsing lives in `parsers::rule`. Registry API in `kei_registry`.
//! Migration (rebuild) lives in `rules_rebuild`.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use kei_registry::{open_db, register, Block, BlockType};

use crate::parsers::{parse_rule_file, RuleFragment};
use crate::rules_paths::{resolve_db_path, resolve_fragments_dir, resolve_rules_dir};
use crate::rules_walker::collect_rule_files;

/// Counters returned after a full run.
#[derive(Debug, Default)]
pub struct RunStats {
    pub files: usize,
    pub fragments: usize,
    pub new_or_superseded: usize,
    pub unchanged: usize,
}

/// Entry point called from `main.rs`.
pub fn run(
    rules_dir: Option<PathBuf>,
    registry_db: Option<PathBuf>,
    fragments_dir: Option<PathBuf>,
    dry_run: bool,
    rebuild_fragments: bool,
) -> ExitCode {
    let rules_dir = resolve_rules_dir(rules_dir);
    let db_path = resolve_db_path(registry_db);
    let frags_dir = resolve_fragments_dir(fragments_dir);

    if rebuild_fragments {
        match crate::rules_rebuild::run(&db_path, &frags_dir, dry_run) {
            Ok(n) => {
                println!("rebuild-fragments: {n} rows updated (dry_run={dry_run})");
                return ExitCode::SUCCESS;
            }
            Err(e) => return die(&format!("rebuild-fragments: {e}")),
        }
    }

    let paths = match collect_rule_files(&rules_dir) {
        Ok(p) => p,
        Err(e) => return die(&format!("walk failed: {e}")),
    };

    if dry_run {
        return run_dry(&paths, &frags_dir);
    }

    if let Err(e) = ensure_dir(&frags_dir) {
        return die(&format!("create fragments dir {}: {e}", frags_dir.display()));
    }

    let conn = match open_db(&db_path) {
        Ok(c) => c,
        Err(e) => return die(&format!("open registry at {}: {e}", db_path.display())),
    };

    let mut stats = RunStats::default();
    for path in &paths {
        if let Err(e) = process_file(path, &conn, &frags_dir, &mut stats) {
            eprintln!("warn: skip {} — {e}", path.display());
        }
    }
    print_summary(&stats);
    ExitCode::SUCCESS
}

// ── per-file processing ──────────────────────────────────────────────────────

fn process_file(
    path: &Path,
    conn: &rusqlite::Connection,
    frags_dir: &Path,
    stats: &mut RunStats,
) -> Result<()> {
    let frags = parse_rule_file(path)?;
    stats.files += 1;
    for frag in frags {
        stats.fragments += 1;
        let block = register_fragment(conn, frags_dir, path, &frag)?;
        if block.superseded_by.is_some() || is_fresh(&block) {
            stats.new_or_superseded += 1;
        } else {
            stats.unchanged += 1;
        }
    }
    Ok(())
}

fn register_fragment(
    conn: &rusqlite::Connection,
    frags_dir: &Path,
    source_path: &Path,
    frag: &RuleFragment,
) -> Result<Block> {
    let real_path = fragment_path(frags_dir, &frag.rule_slug, &frag.section_slug);
    write_fragment_file(&real_path, &frag.body)?;
    let name = format!("{}::{}", frag.rule_slug, frag.section_slug);
    let path_str = real_path
        .to_str()
        .with_context(|| format!("non-UTF8 fragment path: {}", real_path.display()))?;
    register(conn, BlockType::Rule, &name, path_str, frag.body.as_bytes(), "")
        .with_context(|| format!("register {name} (source: {})", source_path.display()))
}

// ── dry-run ──────────────────────────────────────────────────────────────────

fn run_dry(paths: &[PathBuf], frags_dir: &Path) -> ExitCode {
    let mut total_files = 0usize;
    let mut total_frags = 0usize;
    for path in paths {
        match parse_rule_file(path) {
            Ok(frags) => {
                total_files += 1;
                for f in &frags {
                    total_frags += 1;
                    let dest = fragment_path(frags_dir, &f.rule_slug, &f.section_slug);
                    println!(
                        "[dry-run] would write {} → register {}::{}",
                        dest.display(), f.rule_slug, f.section_slug
                    );
                }
            }
            Err(e) => eprintln!("warn: skip {} — {e}", path.display()),
        }
    }
    println!("[dry-run] {total_files} files, {total_frags} fragments");
    ExitCode::SUCCESS
}

// ── shared helpers (pub for rules_rebuild) ───────────────────────────────────

/// Canonical fragment file: `<frags_dir>/<rule>__<section>.md`.
pub fn fragment_path(frags_dir: &Path, rule_slug: &str, section_slug: &str) -> PathBuf {
    frags_dir.join(format!("{rule_slug}__{section_slug}.md"))
}

/// Write body to disk only if content differs from existing file.
pub fn write_fragment_file(path: &Path, body: &str) -> Result<()> {
    if let Ok(existing) = std::fs::read_to_string(path) {
        if existing == body { return Ok(()); }
    }
    std::fs::write(path, body)
        .with_context(|| format!("write fragment {}", path.display()))
}

/// Create directory (and parents) if absent.
pub fn ensure_dir(dir: &Path) -> Result<()> {
    std::fs::create_dir_all(dir)
        .with_context(|| format!("create dir {}", dir.display()))
}

fn is_fresh(block: &Block) -> bool {
    block.created == block.modified
}

fn print_summary(stats: &RunStats) {
    println!(
        "Decomposed {} rule files into {} fragments ({} new/superseded, {} unchanged)",
        stats.files, stats.fragments, stats.new_or_superseded, stats.unchanged
    );
}

fn die(msg: &str) -> ExitCode {
    eprintln!("decompose-rules error: {msg}");
    ExitCode::from(1)
}
