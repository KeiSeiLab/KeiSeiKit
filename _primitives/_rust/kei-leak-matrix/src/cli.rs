//! CLI surface — clap structs for every subcommand.
//!
//! Parsing only. Side-effects (file I/O, stdout, exit codes) live in
//! main.rs dispatch + the scanner / substituter / matrix modules.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "kei-leak-matrix",
          about = "Content protection scanner — SSoT for leak patterns.")]
pub struct Cli {
    /// Override matrix path (default: $KEI_LEAK_MATRIX_PATH or
    /// ~/Projects/KeiSeiKit/security/leak-matrix.toml)
    #[arg(long, global = true)]
    pub matrix: Option<PathBuf>,

    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ScopeArg { AllWrites, PublicMirror, GithubPush, CommitMsg }

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SeverityArg { Block, Warn, Substitute }

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CategoryArg { PatentIp, Secret, Personal, InternalInfra, PrivateProject }

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Scan a single file; emit JSON violations.
    Scan {
        file: PathBuf,
        #[arg(long)] scope: ScopeArg,
        #[arg(long)] severity: Option<SeverityArg>,
    },
    /// Recurse a directory; aggregate violations as JSON.
    ScanTree {
        dir: PathBuf,
        #[arg(long)] scope: ScopeArg,
        #[arg(long)] severity: Option<SeverityArg>,
    },
    /// Read stdin, apply substitute-severity rules, write to stdout.
    Substitute {
        #[arg(long)] scope: ScopeArg,
    },
    /// Test if a candidate pattern is already covered by a rule.
    Lint {
        #[arg(long)] pattern: String,
    },
    /// List all rules in markdown table form.
    List {
        #[arg(long)] category: Option<CategoryArg>,
    },
    /// Scan a literal command string (for hook integration).
    ScanCmd {
        cmd: String,
        #[arg(long)] scope: ScopeArg,
    },
}

impl ScopeArg {
    pub fn into_matrix(self) -> crate::matrix::Scope {
        match self {
            ScopeArg::AllWrites => crate::matrix::Scope::AllWrites,
            ScopeArg::PublicMirror => crate::matrix::Scope::PublicMirror,
            ScopeArg::GithubPush => crate::matrix::Scope::GithubPush,
            ScopeArg::CommitMsg => crate::matrix::Scope::CommitMsg,
        }
    }
}

impl SeverityArg {
    pub fn into_matrix(self) -> crate::matrix::Severity {
        match self {
            SeverityArg::Block => crate::matrix::Severity::Block,
            SeverityArg::Warn => crate::matrix::Severity::Warn,
            SeverityArg::Substitute => crate::matrix::Severity::Substitute,
        }
    }
}

impl CategoryArg {
    pub fn into_matrix(self) -> crate::matrix::Category {
        match self {
            CategoryArg::PatentIp => crate::matrix::Category::PatentIp,
            CategoryArg::Secret => crate::matrix::Category::Secret,
            CategoryArg::Personal => crate::matrix::Category::Personal,
            CategoryArg::InternalInfra => crate::matrix::Category::InternalInfra,
            CategoryArg::PrivateProject => crate::matrix::Category::PrivateProject,
        }
    }
}
