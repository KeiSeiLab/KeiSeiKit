//! Sage-local aliases over `kei-atom-discovery` helpers.
//!
//! Historical sage API: `split_frontmatter`, `parse_wikilink`, `is_atom_target`,
//! `split_atom_id`. All now delegate to the shared crate; kept here so sage
//! internals compile without touch.

use anyhow::{anyhow, Result};
use kei_atom_discovery as shared;

pub use shared::{is_atom_target, parse_wikilink};

/// Split a `.md` file into (frontmatter_yaml, body). Delegates to the shared
/// `parse_frontmatter` — preserves the sage `anyhow::Result` return type.
pub fn split_frontmatter(text: &str) -> Result<(&str, &str)> {
    shared::parse_frontmatter(text).map_err(|e| anyhow!(e.to_string()))
}

/// Split `<crate>::<verb>` atom id into components.
pub fn split_atom_id(id: &str) -> Result<(String, String)> {
    shared::split_atom_id(id).map_err(|e| anyhow!(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_basic() {
        let src = "---\nfoo: bar\n---\nbody text\n";
        let (fm, body) = split_frontmatter(src).unwrap();
        assert_eq!(fm, "foo: bar\n");
        assert_eq!(body, "body text\n");
    }

    #[test]
    fn split_crlf() {
        let src = "---\r\nfoo: bar\r\n---\r\nbody\r\n";
        let (fm, _body) = split_frontmatter(src).unwrap();
        assert!(fm.contains("foo: bar"));
    }

    #[test]
    fn split_missing_start() {
        assert!(split_frontmatter("no frontmatter\n").is_err());
    }

    #[test]
    fn split_missing_end() {
        assert!(split_frontmatter("---\nfoo: bar\nbody\n").is_err());
    }

    #[test]
    fn wikilink_simple() {
        assert_eq!(
            parse_wikilink("[[kei-task::create]]"),
            Some("kei-task::create".into())
        );
    }

    #[test]
    fn wikilink_none() {
        assert_eq!(parse_wikilink("just text"), None);
        assert_eq!(parse_wikilink("[[ ]]"), None);
    }

    #[test]
    fn atom_target_filter() {
        assert!(is_atom_target("kei-task::create"));
        assert!(!is_atom_target("rules/RULE 0.12"));
    }

    #[test]
    fn split_id_ok() {
        let (c, v) = split_atom_id("kei-task::create").unwrap();
        assert_eq!(c, "kei-task");
        assert_eq!(v, "create");
    }

    #[test]
    fn split_id_bad() {
        assert!(split_atom_id("no-separator").is_err());
        assert!(split_atom_id("::empty").is_err());
    }
}
