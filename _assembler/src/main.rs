//! CLI entry: build [--validate] [--in-place] [<manifest.toml> ...]
//!
//! Default: read all _manifests/*.toml, write to _generated/*.md.
//! --in-place: write to agents/<name>.md (replaces generated file).
//! --validate: parse + validate only, no output.
//! Positional args: specific manifest files to process.

mod assembler;
mod manifest;
mod placeholders;
mod registry_client;
mod schemas_export;
mod substrate;
mod validator;

use manifest::Manifest;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

fn main() -> ExitCode {
    let root = root_dir();
    let blocks = root.join("_blocks");
    let manifests = root.join("_manifests");
    let generated = root.join("_generated");

    let args: Vec<String> = env::args().skip(1).collect();
    let validate_only = args.iter().any(|a| a == "--validate");
    let in_place = args.iter().any(|a| a == "--in-place");
    let targets: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();

    let paths: Vec<PathBuf> = if targets.is_empty() {
        collect_manifests(&manifests)
    } else {
        targets.iter().map(|t| PathBuf::from(t)).collect()
    };

    if paths.is_empty() {
        eprintln!("no manifests found in {}", manifests.display());
        return ExitCode::from(1);
    }

    let mut errors = 0u32;
    for path in &paths {
        match process(path, &blocks, &generated, &root, validate_only, in_place) {
            Ok(out_path) => {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                match out_path {
                    Some(p) => println!("OK  {name} → {}", relative_to(&p, root.parent().unwrap_or(root.as_path()))),
                    None => println!("OK  {name}"),
                }
            }
            Err(e) => {
                eprintln!("FAIL {}: {e}", path.display());
                errors += 1;
            }
        }
    }

    if errors > 0 { ExitCode::from(1) } else { ExitCode::SUCCESS }
}

fn process(
    path: &Path,
    blocks: &Path,
    generated: &Path,
    root: &Path,
    validate_only: bool,
    in_place: bool,
) -> Result<Option<PathBuf>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read: {e}"))?;
    let m: Manifest = toml::from_str(&text).map_err(|e| format!("parse: {e}"))?;
    validator::validate(&m, blocks)?;

    if validate_only {
        return Ok(None);
    }

    let content = assembler::assemble(&m, blocks)?;
    let out_path = if in_place {
        root.join(format!("{}.md", m.name))
    } else {
        fs::create_dir_all(generated).map_err(|e| format!("mkdir generated: {e}"))?;
        generated.join(format!("{}.md", m.name))
    };
    fs::write(&out_path, content).map_err(|e| format!("write {}: {e}", out_path.display()))?;
    Ok(Some(out_path))
}

fn root_dir() -> PathBuf {
    // Priority: AGENT_ROOT env > HOME/.claude/agents default.
    // (exe-relative would break when the binary is symlinked or copied.)
    if let Ok(v) = env::var("AGENT_ROOT") {
        return PathBuf::from(v);
    }
    PathBuf::from(env::var("HOME").unwrap_or_default()).join(".claude/agents")
}

fn collect_manifests(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for entry in rd.flatten() {
            let p = entry.path();
            if p.extension().and_then(|e| e.to_str()) == Some("toml") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn relative_to(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}
