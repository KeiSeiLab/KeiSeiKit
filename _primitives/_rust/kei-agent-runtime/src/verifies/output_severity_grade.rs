//! `output::severity-grade` verify — asserts the agent's report mentions at
//! least one of HIGH / MEDIUM / LOW severity grades per schema §Output.

use crate::capability::*;
use std::path::PathBuf;

pub struct SeverityGrade;

impl Capability for SeverityGrade {
    fn name(&self) -> &'static str {
        "output::severity-grade"
    }

    fn verify(&self, ctx: &VerifyContext) -> VerifyResult {
        let report = match load_report(ctx) {
            Ok(r) => r,
            Err(e) => {
                return VerifyResult::Fail {
                    reason: "report file not found".into(),
                    detail: Some(e),
                }
            }
        };
        let has_grade =
            report.contains("HIGH") || report.contains("MEDIUM") || report.contains("LOW");
        if has_grade {
            VerifyResult::Pass
        } else {
            VerifyResult::Fail {
                reason: "report missing HIGH/MEDIUM/LOW severity grade".into(),
                detail: None,
            }
        }
    }
}

fn load_report(ctx: &VerifyContext) -> Result<String, String> {
    if let Ok(p) = std::env::var("AGENT_REPORT_PATH") {
        return std::fs::read_to_string(&p).map_err(|e| format!("{p}: {e}"));
    }
    let mut p: PathBuf = ctx.worktree_path.to_path_buf();
    p.push(".claude");
    p.push("agents");
    p.push(ctx.agent_id);
    p.push("review.md");
    std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))
}
