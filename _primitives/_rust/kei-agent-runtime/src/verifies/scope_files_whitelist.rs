//! `scope::files-whitelist` verify — `git diff --name-only main` on agent
//! worktree; fails if any touched path is outside the whitelist.

use crate::capability::*;
use crate::simulated_merge::{glob_match, run_git};

pub struct FilesWhitelistVerify;

impl Capability for FilesWhitelistVerify {
    fn name(&self) -> &'static str {
        "scope::files-whitelist"
    }

    fn verify(&self, ctx: &VerifyContext) -> VerifyResult {
        let whitelist = &ctx.task.scope.files_whitelist;
        if whitelist.is_empty() {
            return VerifyResult::Pass;
        }
        let diff = match run_git(ctx.worktree_path, &["diff", "--name-only", "main"]) {
            Ok(s) => s,
            Err(e) => {
                return VerifyResult::Fail {
                    reason: "git diff --name-only main failed".into(),
                    detail: Some(e.to_string()),
                }
            }
        };
        let violators: Vec<&str> = diff
            .lines()
            .filter(|p| !p.is_empty())
            .filter(|p| !whitelist.iter().any(|g| glob_match(g, p)))
            .collect();
        if violators.is_empty() {
            VerifyResult::Pass
        } else {
            VerifyResult::Fail {
                reason: format!("{} path(s) outside whitelist", violators.len()),
                detail: Some(violators.join("\n")),
            }
        }
    }
}
