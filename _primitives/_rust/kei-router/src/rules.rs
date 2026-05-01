//! Keyword rule type + `require` predicate model.

use crate::extract::Extracted;

/// A dispatch rule: any matching keyword routes to `tool` if `require(extracted)` is true.
#[derive(Clone)]
pub struct KeywordRule {
    pub tool: &'static str,
    pub keywords: &'static [&'static str],
    pub require: fn(&Extracted) -> bool,
}

/// A dynamic (runtime-added) rule — owned strings so caller can build at startup.
#[derive(Clone, Debug)]
pub struct DynRule {
    pub tool: String,
    pub keywords: Vec<String>,
}

// Predicates mirroring the Go require funcs.
pub fn always(_e: &Extracted) -> bool {
    true
}
pub fn has_path(e: &Extracted) -> bool {
    !e.path.is_empty()
}
pub fn has_id(e: &Extracted) -> bool {
    e.id > 0
}
pub fn has_paths(e: &Extracted) -> bool {
    !e.paths.is_empty()
}
pub fn has_any_id_or_query(e: &Extracted) -> bool {
    e.id > 0 || !e.query.is_empty()
}
