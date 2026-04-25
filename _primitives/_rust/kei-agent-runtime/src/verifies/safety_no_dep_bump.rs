//! `safety::no-dep-bump` verify — git-diffs Cargo.toml / Cargo.lock between
//! main and HEAD of the agent worktree; fails if any `version =` line changed.
//!
//! As of v0.18 convergence wave: `CommandVerify` wrapper with a custom
//! runner (git-diff + regex, not a plain exit-code check).

use super::command_verify::{CommandVerify, WorkDir};
use crate::capability::{VerifyContext, VerifyResult};
use crate::simulated_merge::run_git;
use once_cell::sync::Lazy;
use regex::Regex;

pub const NO_DEP_BUMP_VERIFY: CommandVerify = CommandVerify {
    name: "safety::no-dep-bump",
    program: "git",
    base_args: &[],
    work_dir: WorkDir::WorktreePath,
    expected_exit: 0,
    fail_reason: "safety::no-dep-bump — version bump detected",
    custom_runner: Some(run_no_dep_bump),
    arg_builder: None,
    result_mapper: None,
};

static VERSION_LINE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?m)^[-+]\s*version\s*=\s*".+""#).unwrap());

fn run_no_dep_bump(_cv: &CommandVerify, ctx: &VerifyContext) -> VerifyResult {
    let targets = ["Cargo.toml", "Cargo.lock"];
    let mut hits: Vec<String> = Vec::new();
    for t in targets.iter() {
        let args = ["diff", "main", "--", &format!("**/{t}"), t];
        if let Ok(diff) = run_git(ctx.worktree_path, &args) {
            for m in VERSION_LINE.find_iter(&diff) {
                hits.push(format!("{t}: {}", m.as_str()));
            }
        }
    }
    if hits.is_empty() {
        VerifyResult::Pass
    } else {
        VerifyResult::Fail {
            reason: format!("{} dep-bump line(s) detected", hits.len()),
            detail: Some(hits.join("\n")),
        }
    }
}
