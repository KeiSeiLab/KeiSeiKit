//! Documentation-presence detector.
//!
//! Constructor Pattern: one cube = "does this project have N standard
//! doc files?". No git2 dep — the dashboard can answer "which repos
//! lack a CLAUDE.md?" without pulling libgit2 in.

use std::path::Path;

/// Four-way doc-presence snapshot at the project root.
#[derive(Debug, Clone, Copy, Default)]
pub struct DocsState {
    pub has_claude_md: bool,
    pub has_decisions_md: bool,
    pub has_runbook_md: bool,
    pub has_readme: bool,
}

/// Case-insensitive `<root>/<name>` lookup. Probes the supplied form
/// (canonical uppercase-stem) and its lowercase variant — matches the
/// `readme.md` lowercase convention seen in some sister repos.
fn has_file_ci(root: &Path, name: &str) -> bool {
    if root.join(name).is_file() {
        return true;
    }
    root.join(name.to_lowercase()).is_file()
}

/// Detect CLAUDE.md / DECISIONS.md / RUNBOOK.md / README.md at the
/// project root. Lowercase variants are accepted.
pub fn detect_docs(project_root: &Path) -> DocsState {
    DocsState {
        has_claude_md: has_file_ci(project_root, "CLAUDE.md"),
        has_decisions_md: has_file_ci(project_root, "DECISIONS.md"),
        has_runbook_md: has_file_ci(project_root, "RUNBOOK.md"),
        has_readme: has_file_ci(project_root, "README.md"),
    }
}
