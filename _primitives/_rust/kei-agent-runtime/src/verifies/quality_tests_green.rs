//! `quality::tests-green` — runs `cargo test -p <crate>` for each crate in
//! `task.verification.cargo-test-crates`; parses `test result: ok. N passed`
//! lines; asserts total count ≥ `test_count_min` when set.
//!
//! As of v0.18 convergence wave: `CommandVerify` wrapper with a custom
//! per-crate runner (default exit-check shape doesn't fit the loop).

use super::command_verify::{tail, CommandVerify, WorkDir};
use crate::capability::{VerifyContext, VerifyResult};
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;
use std::process::Command;

pub const TESTS_GREEN: CommandVerify = CommandVerify {
    name: "quality::tests-green",
    program: "cargo",
    base_args: &[],
    work_dir: WorkDir::WorkspaceRoot,
    expected_exit: 0,
    fail_reason: "cargo test FAILED",
    custom_runner: Some(run_tests_green),
    arg_builder: None,
    result_mapper: None,
};

static TEST_SUMMARY: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"test result: ok\. (\d+) passed").unwrap());

fn run_tests_green(cv: &CommandVerify, ctx: &VerifyContext) -> VerifyResult {
    let crates = &ctx.task.verification.cargo_test_crates;
    if crates.is_empty() {
        return VerifyResult::Pass;
    }
    let dir = cv.resolve_dir(ctx);
    let mut total_passed: u64 = 0;
    for crate_name in crates {
        match run_test(&dir, crate_name) {
            Ok(n) => total_passed += n,
            Err(detail) => {
                return VerifyResult::Fail {
                    reason: format!("cargo test -p {crate_name} FAILED"),
                    detail: Some(detail),
                };
            }
        }
    }
    enforce_min(total_passed, ctx)
}

fn enforce_min(total: u64, ctx: &VerifyContext) -> VerifyResult {
    if let Some(min) = ctx.task.verification.test_count_min {
        if total < min as u64 {
            return VerifyResult::Fail {
                reason: format!("test count {total} < min {min}"),
                detail: None,
            };
        }
    }
    VerifyResult::Pass
}

fn run_test(dir: &Path, crate_name: &str) -> Result<u64, String> {
    let out = Command::new("cargo")
        .arg("test")
        .arg("-p")
        .arg(crate_name)
        .current_dir(dir)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(tail(&out.stderr, 10));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let passed: u64 = TEST_SUMMARY
        .captures_iter(&stdout)
        .filter_map(|c| c.get(1).and_then(|m| m.as_str().parse::<u64>().ok()))
        .sum();
    Ok(passed)
}
