//! kei-leak-matrix — binary entry. Dispatch only; logic lives in modules.

use anyhow::Result;
use clap::Parser;
use kei_leak_matrix::cli::{Cli, Cmd};
use kei_leak_matrix::matrix::{cmd_lint, cmd_list, default_matrix_path, Matrix};
use kei_leak_matrix::scanner::{cmd_scan_cmd, cmd_scan_file, cmd_scan_tree};
use kei_leak_matrix::substituter::cmd_substitute;
use std::process::ExitCode;

fn run() -> Result<i32> {
    let cli = Cli::parse();
    let path = cli.matrix.unwrap_or_else(default_matrix_path);
    let matrix = Matrix::load(&path)?;
    Ok(match cli.cmd {
        Cmd::Scan { file, scope, severity } =>
            cmd_scan_file(&matrix, &file, scope.into_matrix(), severity.map(|s| s.into_matrix()))?,
        Cmd::ScanTree { dir, scope, severity } =>
            cmd_scan_tree(&matrix, &dir, scope.into_matrix(), severity.map(|s| s.into_matrix()))?,
        Cmd::Substitute { scope } => cmd_substitute(&matrix, scope.into_matrix())?,
        Cmd::Lint { pattern } => cmd_lint(&matrix, &pattern),
        Cmd::List { category } => cmd_list(&matrix, category.map(|c| c.into_matrix())),
        Cmd::ScanCmd { cmd, scope } => cmd_scan_cmd(&matrix, &cmd, scope.into_matrix()),
    })
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => { eprintln!("kei-leak-matrix: {e:#}"); ExitCode::from(1) }
    }
}
