//! kei-diff CLI.
//!
//! Usage:
//!   kei-diff diff  --old <path> --new <path>       # prints RFC 6902 patch
//!   kei-diff apply --base <path> --patch <path>    # prints result document
//!
//! No external arg-parser dep — this is a two-verb tool with fixed flag sets,
//! hand-rolling keeps the crate zero-dep beyond serde/serde_json.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(out) => {
            println!("{out}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kei-diff: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<String, String> {
    match args.first().map(String::as_str) {
        Some("diff") => cmd_diff(&args[1..]),
        Some("apply") => cmd_apply(&args[1..]),
        Some("help") | Some("--help") | Some("-h") | None => Ok(usage()),
        Some(other) => Err(format!("unknown subcommand {other:?}\n{}", usage())),
    }
}

fn cmd_diff(args: &[String]) -> Result<String, String> {
    let old_path = flag(args, "--old")?;
    let new_path = flag(args, "--new")?;
    let old = read_json(&old_path)?;
    let new = read_json(&new_path)?;
    let patch = kei_diff::diff(&old, &new);
    serde_json::to_string_pretty(&patch).map_err(|e| e.to_string())
}

fn cmd_apply(args: &[String]) -> Result<String, String> {
    let base_path = flag(args, "--base")?;
    let patch_path = flag(args, "--patch")?;
    let base = read_json(&base_path)?;
    let patch_json = std::fs::read_to_string(&patch_path)
        .map_err(|e| format!("read {patch_path}: {e}"))?;
    let patch: kei_diff::Patch = serde_json::from_str(&patch_json)
        .map_err(|e| format!("parse patch: {e}"))?;
    let out = kei_diff::apply(&base, &patch).map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&out).map_err(|e| e.to_string())
}

fn flag(args: &[String], name: &str) -> Result<String, String> {
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        if a == name {
            return iter
                .next()
                .cloned()
                .ok_or_else(|| format!("flag {name} requires a value"));
        }
    }
    Err(format!("missing required flag {name}"))
}

fn read_json(path: &str) -> Result<serde_json::Value, String> {
    let txt = std::fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
    serde_json::from_str(&txt).map_err(|e| format!("parse {path}: {e}"))
}

fn usage() -> String {
    "kei-diff — structural JSON diff (RFC 6902 add/remove/replace)\n\n\
     USAGE:\n  \
       kei-diff diff  --old <file> --new <file>\n  \
       kei-diff apply --base <file> --patch <file>\n"
        .to_string()
}
