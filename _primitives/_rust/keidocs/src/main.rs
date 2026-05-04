//! keidocs CLI — extract / validate per-file markdown docs.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use keidocs::dna::compute_dna;
use keidocs::extractor::{extract_jsdoc, extract_md_headers, extract_rustdoc, Section};
use keidocs::render::render_markdown;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "keidocs", about = "Auto-extract per-file documentation with DNA frontmatter")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Walk source tree under --root and emit one .md per source file in --out.
    Extract {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Verify each .md in --out has dna_hash + parent backlink.
    Validate {
        #[arg(long)]
        out: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Extract { root, out } => run_extract(&root, &out),
        Cmd::Validate { out } => run_validate(&out),
    }
}

fn run_extract(root: &Path, out: &Path) -> Result<()> {
    fs::create_dir_all(out).with_context(|| format!("create {:?}", out))?;
    let mut count = 0usize;
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let lang = match detect_language(path) {
            Some(l) => l,
            None => continue,
        };
        if should_skip(path) {
            continue;
        }
        emit_one(root, path, out, lang)?;
        count += 1;
    }
    println!("{}", serde_json::json!({"extracted": count, "out": out}));
    Ok(())
}

fn detect_language(p: &Path) -> Option<&'static str> {
    let ext = p.extension()?.to_str()?;
    match ext {
        "rs" => Some("rust"),
        "ts" | "tsx" | "js" | "jsx" => Some("javascript"),
        "md" => Some("markdown"),
        _ => None,
    }
}

fn should_skip(p: &Path) -> bool {
    let s = p.to_string_lossy();
    s.contains("/target/") || s.contains("/node_modules/") || s.contains("/.git/")
}

fn emit_one(root: &Path, src: &Path, out: &Path, lang: &str) -> Result<()> {
    let content = fs::read_to_string(src).with_context(|| format!("read {:?}", src))?;
    let rel = src.strip_prefix(root).unwrap_or(src).to_string_lossy().to_string();
    let sections = extract_for_language(&content, lang);
    let deps = guess_deps(&content, lang);
    let dna = compute_dna(&rel, &content, &deps);
    let loc = content.lines().count();
    let md = render_markdown(&rel, &dna, lang, loc, &sections, &deps);
    // .md — KeiComments mounts via client-side script on <div id="keicomments-mount">.
    // (MDX parses every `<` as JSX, breaks on the 1.6k auto-generated source files.)
    let target_path = out.join(format!("{}.md", flatten_path(&rel)));
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(&target_path, md).with_context(|| format!("write {:?}", target_path))?;
    Ok(())
}

fn extract_for_language(content: &str, lang: &str) -> Vec<Section> {
    match lang {
        "rust" => extract_rustdoc(content),
        "javascript" => extract_jsdoc(content),
        "markdown" => extract_md_headers(content),
        _ => Vec::new(),
    }
}

fn guess_deps(content: &str, lang: &str) -> Vec<String> {
    let mut out = Vec::new();
    match lang {
        "rust" => collect_rust_uses(content, &mut out),
        "javascript" => collect_js_imports(content, &mut out),
        _ => {}
    }
    out.sort();
    out.dedup();
    out
}

fn collect_rust_uses(content: &str, out: &mut Vec<String>) {
    for line in content.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("use ") {
            if let Some(crate_name) = rest.split([':', ';', ' ']).next() {
                if !crate_name.is_empty() && crate_name != "self" && crate_name != "super" {
                    out.push(crate_name.to_string());
                }
            }
        }
    }
}

fn collect_js_imports(content: &str, out: &mut Vec<String>) {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("import ") || t.starts_with("from ") {
            out.push(t.to_string());
        }
    }
}

fn flatten_path(rel: &str) -> String {
    rel.replace('/', "__").replace('\\', "__")
}

fn run_validate(out: &Path) -> Result<()> {
    let mut ok = 0usize;
    let mut bad = 0usize;
    for entry in WalkDir::new(out).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let txt = fs::read_to_string(entry.path()).context("read md")?;
        if !txt.contains("dna_hash:") || !txt.contains("- parent:") {
            bad += 1;
        } else {
            ok += 1;
        }
    }
    println!("{}", serde_json::json!({"ok": ok, "bad": bad}));
    if bad > 0 {
        bail!("{} files missing dna_hash or parent backlink", bad);
    }
    Ok(())
}
