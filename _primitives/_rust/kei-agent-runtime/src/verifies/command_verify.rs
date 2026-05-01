//! Generic command-driven on-return verify (Layer D convergence, 2026-04-23).
//!
//! Absorbs the "run external command, check exit, optionally parse output"
//! shape shared by `quality::cargo-check-green`, `quality::tests-green`,
//! `safety::no-dep-bump` verify. Each concrete verify is now a
//! `CommandVerify` const declaration in its own file; execution logic lives
//! here.
//!
//! `quality::constructor-pattern` (LOC walker) and the `output::*` verifies
//! (parse agent report, no subprocess) stay in their own modules.

use crate::capability::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Where to run the command from.
#[derive(Clone, Copy)]
pub enum WorkDir {
    /// `<run_dir>/_primitives/_rust` if it exists, else `<run_dir>`.
    WorkspaceRoot,
    /// Raw `ctx.run_dir()`.
    RunDir,
    /// Raw `ctx.worktree_path` (pre-merge, for git-diff verifies).
    WorktreePath,
}

/// Decides how to build argv from `ctx` + config.
pub type ArgBuilder = fn(&VerifyContext, &CommandVerify) -> Vec<String>;

/// Post-processes `Output` into a `VerifyResult`. Default = exit-code check.
pub type ResultMapper = fn(&CommandVerify, &VerifyContext, &Output) -> VerifyResult;

/// Generic command-runner verify capability.
pub struct CommandVerify {
    pub name: &'static str,
    /// Executable name (e.g. "cargo", "git").
    pub program: &'static str,
    /// Literal args joined before per-crate / per-target dispatch. Used by
    /// `default_args` to produce the argv.
    pub base_args: &'static [&'static str],
    pub work_dir: WorkDir,
    pub expected_exit: i32,
    /// Human-readable failure reason when exit != expected.
    pub fail_reason: &'static str,
    /// If set, overrides the default "one shot, expected_exit" runner.
    /// Used by `quality::tests-green` (per-crate loop) + `safety::no-dep-bump`
    /// verify (regex over diff).
    pub custom_runner: Option<fn(&CommandVerify, &VerifyContext) -> VerifyResult>,
    /// If set, overrides `default_args`.
    pub arg_builder: Option<ArgBuilder>,
    /// If set, overrides `default_result_mapper`.
    pub result_mapper: Option<ResultMapper>,
}

impl Capability for CommandVerify {
    fn name(&self) -> &'static str {
        self.name
    }

    fn verify(&self, ctx: &VerifyContext) -> VerifyResult {
        if let Some(runner) = self.custom_runner {
            return runner(self, ctx);
        }
        self.run_once(ctx)
    }
}

impl CommandVerify {
    fn run_once(&self, ctx: &VerifyContext) -> VerifyResult {
        let dir = self.resolve_dir(ctx);
        let args = self.build_args(ctx);
        let out = Command::new(self.program).args(&args).current_dir(&dir).output();
        match out {
            Err(e) => VerifyResult::Fail {
                reason: format!("{} invocation failed", self.program),
                detail: Some(e.to_string()),
            },
            Ok(o) => self.map_result(ctx, &o),
        }
    }

    pub fn resolve_dir(&self, ctx: &VerifyContext) -> PathBuf {
        match self.work_dir {
            WorkDir::WorkspaceRoot => {
                let primitives = ctx.run_dir().join("_primitives/_rust");
                if primitives.is_dir() {
                    primitives
                } else {
                    ctx.run_dir()
                }
            }
            WorkDir::RunDir => ctx.run_dir(),
            WorkDir::WorktreePath => ctx.worktree_path.to_path_buf(),
        }
    }

    fn build_args(&self, ctx: &VerifyContext) -> Vec<String> {
        if let Some(f) = self.arg_builder {
            f(ctx, self)
        } else {
            self.base_args.iter().map(|s| s.to_string()).collect()
        }
    }

    fn map_result(&self, ctx: &VerifyContext, out: &Output) -> VerifyResult {
        if let Some(f) = self.result_mapper {
            return f(self, ctx, out);
        }
        default_exit_mapper(self, out)
    }
}

/// Default result mapper: pass iff exit == `expected_exit`, else Fail with
/// stderr tail.
pub fn default_exit_mapper(cv: &CommandVerify, out: &Output) -> VerifyResult {
    let actual = out.status.code().unwrap_or(-1);
    if actual == cv.expected_exit {
        VerifyResult::Pass
    } else {
        VerifyResult::Fail {
            reason: cv.fail_reason.to_string(),
            detail: Some(tail(&out.stderr, 10)),
        }
    }
}

/// Utility: last `n` lines of `bytes` as a String (lossy utf-8).
pub fn tail(bytes: &[u8], n: usize) -> String {
    let s = String::from_utf8_lossy(bytes);
    let lines: Vec<&str> = s.lines().collect();
    let start = lines.len().saturating_sub(n);
    lines[start..].join("\n")
}

/// Helper: run `cmd args...` in `dir`, return Output or stringified err.
pub fn run_in(dir: &Path, program: &str, args: &[&str]) -> Result<Output, String> {
    Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| e.to_string())
}
