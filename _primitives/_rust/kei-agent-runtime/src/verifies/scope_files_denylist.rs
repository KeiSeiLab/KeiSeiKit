//! `scope::files-denylist` verify — `git diff --name-only main` on agent
//! worktree; fails if any touched path matches the denylist.

use crate::capability::*;
use crate::simulated_merge::{glob_match, run_git};

pub struct FilesDenylistVerify;

impl Capability for FilesDenylistVerify {
    fn name(&self) -> &'static str {
        "scope::files-denylist"
    }

    fn verify(&self, ctx: &VerifyContext) -> VerifyResult {
        let denylist = &ctx.task.scope.files_denylist;
        if denylist.is_empty() {
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
        let hits: Vec<&str> = diff
            .lines()
            .filter(|p| !p.is_empty())
            .filter(|p| denylist.iter().any(|g| glob_match(g, p)))
            .collect();
        if hits.is_empty() {
            VerifyResult::Pass
        } else {
            VerifyResult::Fail {
                reason: format!("{} path(s) in denylist", hits.len()),
                detail: Some(hits.join("\n")),
            }
        }
    }
}
