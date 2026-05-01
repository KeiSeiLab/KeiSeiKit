//! Conflict record — the single unit of output.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Rules,
    Hooks,
    Blocks,
    Orphans,
    Cp,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Rules => "rules",
            Category::Hooks => "hooks",
            Category::Blocks => "blocks",
            Category::Orphans => "orphans",
            Category::Cp => "cp",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub category: Category,
    pub severity: Severity,
    pub files: Vec<String>,
    pub evidence: String,
    pub suggested_fix: String,
    /// `true` → refactor-engine may auto-apply. `false` → plan-only.
    pub auto_resolvable: bool,
}

impl Conflict {
    pub fn new(
        category: Category,
        severity: Severity,
        files: Vec<String>,
        evidence: impl Into<String>,
        suggested_fix: impl Into<String>,
        auto_resolvable: bool,
    ) -> Self {
        Self {
            category,
            severity,
            files,
            evidence: evidence.into(),
            suggested_fix: suggested_fix.into(),
            auto_resolvable,
        }
    }
}
