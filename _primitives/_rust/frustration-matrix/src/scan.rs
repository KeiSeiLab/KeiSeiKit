//! Scanner — walk `root`, parse each chatlog, apply compiled categories
//! (or the firmware `Classifier` when `--model-dir` is set), emit rows.
//! Handles both curated markdown (`.md`) and raw Claude Code `.jsonl`.
//! Constructor Pattern: one public entry (`run`); helpers small + private.

use crate::categories::{compile_all, CompiledCategory};
use crate::classifier::Classifier;
use crate::hit::Hit;
use crate::jsonl::parse_user_lines as parse_jsonl;
use crate::markdown::parse as parse_md;
use crate::row::{to_csv, to_jsonl, Row, CSV_HEADER};
use crate::scan_classifier::build_row as build_classifier_row;
use crate::since;
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

/// Output format accepted by `scan`.
#[derive(Copy, Clone, Debug)]
pub enum Format {
    Csv,
    Jsonl,
}

/// Source file kind — dispatch target for per-file parser.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum FileKind {
    Markdown,
    Jsonl,
}

/// Inputs from the CLI layer — keep the main.rs dispatch thin.
pub struct ScanArgs<'a> {
    pub root: &'a Path,
    pub since_spec: &'a str,
    pub format: Format,
    pub output: &'a Path,
    pub skip_jsonl: bool,
    pub min_len: usize,
    /// When `Some`: bypass regex, classify via firmware. `None`: regex path.
    pub classifier: Option<&'a Classifier>,
}

/// Execute a full scan. Returns number of rows emitted.
pub fn run(args: ScanArgs<'_>) -> Result<usize> {
    let cutoff = since::parse(args.since_spec)?;
    let cats = compile_all();
    let files = collect_files(args.root, cutoff, args.skip_jsonl);
    let mut sink = open_sink(args.output, args.format)?;
    let mut total = 0usize;
    for (file, kind) in &files {
        total += scan_one(file, *kind, &cats, &mut sink, &args)?;
    }
    sink.flush().context("flush output sink")?;
    eprintln!(
        "frustration-matrix: {} rows from {} file(s) → {}",
        total,
        files.len(),
        args.output.display()
    );
    Ok(total)
}

fn collect_files(
    root: &Path,
    cutoff: Option<SystemTime>,
    skip_jsonl: bool,
) -> Vec<(PathBuf, FileKind)> {
    WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|r| r.ok())
        .filter_map(|e| classify_path(e.path(), skip_jsonl).map(|k| (e.into_path(), k)))
        .filter(|(p, _)| since::passes(p, cutoff))
        .collect()
}

/// Map a filesystem path to its parser kind, or `None` to skip.
fn classify_path(p: &Path, skip_jsonl: bool) -> Option<FileKind> {
    if !p.is_file() {
        return None;
    }
    let ext = p.extension().and_then(|e| e.to_str())?;
    if ext.eq_ignore_ascii_case("md") {
        Some(FileKind::Markdown)
    } else if !skip_jsonl && ext.eq_ignore_ascii_case("jsonl") {
        Some(FileKind::Jsonl)
    } else {
        None
    }
}

fn open_sink(output: &Path, fmt: Format) -> Result<Box<dyn Write>> {
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| format!("mkdir {}", parent.display()))?;
        }
    }
    let file = fs::File::create(output)
        .with_context(|| format!("create output {}", output.display()))?;
    let mut sink: Box<dyn Write> = Box::new(std::io::BufWriter::new(file));
    if matches!(fmt, Format::Csv) {
        writeln!(sink, "{CSV_HEADER}")?;
    }
    Ok(sink)
}

fn scan_one(
    file: &Path,
    kind: FileKind,
    cats: &[CompiledCategory],
    sink: &mut dyn Write,
    args: &ScanArgs<'_>,
) -> Result<usize> {
    let mtime = file_mtime_iso(file);
    let hits = load_hits(file, kind)?;
    let mut count = 0usize;
    for h in &hits {
        if h.text.chars().count() < args.min_len {
            continue;
        }
        count += match args.classifier {
            Some(c) => {
                let row = build_classifier_row(h, c, &mtime, args.min_len);
                write_row(sink, &row, args.format)?;
                1
            }
            None => apply_categories(h, cats, &mtime, sink, args.format)?,
        };
    }
    Ok(count)
}

/// Dispatch to the parser for `kind` and return `Hit`s. Markdown reads
/// the whole file (small curated chatlogs); JSONL streams line-by-line.
fn load_hits(file: &Path, kind: FileKind) -> Result<Vec<Hit>> {
    match kind {
        FileKind::Markdown => {
            let body = fs::read_to_string(file)
                .with_context(|| format!("read {}", file.display()))?;
            Ok(parse_md(file, &body).into_iter().map(Hit::from).collect())
        }
        FileKind::Jsonl => Ok(parse_jsonl(file)?.into_iter().map(Hit::from).collect()),
    }
}

fn apply_categories(
    hit: &Hit,
    cats: &[CompiledCategory],
    fallback_ts: &str,
    sink: &mut dyn Write,
    fmt: Format,
) -> Result<usize> {
    let mut count = 0usize;
    for c in cats {
        if c.patterns.iter().any(|p| p.is_match(&hit.text)) {
            let row = Row {
                category: c.id.to_string(),
                chatlog_file: hit.file.clone(),
                line_no: hit.line_no,
                timestamp: hit
                    .timestamp
                    .clone()
                    .unwrap_or_else(|| fallback_ts.to_string()),
                quote: hit.text.clone(),
                weight: c.weight,
            };
            write_row(sink, &row, fmt)?;
            count += 1;
        }
    }
    Ok(count)
}

fn write_row(sink: &mut dyn Write, row: &Row, fmt: Format) -> Result<()> {
    match fmt {
        Format::Csv => writeln!(sink, "{}", to_csv(row))?,
        Format::Jsonl => writeln!(sink, "{}", to_jsonl(row)?)?,
    }
    Ok(())
}

/// Best-effort ISO-ish stamp from mtime. Returns empty on FS errors — row
/// still lands, which matters for debugging a mis-configured scan.
fn file_mtime_iso(path: &Path) -> String {
    let Ok(meta) = fs::metadata(path) else {
        return String::new();
    };
    let Ok(mtime) = meta.modified() else {
        return String::new();
    };
    let Ok(dur) = mtime.duration_since(SystemTime::UNIX_EPOCH) else {
        return String::new();
    };
    format!("{}s", dur.as_secs())
}
