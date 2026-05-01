//! kei-decompose CLI entry — clap dispatch only.
//!
//! All real work lives in module entrypoints. This file's only job is to
//! convert clap matches → module call → exit code.
//!
//! Exit codes:
//!   0  success
//!   1  file/IO error
//!   2  no parser detected / parse error
//!   3  kei-spawn invocation failed

use clap::Parser;
use std::path::Path;
use std::process::ExitCode;

use kei_decompose::cli::{Cli, Cmd, FormatHint};
use kei_decompose::dispatcher::{dispatch_all, DispatchOpts};
use kei_decompose::emitter::emit_all;
use kei_decompose::normalizer::Action;
use kei_decompose::parsers::{detect_format, parser_by_name, registry, FormatParser};
use kei_decompose::rules_cmd;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Detect { md } => run_detect(&md),
        Cmd::Parse { md, format } => run_parse(&md, format),
        Cmd::Emit { md, out, format } => run_emit(&md, &out, format),
        Cmd::Dispatch { md, dry_run, limit, format, no_ledger } => {
            run_dispatch(&md, dry_run, limit, format, no_ledger)
        }
        Cmd::Formats => run_formats(),
        Cmd::DecomposeRules { rules_dir, registry_db, fragments_dir, dry_run, rebuild_fragments } => {
            rules_cmd::run(rules_dir, registry_db, fragments_dir, dry_run, rebuild_fragments)
        }
    }
}

fn run_detect(md: &Path) -> ExitCode {
    let body = match read_or_die(md) {
        Ok(b) => b,
        Err(c) => return c,
    };
    let result = detect_format(&body);
    let json = serde_json::json!({
        "path": md.display().to_string(),
        "detected_format": result.winner,
        "confidence": result.confidence,
        "registered_parsers": result.all_scores.iter().map(|(n, _)| n).collect::<Vec<_>>(),
        "scoreboard": result.all_scores,
    });
    println!("{}", json);
    if result.winner.is_some() { ExitCode::SUCCESS } else { ExitCode::from(2) }
}

fn run_parse(md: &Path, hint: FormatHint) -> ExitCode {
    let actions = match parse_with_hint(md, hint) {
        Ok(a) => a,
        Err(c) => return c,
    };
    println!("{}", serde_json::to_string_pretty(&actions).unwrap_or_default());
    ExitCode::SUCCESS
}

fn run_emit(md: &Path, out: &Path, hint: FormatHint) -> ExitCode {
    let actions = match parse_with_hint(md, hint) {
        Ok(a) => a,
        Err(c) => return c,
    };
    match emit_all(&actions, out) {
        Ok(emitted) => {
            let paths: Vec<_> = emitted.iter().map(|e| &e.path).collect();
            println!("{}", serde_json::to_string_pretty(&paths).unwrap_or_default());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("emit failed: {e}");
            ExitCode::from(1)
        }
    }
}

fn run_dispatch(
    md: &Path,
    dry_run: bool,
    limit: Option<usize>,
    hint: FormatHint,
    no_ledger: bool,
) -> ExitCode {
    let actions = match parse_with_hint(md, hint) {
        Ok(a) => a,
        Err(c) => return c,
    };
    let temp = std::env::temp_dir().join(format!("kei-decompose-{}", std::process::id()));
    let emitted = match emit_all(&actions, &temp) {
        Ok(e) => e,
        Err(e) => return die(1, &format!("emit failed: {e}")),
    };
    let opts = DispatchOpts { dry_run, limit, no_ledger };
    finish_dispatch(&emitted, &opts)
}

fn finish_dispatch(emitted: &[kei_decompose::emitter::EmitOutput], opts: &DispatchOpts) -> ExitCode {
    match dispatch_all(emitted, opts) {
        Ok(records) => {
            println!("{}", serde_json::to_string_pretty(&records).unwrap_or_default());
            ExitCode::SUCCESS
        }
        Err(e) => die(3, &format!("dispatch failed: {e}")),
    }
}

fn die(code: u8, msg: &str) -> ExitCode {
    eprintln!("{msg}");
    ExitCode::from(code)
}

fn run_formats() -> ExitCode {
    let r = registry();
    let info: Vec<_> = r
        .iter()
        .map(|p| {
            serde_json::json!({
                "name": p.name(),
                "signatures": signature_for(p.name()),
            })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&info).unwrap_or_default());
    ExitCode::SUCCESS
}

fn signature_for(name: &str) -> Vec<&'static str> {
    match name {
        "research" => vec!["## Actionable plan", "## Backlog", "## Action items", "| Action |"],
        "audit" => vec!["## Wave N", "## Priority Matrix", "## Apply Plan"],
        "sleep" => vec!["REM:", "NREM:", "## Patterns", "## Backlog"],
        "architecture" => vec!["## Decision", "## Recommendation(s)", "## Implementation"],
        "new-project" => vec!["## Phase N", "## Verification"],
        _ => vec![],
    }
}

fn parse_with_hint(md: &Path, hint: FormatHint) -> Result<Vec<Action>, ExitCode> {
    let body = read_or_die(md)?;
    let parser = match hint {
        FormatHint::Auto => pick_auto_parser(&body)?,
        FormatHint::Research => parser_by_name("research").unwrap(),
        FormatHint::Audit => parser_by_name("audit").unwrap(),
        FormatHint::Sleep => parser_by_name("sleep").unwrap(),
        FormatHint::Architecture => parser_by_name("architecture").unwrap(),
        FormatHint::NewProject => parser_by_name("new-project").unwrap(),
    };
    parser.parse(md).map_err(|e| {
        eprintln!("parse failed: {e}");
        ExitCode::from(2)
    })
}

fn pick_auto_parser(body: &str) -> Result<Box<dyn FormatParser>, ExitCode> {
    let r = detect_format(body);
    if r.confidence < 0.5 {
        eprintln!("no parser claimed file (best confidence {})", r.confidence);
        return Err(ExitCode::from(2));
    }
    let name = r.winner.as_deref().ok_or_else(|| {
        eprintln!("detect winner missing");
        ExitCode::from(2)
    })?;
    parser_by_name(name).ok_or_else(|| {
        eprintln!("registered parser {name} not resolvable");
        ExitCode::from(2)
    })
}

fn read_or_die(md: &Path) -> Result<String, ExitCode> {
    std::fs::read_to_string(md).map_err(|e| {
        eprintln!("read {} failed: {e}", md.display());
        ExitCode::from(1)
    })
}
