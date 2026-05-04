//! kei-cleanup CLI entrypoint — see CLEANUP-RUNTIME-SPEC.md §3.4.
//!
//! v0.1 flags:
//!   * positional PATH (default `.`) — workspace root
//!   * `--json FILE`     — emit serialised CleanupReport
//!   * `--quiet`         — suppress TTY pretty output
//!   * `--fail-on LEVEL` — exit 1 if findings ≥ level (high|medium|low)
//!   * `--only LIST`     — comma-separated scanner whitelist

use anyhow::{anyhow, Result};
use clap::Parser;
use kei_cleanup::{run_all, run_scanner, CleanupReport, Config, Finding, Severity};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kei-cleanup", version, about = "Code-hygiene scanner")]
struct Cli {
    /// Workspace root (default: current directory).
    path: Option<PathBuf>,
    /// Emit JSON report to FILE.
    #[arg(long)]
    json: Option<PathBuf>,
    /// Suppress TTY output.
    #[arg(long)]
    quiet: bool,
    /// Exit 1 if findings ≥ this severity (high|medium|low).
    #[arg(long)]
    fail_on: Option<String>,
    /// Comma-list of scanner names to run.
    #[arg(long)]
    only: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let workspace = cli.path.unwrap_or_else(|| PathBuf::from("."));
    let cfg = Config::load_or_default(&workspace)?;

    let report = if let Some(list) = cli.only.as_deref() {
        run_subset(&workspace, &cfg, list)?
    } else {
        run_all(&workspace, &cfg)?
    };

    if !cli.quiet {
        print_pretty(&report);
    }
    if let Some(json_path) = cli.json {
        write_json(&report, &json_path)?;
    }
    if let Some(level) = cli.fail_on.as_deref() {
        let min = Severity::parse_min(level).ok_or_else(|| anyhow!("invalid --fail-on: {level}"))?;
        if has_at_or_above(&report.findings, min) {
            std::process::exit(1);
        }
    }
    Ok(())
}

fn run_subset(workspace: &PathBuf, cfg: &Config, list: &str) -> Result<CleanupReport> {
    let names: Vec<&str> = list.split(',').map(str::trim).filter(|s| !s.is_empty()).collect();
    let mut all = Vec::new();
    let mut run = Vec::new();
    let mut skipped = Vec::new();
    for name in names {
        match run_scanner(name, workspace, cfg) {
            Ok(mut f) => {
                all.append(&mut f);
                run.push(name.to_string());
            }
            Err(e) => skipped.push((name.to_string(), format!("{e}"))),
        }
    }
    let counts = kei_cleanup::Counts::from_findings(&all);
    Ok(CleanupReport {
        schema_version: 1,
        workspace: workspace.clone(),
        workspace_sha: "unknown".into(),
        timestamp_utc: chrono::Utc::now(),
        runtime_ms: 0,
        findings: all,
        counts,
        scanners_run: run,
        scanners_skipped: skipped,
    })
}

fn has_at_or_above(findings: &[Finding], min: Severity) -> bool {
    findings.iter().any(|f| f.severity >= min)
}

fn print_pretty(report: &CleanupReport) {
    let ts = report.timestamp_utc.format("%Y-%m-%dT%H-%M-%SZ");
    println!(
        "kei-cleanup v{}  •  workspace={}  •  sha={}",
        env!("CARGO_PKG_VERSION"),
        report.workspace.display(),
        report.workspace_sha
    );
    println!();
    println!(
        "═══ findings: {} [REAL: cleanup-run-{}] ═══",
        report.counts.total, ts
    );
    println!("  [HIGH]    {}", report.counts.high);
    println!("  [MEDIUM]  {}", report.counts.medium);
    println!("  [LOW]     {}", report.counts.low);
    println!();
    for f in &report.findings {
        print_finding(f);
    }
    if !report.scanners_skipped.is_empty() {
        println!("\nscanners skipped:");
        for (n, r) in &report.scanners_skipped {
            println!("  {n}: {r}");
        }
    }
}

fn print_finding(f: &Finding) {
    let sev = match f.severity {
        Severity::High => "HIGH",
        Severity::Medium => "MEDIUM",
        Severity::Low => "LOW",
    };
    let kind = format!("{:?}", f.kind);
    let line = f.location.line.map(|l| format!(":{l}")).unwrap_or_default();
    println!(
        "[{sev}] {kind}  {}{line}",
        f.location.file.display()
    );
    println!("  {}", f.message);
    if let Some(ref h) = f.fix_hint {
        println!("  fix: {h}");
    }
}

fn write_json(report: &CleanupReport, path: &PathBuf) -> Result<()> {
    let text = serde_json::to_string_pretty(report)?;
    std::fs::write(path, text)?;
    Ok(())
}
