//! `output::report-format` verify — reads agent's final report (env var
//! `AGENT_REPORT_PATH` or `.claude/agents/<id>/review.md`), asserts every
//! field in `task.output.report-fields-required` is mentioned.

use crate::capability::*;
use std::path::PathBuf;

pub struct ReportFormat;

impl Capability for ReportFormat {
    fn name(&self) -> &'static str {
        "output::report-format"
    }

    fn verify(&self, ctx: &VerifyContext) -> VerifyResult {
        let required = &ctx.task.output.report_fields_required;
        if required.is_empty() {
            return VerifyResult::Pass;
        }
        let report = match load_report(ctx) {
            Ok(r) => r,
            Err(e) => {
                return VerifyResult::Fail {
                    reason: "report file not found".into(),
                    detail: Some(e),
                }
            }
        };
        let missing: Vec<&String> = required.iter().filter(|f| !report.contains(f.as_str())).collect();
        if missing.is_empty() {
            VerifyResult::Pass
        } else {
            VerifyResult::Fail {
                reason: format!("{} required field(s) missing from report", missing.len()),
                detail: Some(
                    missing
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
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
