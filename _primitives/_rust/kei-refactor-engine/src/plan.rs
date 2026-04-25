//! Plan builder — turns Conflict list into PlanItems grouped by resolution.

use crate::input::Conflict;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Resolution {
    /// Engine can deterministically propose a patch.
    AutoApply,
    /// Engine flags, human decides — NEVER in patch.
    RequiresHumanDecision,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanItem {
    pub resolution: Resolution,
    pub category: String,
    pub severity: String,
    pub files: Vec<String>,
    pub why: String,
    pub example: String,
    pub tradeoff: String,
}

#[derive(Debug, Serialize)]
pub struct Plan {
    pub items: Vec<PlanItem>,
}

impl Plan {
    pub fn from_conflicts(conflicts: &[Conflict]) -> Self {
        let items = conflicts.iter().map(to_plan_item).collect();
        Plan { items }
    }

    pub fn auto_items(&self) -> Vec<&PlanItem> {
        self.items
            .iter()
            .filter(|i| i.resolution == Resolution::AutoApply)
            .collect()
    }

    pub fn manual_items(&self) -> Vec<&PlanItem> {
        self.items
            .iter()
            .filter(|i| i.resolution == Resolution::RequiresHumanDecision)
            .collect()
    }
}

fn to_plan_item(c: &Conflict) -> PlanItem {
    let resolution = if c.auto_resolvable {
        Resolution::AutoApply
    } else {
        Resolution::RequiresHumanDecision
    };
    PlanItem {
        resolution,
        category: c.category.clone(),
        severity: c.severity.clone(),
        files: c.files.clone(),
        why: c.evidence.clone(),
        example: build_example(c),
        tradeoff: build_tradeoff(c),
    }
}

fn build_example(c: &Conflict) -> String {
    match c.category.as_str() {
        "blocks" => format!(
            "keep `{}` as canonical; add a `> Deprecated: see <canonical>` header to the other",
            c.files.first().cloned().unwrap_or_default()
        ),
        "orphans" => format!("edit {} to remove the stale link, OR create the target", c.files.first().cloned().unwrap_or_default()),
        "hooks" => "union the matchers in one file; delete the other".to_string(),
        "rules" => "narrow one directive with a scope qualifier, keep the other strict".to_string(),
        "cp" => "extract the oversize part into a new sibling file".to_string(),
        _ => c.suggested_fix.clone(),
    }
}

fn build_tradeoff(c: &Conflict) -> String {
    match c.category.as_str() {
        "blocks" => "merge loses cross-link context; kept in deprecation header".to_string(),
        "orphans" => "deleting a stale ref may hide an intended-but-missing file".to_string(),
        "hooks" => "merged hook runs all logic on all matches; fine if logic is idempotent".to_string(),
        "rules" => "narrowing a rule reduces coverage; document the carve-out in the rule file".to_string(),
        "cp" => "split increases file count; offset by smaller cognitive units".to_string(),
        _ => "engine cannot evaluate tradeoff; human review required".to_string(),
    }
}
