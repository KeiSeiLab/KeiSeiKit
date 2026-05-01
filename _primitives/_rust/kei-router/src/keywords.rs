//! Default keyword tables — aggregated from per-domain cubes.
//!
//! Ordering matters — more-specific multi-word keywords must come before
//! single-word matches on the same tool family.

use crate::kw_tables::{
    CHAT_RULES, CODE_RULES, CONTENT_RULES, CROSS_RULES, CURATOR_RULES,
    SAGE_RULES, SEARCH_RULES, SOCIAL_RULES, TASK_RULES,
};
use crate::rules::KeywordRule;

pub fn default_rules() -> Vec<KeywordRule> {
    let mut rules = Vec::with_capacity(128);
    rules.extend_from_slice(&SAGE_RULES);
    rules.extend_from_slice(&CODE_RULES);
    rules.extend_from_slice(&TASK_RULES);
    rules.extend_from_slice(&CHAT_RULES);
    rules.extend_from_slice(&CONTENT_RULES);
    rules.extend_from_slice(&SOCIAL_RULES);
    rules.extend_from_slice(&CROSS_RULES);
    rules.extend_from_slice(&CURATOR_RULES);
    rules.extend_from_slice(&SEARCH_RULES);
    rules
}
