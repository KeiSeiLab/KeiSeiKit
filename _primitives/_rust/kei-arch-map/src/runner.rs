//! Verify-run orchestrator. Dispatches each Claim to the matching
//! `evidence::*::check` function, prints a per-claim PASS/FAIL line,
//! and returns Err on any FAIL.

use anyhow::{anyhow, Result};
use kei_arch_map::evidence;
use kei_arch_map::schema::{self, Claim, Evidence};
use std::path::{Path, PathBuf};

pub use kei_arch_map::evidence::path_resolve::{confine_out, repo_root};

/// Run all claims in `plan_path`. Err if any FAIL.
pub fn run(plan_path: &Path) -> Result<()> {
    let plan = schema::load(plan_path)?;
    let root: PathBuf = repo_root(plan_path)?;
    let (total, pass, fail) = run_all(&plan, &root);
    println!("Total: {} claims, {} PASS, {} FAIL", total, pass, fail);
    if fail > 0 {
        Err(anyhow!("verification failed: {} claims", fail))
    } else {
        Ok(())
    }
}

fn run_all(plan: &schema::Plan, root: &Path) -> (usize, usize, usize) {
    let mut total = 0usize;
    let mut pass = 0usize;
    let mut fail = 0usize;
    for module in &plan.modules {
        for claim in &module.claims {
            total += 1;
            let (ok, reason) = check_claim(claim, root);
            if ok {
                pass += 1;
                println!("[PASS] {}::{}", module.id, claim.id);
            } else {
                fail += 1;
                eprintln!("[FAIL] {}::{} — {}", module.id, claim.id, reason);
            }
        }
    }
    (total, pass, fail)
}

/// Check a single claim. Returns (passed, reason_if_failed).
pub fn check_claim(claim: &Claim, root: &Path) -> (bool, String) {
    match &claim.evidence {
        Evidence::FileExists { path } => evidence::file_exists::check(path, root),
        Evidence::RegexMatch { file, pattern } => {
            evidence::regex_match::check(file, pattern, root)
        }
        Evidence::GrepCount {
            file,
            pattern,
            expected,
        } => evidence::grep_count::check(file, pattern, *expected, root),
        Evidence::FileSize { path, range } => evidence::file_size::check(path, range, root),
        Evidence::JsonField {
            file,
            path,
            expected,
        } => evidence::json_field::check(file, path, expected, root),
        Evidence::CargoCheckClean { manifest_dir } => {
            evidence::cargo_check::check(manifest_dir, root)
        }
        Evidence::CargoCheckSafe {
            manifest_dir,
            allowed_paths,
        } => evidence::cargo_check_safe::check(manifest_dir, allowed_paths, root),
        Evidence::HttpStatus { url, expected } => evidence::http_status::check(url, expected),
    }
}
