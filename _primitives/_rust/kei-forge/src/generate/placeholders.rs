//! Placeholder substitution for atom-template rendering.
//!
//! Pure string replace — six tokens, one pass per token. Called by
//! `super::generate_atom` for each of the five template files.
//!
//! Order matters: `__CRATE_SNAKE__` must be replaced BEFORE `__CRATE__`
//! (the latter is a substring of the former). Same for `__VERB_SNAKE__`
//! vs `__VERB__`. The implementation does longer-tokens-first.

use crate::form::ForgeRequest;

pub struct Placeholders {
    pub crate_name: String,
    pub crate_snake: String,
    pub verb: String,
    pub verb_snake: String,
    pub kind: String,
    pub description: String,
}

impl Placeholders {
    pub fn from_request(req: &ForgeRequest) -> Self {
        Self {
            crate_snake: req.crate_name.replace('-', "_"),
            crate_name: req.crate_name.clone(),
            verb_snake: req.verb.replace('-', "_"),
            verb: req.verb.clone(),
            kind: req.kind.clone(),
            description: req.description.clone(),
        }
    }

    /// Apply all six substitutions to `src`. Longer tokens first so that
    /// `__CRATE_SNAKE__` isn't consumed by the `__CRATE__` pass.
    pub fn substitute(&self, src: &str) -> String {
        src.replace("__CRATE_SNAKE__", &self.crate_snake)
            .replace("__VERB_SNAKE__", &self.verb_snake)
            .replace("__DESCRIPTION__", &self.description)
            .replace("__CRATE__", &self.crate_name)
            .replace("__VERB__", &self.verb)
            .replace("__KIND__", &self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req() -> ForgeRequest {
        ForgeRequest {
            crate_name: "kei-task".into(),
            verb: "add-dep".into(),
            kind: "command".into(),
            description: "docs".into(),
        }
    }

    #[test]
    fn snake_before_dash_form() {
        // If __CRATE__ ran first, __CRATE_SNAKE__ would become
        // "kei-task_SNAKE__" — verify ordering is correct.
        let p = Placeholders::from_request(&req());
        let out = p.substitute("__CRATE_SNAKE__ / __CRATE__");
        assert_eq!(out, "kei_task / kei-task");
    }

    #[test]
    fn verb_snake_correct() {
        let p = Placeholders::from_request(&req());
        let out = p.substitute("__VERB_SNAKE__ vs __VERB__");
        assert_eq!(out, "add_dep vs add-dep");
    }

    #[test]
    fn description_and_kind_pass_through() {
        let p = Placeholders::from_request(&req());
        assert_eq!(p.substitute("__KIND__"), "command");
        assert_eq!(p.substitute("__DESCRIPTION__"), "docs");
    }

    #[test]
    fn multiple_occurrences_all_replaced() {
        let p = Placeholders::from_request(&req());
        let out = p.substitute("__VERB__ __VERB__ __VERB__");
        assert_eq!(out, "add-dep add-dep add-dep");
    }

    #[test]
    fn empty_verb_crate_supported() {
        // Used in case callers pass an already-snake name (no dashes).
        let req = ForgeRequest {
            crate_name: "noop".into(),
            verb: "run".into(),
            kind: "query".into(),
            description: "".into(),
        };
        let p = Placeholders::from_request(&req);
        assert_eq!(p.substitute("__CRATE__ __CRATE_SNAKE__"), "noop noop");
        assert_eq!(p.substitute("__VERB__ __VERB_SNAKE__"), "run run");
    }
}
