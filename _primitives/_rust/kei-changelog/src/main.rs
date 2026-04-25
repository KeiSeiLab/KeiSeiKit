//! kei-changelog — CLI entry point.
//!
//! Thin wrapper over the library modules. Keeps flag parsing + IO here; all
//! commit / render logic lives in `lib.rs`.

use anyhow::{Context, Result};
use clap::Parser;
use kei_changelog::{
    group::Grouped, render::render_markdown, render::RenderOpts, walk::walk_range, walk::WalkRange,
};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "kei-changelog", version, about = "Generate CHANGELOG.md from conventional commits")]
struct Cli {
    /// Starting ref (exclusive). Defaults to the full history root.
    #[arg(long)]
    from: Option<String>,

    /// Ending ref (inclusive). Defaults to `HEAD`.
    #[arg(long, default_value = "HEAD")]
    to: String,

    /// Treat the range as an Unreleased section (overrides --version heading).
    #[arg(long)]
    unreleased: bool,

    /// Version label for the rendered block (e.g. "v0.7.0"). Ignored with --unreleased.
    #[arg(long, default_value = "v0.1.0")]
    version: String,

    /// Repository path. Defaults to current directory.
    #[arg(long, default_value = ".")]
    repo: PathBuf,

    /// Prepend output to this file (creates if missing). Without it, prints to stdout.
    #[arg(long)]
    update: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let range = WalkRange {
        from: cli.from.clone(),
        to: cli.to.clone(),
    };
    let commits = walk_range(&cli.repo, &range)?;
    let grouped = Grouped::from_commits(&commits);

    let version = if cli.unreleased {
        "Unreleased".to_string()
    } else {
        cli.version.clone()
    };
    let opts = RenderOpts::new(version);
    let rendered = render_markdown(&grouped, &opts);

    if let Some(path) = cli.update.as_ref() {
        let existing = fs::read_to_string(path).unwrap_or_default();
        let body = if existing.is_empty() {
            format!("# CHANGELOG\n\n{rendered}")
        } else {
            prepend_section(&existing, &rendered)
        };
        fs::write(path, body).with_context(|| format!("write {}", path.display()))?;
        eprintln!("[kei-changelog] updated {}", path.display());
    } else {
        print!("{rendered}");
    }
    Ok(())
}

/// Insert `section` after the top-level `# CHANGELOG` heading if present,
/// otherwise prepend. Never duplicates an existing identical section verbatim.
fn prepend_section(existing: &str, section: &str) -> String {
    if section.trim().is_empty() {
        return existing.to_string();
    }
    if existing.contains(section.trim()) {
        return existing.to_string();
    }
    if let Some(rest) = existing.strip_prefix("# CHANGELOG\n\n") {
        format!("# CHANGELOG\n\n{section}{rest}")
    } else {
        format!("{section}\n{existing}")
    }
}
