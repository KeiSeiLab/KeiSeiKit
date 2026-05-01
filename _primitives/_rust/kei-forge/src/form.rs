//! Form request deserialization + validation.
//!
//! Accepts either `application/x-www-form-urlencoded` (HTML `<form>`) or
//! `application/json` (future API clients). Validation enforces the
//! locked substrate schema — verb naming (kebab-case) and atom kind
//! enumeration (command | query | stream | transform).

use serde::{Deserialize, Serialize};

/// Incoming POST /forge body.
///
/// `crate` is renamed because it's a Rust reserved word.
#[derive(Debug, Deserialize, Serialize)]
pub struct ForgeRequest {
    #[serde(rename = "crate")]
    pub crate_name: String,
    pub verb: String,
    pub kind: String,
    pub description: String,
}

/// Validation outcome. `Ok(())` if the request matches schema constraints.
pub fn validate(req: &ForgeRequest) -> Result<(), String> {
    validate_crate_name(&req.crate_name)?;
    validate_verb(&req.verb)?;
    validate_kind(&req.kind)?;
    validate_description(&req.description)?;
    Ok(())
}

/// Description whitelist — ASCII printable only.
///
/// Hardening against shell-substitution in `scripts/new-atom.sh`: an
/// attacker-controlled newline, backtick, or `$` could smuggle a
/// secondary `sed` expression into the template-substitution step and
/// poison generated Rust source. Blocking these at the Rust layer
/// prevents the shell from ever seeing a hostile byte.
fn validate_description(d: &str) -> Result<(), String> {
    if d.len() > MAX_DESCRIPTION_LEN {
        return Err(format!(
            "description must be ≤{MAX_DESCRIPTION_LEN} chars (got {})",
            d.len()
        ));
    }
    for (i, b) in d.bytes().enumerate() {
        if !(0x20..=0x7E).contains(&b) {
            return Err(format!(
                "description contains non-printable byte 0x{b:02X} at offset {i}"
            ));
        }
        if matches!(b, b'`' | b'$') {
            return Err(format!(
                "description contains forbidden character '{}' at offset {i}",
                b as char
            ));
        }
    }
    Ok(())
}

const MAX_DESCRIPTION_LEN: usize = 200;

fn validate_crate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("crate must not be empty".to_string());
    }
    if !is_kebab_lower(name) {
        return Err(format!(
            "crate must be lowercase kebab-case (got '{name}')"
        ));
    }
    Ok(())
}

fn validate_verb(verb: &str) -> Result<(), String> {
    if verb.is_empty() {
        return Err("verb must not be empty".to_string());
    }
    if !is_kebab_lower(verb) {
        return Err(format!(
            "verb must be lowercase kebab-case (got '{verb}')"
        ));
    }
    Ok(())
}

fn validate_kind(kind: &str) -> Result<(), String> {
    match kind {
        "command" | "query" | "stream" | "transform" => Ok(()),
        other => Err(format!(
            "kind must be command|query|stream|transform (got '{other}')"
        )),
    }
}

/// Matches regex `^[a-z][a-z0-9]*(-[a-z0-9]+)*$` without pulling in `regex`.
/// Hand-rolled because it's ~10 LOC and saves a workspace-wide dep.
fn is_kebab_lower(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_lowercase() {
        return false;
    }
    let mut prev_dash = false;
    for c in chars {
        match c {
            'a'..='z' | '0'..='9' => prev_dash = false,
            '-' if !prev_dash => prev_dash = true,
            _ => return false,
        }
    }
    !prev_dash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty() {
        assert!(validate_verb("").is_err());
        assert!(validate_crate_name("").is_err());
    }

    #[test]
    fn rejects_upper() {
        assert!(validate_verb("addDependency").is_err());
    }

    #[test]
    fn rejects_trailing_dash() {
        assert!(validate_verb("add-").is_err());
    }

    #[test]
    fn rejects_double_dash() {
        assert!(validate_verb("add--dep").is_err());
    }

    #[test]
    fn accepts_kebab() {
        assert!(validate_verb("add-dependency").is_ok());
        assert!(validate_verb("search").is_ok());
        assert!(validate_verb("v2-rename").is_ok());
    }

    #[test]
    fn accepts_known_kinds() {
        for k in ["command", "query", "stream", "transform"] {
            let req = ForgeRequest {
                crate_name: "kei-task".into(),
                verb: "noop".into(),
                kind: k.into(),
                description: "test".into(),
            };
            assert!(validate(&req).is_ok(), "kind {k} should validate");
        }
    }

    #[test]
    fn rejects_unknown_kind() {
        let req = ForgeRequest {
            crate_name: "kei-task".into(),
            verb: "noop".into(),
            kind: "saga".into(),
            description: "test".into(),
        };
        assert!(validate(&req).is_err());
    }

    fn req_with_desc(d: &str) -> ForgeRequest {
        ForgeRequest {
            crate_name: "kei-task".into(),
            verb: "noop".into(),
            kind: "command".into(),
            description: d.into(),
        }
    }

    #[test]
    fn description_rejects_newline() {
        let err = validate(&req_with_desc("foo\nbar")).unwrap_err();
        assert!(err.contains("0x0A"), "expected newline byte report: {err}");
    }

    #[test]
    fn description_rejects_carriage_return() {
        assert!(validate(&req_with_desc("foo\rbar")).is_err());
    }

    #[test]
    fn description_rejects_tab() {
        assert!(validate(&req_with_desc("foo\tbar")).is_err());
    }

    #[test]
    fn description_rejects_nul() {
        assert!(validate(&req_with_desc("foo\0bar")).is_err());
    }

    #[test]
    fn description_rejects_backtick() {
        let err = validate(&req_with_desc("foo`id`bar")).unwrap_err();
        assert!(err.contains('`'), "expected backtick in error: {err}");
    }

    #[test]
    fn description_rejects_dollar_sign() {
        let err = validate(&req_with_desc("foo$(id)bar")).unwrap_err();
        assert!(err.contains('$'), "expected dollar in error: {err}");
    }

    #[test]
    fn description_rejects_over_length() {
        let long = "a".repeat(201);
        assert!(validate(&req_with_desc(&long)).is_err());
    }

    #[test]
    fn description_accepts_minimal() {
        assert!(validate(&req_with_desc("ok")).is_ok());
    }

    #[test]
    fn description_accepts_at_length_cap() {
        let exact = "a".repeat(200);
        assert!(validate(&req_with_desc(&exact)).is_ok());
    }
}
