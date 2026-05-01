//! kei-task atoms — one file per verb, each exposing
//! `pub fn run(store, input) -> Result<Output, Error>`.
//!
//! Reference implementation for the substrate schema (see
//! `docs/SUBSTRATE-SCHEMA.md`). Every other kei-* crate will follow
//! this shape in v0.24+.

pub mod add_dependency;
pub mod create;
pub mod search;

/// Verbs exposed through the `run-atom <verb>` machine-facing CLI.
///
/// Source of truth for the dispatch table. Unit tests assert this stays in
/// sync with the sub-modules so adding a new verb can't silently skip
/// `run-atom` wiring.
pub const VERBS: &[&str] = &["create", "add-dependency", "search"];

/// Errors from the `run-atom` dispatcher layer itself — NOT from the atom
/// bodies. Use `classify_dispatch_error` in main.rs to map to exit codes.
#[derive(Debug)]
pub enum DispatchError {
    UnknownVerb(String),
    InvalidInput(String),
    Create(create::Error),
    AddDep(add_dependency::Error),
    Search(search::Error),
}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownVerb(v) => write!(f, "no such atom verb `{v}` in crate kei-task"),
            Self::InvalidInput(e) => write!(f, "InvalidInput: {e}"),
            Self::Create(e) => write!(f, "{e}"),
            Self::AddDep(e) => write!(f, "{e}"),
            Self::Search(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for DispatchError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verbs_list_matches_submodules() {
        // If a new verb module is added, the VERBS list MUST gain it.
        // This test pins the exact size + order so drift is caught at CI.
        assert_eq!(VERBS, ["create", "add-dependency", "search"]);
    }
}
