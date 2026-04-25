//! `tools::deny-tools` — denies Edit/Write/MultiEdit/NotebookEdit entirely.
//!
//! Renamed from `tools::read-only` in v0.17. The capability adds a list of
//! tools to the PreToolUse deny-list; the old name was a metaphor, the new
//! name describes the mechanism. Old name still resolves via registry alias.

use crate::capability::*;

pub struct DenyTools;

impl Capability for DenyTools {
    fn name(&self) -> &'static str {
        "tools::deny-tools"
    }

    fn check(&self, ctx: &GateContext) -> GateDecision {
        match ctx.tool_name {
            "Edit" | "Write" | "MultiEdit" | "NotebookEdit" => GateDecision::Deny {
                reason: format!(
                    "tools::deny-tools — {} denied (role is read-only)",
                    ctx.tool_name
                ),
            },
            _ => GateDecision::NotApplicable,
        }
    }
}
