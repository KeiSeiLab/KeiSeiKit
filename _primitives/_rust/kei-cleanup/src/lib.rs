//! kei-cleanup library facade — see CLEANUP-RUNTIME-SPEC.md §3.3.
//!
//! v0.1 detection-only API:
//!  * [`run_all`] — invoke every enabled scanner, aggregate findings.
//!  * [`run_scanner`] — invoke one scanner by name.
//!
//! Both produce a [`CleanupReport`] (or `Vec<Finding>` for the
//! single-scanner path). The library never edits files, never
//! commits — see RULE 0.13 ORCHESTRATOR-BRANCH-FIRST.

pub mod config;
pub mod error;
pub mod registry_bridge;
pub mod report;
pub mod scanners;

use chrono::Utc;
use std::path::Path;
use std::time::Instant;

pub use config::Config;
pub use error::CleanupError;
pub use report::{
    CleanupReport, Confidence, Counts, Finding, FindingKind, Location, Severity,
};

/// Run every enabled scanner. Per CLEANUP-RUNTIME-SPEC.md §3.3:
/// scanner failures are recorded under `scanners_skipped`, never
/// abort the whole run.
pub fn run_all(workspace: &Path, cfg: &Config) -> Result<CleanupReport, CleanupError> {
    let started = Instant::now();
    let timestamp_utc = Utc::now();
    let mut findings = Vec::new();
    let mut run = Vec::new();
    let mut skipped = Vec::new();
    dispatch_all(workspace, cfg, &mut findings, &mut run, &mut skipped);
    let counts = Counts::from_findings(&findings);
    Ok(CleanupReport {
        schema_version: 1,
        workspace: workspace.to_path_buf(),
        workspace_sha: read_sha(workspace),
        timestamp_utc,
        runtime_ms: started.elapsed().as_millis() as u64,
        findings,
        counts,
        scanners_run: run,
        scanners_skipped: skipped,
    })
}

fn dispatch_all(
    workspace: &Path,
    cfg: &Config,
    findings: &mut Vec<Finding>,
    run: &mut Vec<String>,
    skipped: &mut Vec<(String, String)>,
) {
    for &name in scanners::ALL {
        if !cfg.scanner_enabled(name) {
            skipped.push((name.to_string(), "disabled in cleanup.toml".into()));
            continue;
        }
        match dispatch(name, workspace, cfg) {
            Ok(mut found) => {
                findings.append(&mut found);
                run.push(name.to_string());
            }
            Err(CleanupError::ToolNotFound(reason)) => skipped.push((name.to_string(), reason)),
            Err(e) => skipped.push((name.to_string(), format!("error: {e}"))),
        }
    }
}

/// Run a single scanner by name.
pub fn run_scanner(name: &str, workspace: &Path, cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    dispatch(name, workspace, cfg)
}

fn dispatch(name: &str, workspace: &Path, cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    match name {
        "dead_code" => scanners::dead_code::scan(workspace, cfg),
        "unused_deps" => scanners::unused_deps::scan(workspace, cfg),
        "dep_drift" => scanners::dep_drift::scan(workspace, cfg),
        "loc_check" => scanners::loc_check::scan(workspace, cfg),
        "todo_age" => scanners::todo_age::scan(workspace, cfg),
        "coverage_map" => scanners::coverage_map::scan(workspace, cfg),
        "workspace_tests" => scanners::workspace_tests::scan(workspace, cfg),
        "doc_warnings" => scanners::doc_warnings::scan(workspace, cfg),
        "naming_consistency" => scanners::naming_consistency::scan(workspace, cfg),
        other => Err(CleanupError::ToolNotFound(format!("unknown scanner: {other}"))),
    }
}

/// Best-effort short SHA — returns "unknown" if not a git workspace
/// or `git` is not on PATH.
fn read_sha(workspace: &Path) -> String {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(workspace)
        .output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => "unknown".to_string(),
    }
}
