//! Auto-resolve plan writer.
//!
//! v0.14.1 retraction: this module used to emit a `*.patch` file with
//! `--- a/<file>` / `+++ b/<file>` headers that *looked* like unified-diff
//! but had no real hunk bodies. `git apply --check` rejects that format.
//! The claim "git apply-ready patch" was incorrect.
//!
//! New behaviour: we write a companion markdown file
//! (`plan-autoresolve.md`) listing the auto-apply candidates so the user
//! can review + apply them manually. File-content diffs would require
//! reading each source file, which is out of scope for this crate and
//! risks hallucinated edits (RULE 0.4). The "applied fork" path in
//! deep-sleep still produces a real branch via rename/move ops — those
//! are performed by the orchestrator, not by this file emitter.
//!
//! Only items whose `resolution == AutoApply` are listed here; the
//! zero-conflict guarantee keeps `requires_human_decision` items out.

use crate::plan::{Plan, PlanItem, Resolution};
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Write the auto-resolve review markdown. Returns the count of auto items.
///
/// The file is intentionally NOT a unified diff. It is a markdown
/// summary humans read before applying changes with an editor.
pub fn write_autoresolve(plan: &Plan, branch: &str, out_file: &Path) -> Result<usize> {
    let auto = plan.auto_items();
    let mut body = String::new();
    body.push_str(&header(branch, auto.len(), plan.manual_items().len()));
    for (idx, item) in auto.iter().enumerate() {
        body.push_str(&entry_for(idx + 1, item));
    }
    fs::write(out_file, body)?;
    Ok(auto.len())
}

fn header(branch: &str, auto: usize, manual: usize) -> String {
    format!(
        "# AUTO-RESOLVABLE items (review, don't `git apply`)\n\
         # Branch intent: {branch}\n\
         # Auto-apply candidates: {auto}\n\
         # Human-decision items (NOT listed here, see plan): {manual}\n\
         #\n\
         # This file is NOT a unified diff. Open each FILE below and apply\n\
         # the EXAMPLE change by hand. The engine does not read file contents\n\
         # and therefore cannot emit real +/- hunks (RULE 0.4: no fabricated\n\
         # edits).\n\n"
    )
}

fn entry_for(n: usize, item: &PlanItem) -> String {
    let files = item.files.join(", ");
    format!(
        "## {n}. [{cat}/{sev}] {first_file}\n\
         - FILES: {files}\n\
         - WHY: {why}\n\
         - EXAMPLE: {ex}\n\
         - TRADEOFF: {tr}\n\n",
        n = n,
        cat = item.category,
        sev = item.severity,
        first_file = item.files.first().cloned().unwrap_or_else(|| "<unknown>".into()),
        files = files,
        why = item.why,
        ex = item.example,
        tr = item.tradeoff,
    )
}

pub fn excluded_manual(plan: &Plan) -> Vec<&PlanItem> {
    plan.items
        .iter()
        .filter(|i| i.resolution == Resolution::RequiresHumanDecision)
        .collect()
}

// Backwards-compatibility shim for callers that still invoke the old name.
// Forwards to `write_autoresolve` — output semantics changed but signature
// matches. New code should call `write_autoresolve` directly.
#[deprecated(note = "renamed to write_autoresolve — output is no longer a unified diff")]
pub fn write_patch(plan: &Plan, branch: &str, out_file: &Path) -> Result<usize> {
    write_autoresolve(plan, branch, out_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{Plan, PlanItem, Resolution};

    fn sample_plan() -> Plan {
        Plan {
            items: vec![PlanItem {
                resolution: Resolution::AutoApply,
                category: "blocks".into(),
                severity: "medium".into(),
                files: vec!["_blocks/a.md".into(), "_blocks/b.md".into()],
                why: "75% shingle overlap".into(),
                example: "keep better-cited".into(),
                tradeoff: "deprecation header loses inbound links".into(),
            }],
        }
    }

    #[test]
    fn autoresolve_output_is_not_claimed_as_diff() {
        let plan = sample_plan();
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let n = write_autoresolve(&plan, "deep-sleep/2026-04-22", tmp.path()).unwrap();
        let body = fs::read_to_string(tmp.path()).unwrap();
        assert_eq!(n, 1);
        // Must NOT start with unified-diff headers — those are a lie here.
        assert!(!body.starts_with("--- a/"), "output starts with --- a/ (fake diff): {body}");
        assert!(!body.contains("\n--- a/"), "output contains --- a/ (fake diff): {body}");
        assert!(!body.contains("+++ b/"), "output contains +++ b/ (fake diff): {body}");
        // Must be human-readable markdown heading.
        assert!(body.contains("AUTO-RESOLVABLE items"));
    }

    #[test]
    fn autoresolve_includes_files_and_example() {
        let plan = sample_plan();
        let tmp = tempfile::NamedTempFile::new().unwrap();
        write_autoresolve(&plan, "x", tmp.path()).unwrap();
        let body = fs::read_to_string(tmp.path()).unwrap();
        assert!(body.contains("_blocks/a.md"));
        assert!(body.contains("_blocks/b.md"));
        assert!(body.contains("keep better-cited"));
    }
}
