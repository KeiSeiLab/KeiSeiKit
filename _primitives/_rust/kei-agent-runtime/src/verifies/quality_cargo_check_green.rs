//! `quality::cargo-check-green` — runs `cargo check --workspace` in
//! `<run_dir>/_primitives/_rust` and reports failure tail on non-zero exit.
//!
//! As of v0.18 convergence wave: thin const wrapper over `CommandVerify`.

use super::command_verify::{CommandVerify, WorkDir};

pub const CARGO_CHECK_GREEN: CommandVerify = CommandVerify {
    name: "quality::cargo-check-green",
    program: "cargo",
    base_args: &["check", "--workspace"],
    work_dir: WorkDir::WorkspaceRoot,
    expected_exit: 0,
    fail_reason: "cargo check --workspace FAILED — agent-local green ≠ integration green",
    custom_runner: None,
    arg_builder: None,
    result_mapper: None,
};
