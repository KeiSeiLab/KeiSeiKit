//! Task-class DNA extraction.
//!
//! Full DNA format: `<role>::<caps>::<sha8-scope>::<sha8-body>-<nonce8>`.
//! Examples:
//!   code-implementer-rust::?::e3929e37::041b7526-674c5cf3
//!   edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821::AC73A6A3-b3d36aa6
//!
//! Three abstraction levels for posterior aggregation:
//!
//! 1. `full_dna` — every spawn unique (random nonce). One observation per row.
//! 2. `task_class_dna` — strip `-<nonce8>`. Same prompt re-runs cluster.
//! 3. `agent_class_dna` — strip `::<body8>-<nonce8>`. Same agent at same scope,
//!    different prompts cluster. Highest-level routable identity.
//!
//! Constructor Pattern: this cube is purely lexical. No I/O, no SQL.

/// Strip trailing `-<nonce8>` from full DNA. Mirrors the SQL VIRTUAL column
/// in `kei-ledger` schema v9.
pub fn task_class_dna(full: &str) -> Option<&str> {
    if full.is_empty() {
        return None;
    }
    let bytes = full.as_bytes();
    if bytes.len() < 9 {
        return Some(full);
    }
    if bytes[bytes.len() - 9] == b'-' {
        Some(&full[..bytes.len() - 9])
    } else {
        Some(full)
    }
}

/// Strip `::<body8>-<nonce8>` from full DNA. Yields role+caps+scope identity.
pub fn agent_class_dna(full: &str) -> Option<&str> {
    let task_class = task_class_dna(full)?;
    let last_sep = task_class.rfind("::")?;
    Some(&task_class[..last_sep])
}

/// First `::` separated component — the substrate role slug.
/// Returns None for empty input or empty role segment (side fix).
pub fn role(dna: &str) -> Option<&str> {
    dna.split("::").next().filter(|s| !s.is_empty())
}

/// Second `::` separated component — capability bundle codes.
pub fn caps(dna: &str) -> Option<&str> {
    dna.split("::").nth(1)
}

/// Third `::` separated component — scope sha-8.
pub fn scope_sha(dna: &str) -> Option<&str> {
    dna.split("::").nth(2)
}

/// Body sha-8 from a task-class DNA (after agent_class).
/// Returns None if input is not a task_class_dna (i.e. shorter than 4 fields).
pub fn body_sha(task_class: &str) -> Option<&str> {
    task_class.split("::").nth(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "code-implementer-rust::?::e3929e37::041b7526-674c5cf3";

    #[test]
    fn strips_nonce_for_task_class() {
        assert_eq!(
            task_class_dna(SAMPLE),
            Some("code-implementer-rust::?::e3929e37::041b7526")
        );
    }

    #[test]
    fn strips_body_and_nonce_for_agent_class() {
        assert_eq!(
            agent_class_dna(SAMPLE),
            Some("code-implementer-rust::?::e3929e37")
        );
    }

    #[test]
    fn extracts_role_with_internal_hyphens() {
        assert_eq!(role(SAMPLE), Some("code-implementer-rust"));
    }

    #[test]
    fn extracts_caps_placeholder() {
        assert_eq!(caps(SAMPLE), Some("?"));
    }

    #[test]
    fn extracts_scope_sha() {
        assert_eq!(scope_sha(SAMPLE), Some("e3929e37"));
    }

    #[test]
    fn extracts_body_sha_from_task_class() {
        let tc = task_class_dna(SAMPLE).unwrap();
        assert_eq!(body_sha(tc), Some("041b7526"));
    }

    #[test]
    fn handles_real_caps_string() {
        let dna = "edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821::AC73A6A3-b3d36aa6";
        assert_eq!(role(dna), Some("edit-local"));
        assert_eq!(caps(dna), Some("NG-FW-FD-CP-CG-TG-ND-RF"));
        assert_eq!(scope_sha(dna), Some("5435F821"));
        assert_eq!(
            agent_class_dna(dna),
            Some("edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821")
        );
    }

    #[test]
    fn empty_returns_none() {
        assert_eq!(task_class_dna(""), None);
        assert_eq!(agent_class_dna(""), None);
        // Side fix: role("") returns None (not Some("")) — empty role is not useful.
        assert_eq!(role(""), None);
    }

    #[test]
    fn short_input_passes_through() {
        // Less than 9 chars total: no nonce to strip.
        assert_eq!(task_class_dna("abc"), Some("abc"));
    }

    #[test]
    fn missing_dash_means_no_nonce_strip() {
        // 9+ chars but no '-' at position len-9.
        let no_nonce = "role::cap::scope12::body";
        assert_eq!(task_class_dna(no_nonce), Some(no_nonce));
    }
}
