//! gap_report — produce a markdown gap report from module match analyses.
//!
//! Three sections: confident matches (≥0.5), weak signals (0.3–0.5),
//! unmatched modules (no trait fits). Sorted descending by confidence
//! within sections; unmatched modules alphabetic.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use crate::matcher::MatchScore;

/// Per-module analysis combining module metadata with matcher output.
pub struct ModuleAnalysis {
    pub module: String,
    /// Number of source files in this module (for unmatched section display).
    pub file_count: usize,
    /// Estimated LOC (0 if unknown).
    pub loc_estimate: usize,
    /// Match scores from `match_module`, already sorted desc by confidence.
    pub matches: Vec<MatchScore>,
}

const CONFIDENT_THRESHOLD: f64 = 0.5;
const WEAK_THRESHOLD: f64 = 0.3;

/// Produce a markdown gap report covering all modules.
///
/// Returns a `String` containing the full report ready for stdout or file write.
pub fn render_gap_report(project_name: &str, analyses: &[ModuleAnalysis]) -> String {
    let mut out = String::with_capacity(4096);
    out.push_str(&format!("# {project_name} \u{2014} kei-import gap report\n\n"));
    render_confident(&mut out, analyses);
    render_weak(&mut out, analyses);
    render_unmatched(&mut out, analyses);
    render_next_steps(&mut out);
    out
}

fn render_confident(out: &mut String, analyses: &[ModuleAnalysis]) {
    out.push_str("## Confident matches (confidence \u{2265} 0.5)\n\n");
    out.push_str("| Module | Suggested trait | Confidence | Matched methods |\n");
    out.push_str("|---|---|---:|---|\n");

    let mut rows: Vec<(&str, &MatchScore)> = analyses
        .iter()
        .filter_map(|a| a.best_confident())
        .collect();
    rows.sort_by(|a, b| b.1.confidence.partial_cmp(&a.1.confidence).unwrap());

    if rows.is_empty() {
        out.push_str("| — | — | — | — |\n");
    }
    for (module, score) in rows {
        let methods = score.matched_methods.join(", ");
        out.push_str(&format!(
            "| {module} | {:?} | {:.2} | {methods} |\n",
            score.kind, score.confidence
        ));
    }
    out.push('\n');
}

fn render_weak(out: &mut String, analyses: &[ModuleAnalysis]) {
    out.push_str("## Weak signals (confidence 0.3\u{2013}0.5; review needed)\n\n");
    out.push_str("| Module | Best-guess trait | Confidence | Notes |\n");
    out.push_str("|---|---|---:|---|\n");

    let mut rows: Vec<(&str, &MatchScore)> = analyses
        .iter()
        .filter_map(|a| a.best_weak())
        .collect();
    rows.sort_by(|a, b| b.1.confidence.partial_cmp(&a.1.confidence).unwrap());

    if rows.is_empty() {
        out.push_str("| — | — | — | — |\n");
    }
    for (module, score) in rows {
        let note = format!("matched keywords: {}", score.matched_keywords.join(", "));
        out.push_str(&format!(
            "| {module} | {:?} | {:.2} | {note} |\n",
            score.kind, score.confidence
        ));
    }
    out.push('\n');
}

fn render_unmatched(out: &mut String, analyses: &[ModuleAnalysis]) {
    out.push_str("## Unmatched modules (no trait fits \u{2014} likely glue, utility, or novel concept)\n\n");

    let mut unmatched: Vec<&ModuleAnalysis> = analyses
        .iter()
        .filter(|a| a.is_unmatched())
        .collect();
    unmatched.sort_by(|a, b| a.module.cmp(&b.module));

    if unmatched.is_empty() {
        out.push_str("_None — all modules matched at least one trait._\n");
    }
    for a in unmatched {
        let loc_part = if a.loc_estimate > 0 {
            format!("{} source files, {} LOC", a.file_count, a.loc_estimate)
        } else {
            format!("{} source files", a.file_count)
        };
        out.push_str(&format!("- {} ({})\n", a.module, loc_part));
    }
    out.push('\n');
}

fn render_next_steps(out: &mut String) {
    out.push_str("## Suggested next steps\n\n");
    out.push_str("1. For confident matches: run `kei-import-project skeleton --module X --trait Y`\n");
    out.push_str("   to generate impl boilerplate.\n");
    out.push_str("2. For weak signals: manual review; the module may implement multiple\n");
    out.push_str("   traits or none.\n");
    out.push_str("3. For unmatched modules: classify manually \u{2014} utility, novel primitive,\n");
    out.push_str("   or out-of-scope.\n");
}

// ─────────────────────────── helpers ────────────────────────────────────────

impl ModuleAnalysis {
    /// Best confident match (confidence ≥ 0.5), if any.
    fn best_confident(&self) -> Option<(&str, &MatchScore)> {
        self.matches
            .iter()
            .find(|m| m.confidence >= CONFIDENT_THRESHOLD)
            .map(|m| (self.module.as_str(), m))
    }

    /// Best weak match (0.3 ≤ confidence < 0.5) ONLY when there's no confident match.
    fn best_weak(&self) -> Option<(&str, &MatchScore)> {
        if self.best_confident().is_some() {
            return None;
        }
        self.matches
            .iter()
            .find(|m| m.confidence >= WEAK_THRESHOLD && m.confidence < CONFIDENT_THRESHOLD)
            .map(|m| (self.module.as_str(), m))
    }

    /// True when NO match meets even the weak threshold.
    fn is_unmatched(&self) -> bool {
        !self.matches.iter().any(|m| m.confidence >= WEAK_THRESHOLD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_analyses_renders_without_panic() {
        let report = render_gap_report("test-project", &[]);
        assert!(report.contains("# test-project"));
        assert!(report.contains("Confident matches"));
        assert!(report.contains("Weak signals"));
        assert!(report.contains("Unmatched modules"));
    }
}
