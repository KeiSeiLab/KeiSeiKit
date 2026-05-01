//! Commit model — parsed conventional-commit record.

use std::fmt;

/// Conventional-commit kind.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommitKind {
    Feat,
    Fix,
    Refactor,
    Docs,
    Test,
    Chore,
    Perf,
    Ci,
    Build,
    Checkpoint,
    Audit,
    /// Anything we do not recognise as conventional.
    Other(String),
}

impl CommitKind {
    /// Stable ordering for grouping in CHANGELOG.md (lower = earlier).
    #[must_use]
    pub fn sort_key(&self) -> u8 {
        match self {
            Self::Feat => 0,
            Self::Fix => 1,
            Self::Perf => 2,
            Self::Refactor => 3,
            Self::Docs => 4,
            Self::Test => 5,
            Self::Build => 6,
            Self::Ci => 7,
            Self::Chore => 8,
            Self::Audit => 9,
            Self::Checkpoint => 10,
            Self::Other(_) => 11,
        }
    }

    /// Human-facing section heading used in `render::render_markdown`.
    #[must_use]
    pub fn heading(&self) -> &str {
        match self {
            Self::Feat => "Features",
            Self::Fix => "Fixes",
            Self::Perf => "Performance",
            Self::Refactor => "Refactor",
            Self::Docs => "Documentation",
            Self::Test => "Tests",
            Self::Build => "Build",
            Self::Ci => "CI",
            Self::Chore => "Chore",
            Self::Audit => "Audit",
            Self::Checkpoint => "Checkpoints",
            Self::Other(_) => "Other",
        }
    }
}

impl fmt::Display for CommitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.heading())
    }
}

/// Parsed commit record used by the walker and renderer.
#[derive(Debug, Clone)]
pub struct Commit {
    pub sha: String,
    pub kind: CommitKind,
    pub scope: Option<String>,
    pub subject: String,
    pub breaking: bool,
}
