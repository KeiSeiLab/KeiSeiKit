//! Task + Milestone value types and enum validation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub task_type: String,
    pub parent_id: i64,
    pub assigned_to: String,
    pub due_date: i64,
    pub completed_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Milestone {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub target_date: i64,
    pub status: String,
    pub created_at: i64,
}

pub const VALID_STATUSES: &[&str] =
    &["pending", "in_progress", "completed", "cancelled", "blocked"];
pub const VALID_PRIORITIES: &[&str] = &["critical", "high", "medium", "low"];
pub const VALID_DEP_TYPES: &[&str] =
    &["blocks", "feeds_into", "subtask_of", "milestone_of", "assigned_to", "depends_on"];

pub fn is_valid_status(s: &str) -> bool {
    VALID_STATUSES.contains(&s)
}
pub fn is_valid_priority(s: &str) -> bool {
    VALID_PRIORITIES.contains(&s)
}
pub fn is_valid_dep(s: &str) -> bool {
    VALID_DEP_TYPES.contains(&s)
}
