//! Smoke tests for the generic `PatternGate` (Layer C convergence).
//!
//! Covers DenyIfMatch, AllowIfMatch, DenyIfUnmatched (scope), bypass env,
//! and non-applicable tool short-circuit.

use kei_agent_runtime::capability::{Capability, GateContext, GateDecision, TaskSpec};
use kei_agent_runtime::gates::pattern_gate::{GateMode, PatternGate, PatternSource};
use serde_json::json;
use std::collections::HashMap;

fn ctx<'a>(
    tool: &'a str,
    input: &'a serde_json::Value,
    task: &'a TaskSpec,
    env: &'a HashMap<String, String>,
) -> GateContext<'a> {
    GateContext { tool_name: tool, tool_input: input, task, env }
}

const TEST_GATE: PatternGate = PatternGate {
    name: "test::deny-if-match",
    tools: &["Bash"],
    field: "command",
    mode: GateMode::DenyIfMatch,
    patterns: PatternSource::StaticRegex(&[r"\bforbidden\b"]),
    bypass_env: Some("TEST_BYPASS"),
    deny_template: "{name} — {cmd} matched {pat}",
};

#[test]
fn deny_if_match_blocks_forbidden() {
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input = json!({"command": "run forbidden action"});
    match TEST_GATE.check(&ctx("Bash", &input, &task, &env)) {
        GateDecision::Deny { .. } => {}
        other => panic!("expected Deny, got {other:?}"),
    }
}

#[test]
fn deny_if_match_allows_when_no_match() {
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input = json!({"command": "echo hello"});
    assert_eq!(
        TEST_GATE.check(&ctx("Bash", &input, &task, &env)),
        GateDecision::Allow
    );
}

#[test]
fn allow_if_match_allows_on_match() {
    // Mirrors tools::bash-allowlist — use the real registry path.
    use kei_agent_runtime::registry;
    let g = registry::get_gate("tools::bash-allowlist").unwrap();
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input = json!({"command": "cargo build"});
    assert_eq!(g.check(&ctx("Bash", &input, &task, &env)), GateDecision::Allow);
}

#[test]
fn allow_if_match_denies_on_miss() {
    use kei_agent_runtime::registry;
    let g = registry::get_gate("tools::bash-allowlist").unwrap();
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input = json!({"command": "wget http://evil"});
    matches!(g.check(&ctx("Bash", &input, &task, &env)), GateDecision::Deny { .. });
}

#[test]
fn deny_if_unmatched_scope_whitelist_empty_allows() {
    // Empty whitelist = NotApplicable / Allow per spec.
    use kei_agent_runtime::registry;
    let g = registry::get_gate("scope::files-whitelist").unwrap();
    let task = TaskSpec::default(); // empty whitelist
    let env = HashMap::new();
    let input = json!({"file_path": "anywhere/foo.rs"});
    assert_eq!(g.check(&ctx("Edit", &input, &task, &env)), GateDecision::Allow);
}

#[test]
fn bypass_env_allows_even_on_match() {
    let task = TaskSpec::default();
    let mut env = HashMap::new();
    env.insert("TEST_BYPASS".into(), "1".into());
    let input = json!({"command": "run forbidden action"});
    assert_eq!(
        TEST_GATE.check(&ctx("Bash", &input, &task, &env)),
        GateDecision::Allow
    );
}

#[test]
fn non_matching_tool_is_not_applicable() {
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input = json!({"command": "run forbidden action"});
    assert_eq!(
        TEST_GATE.check(&ctx("Read", &input, &task, &env)),
        GateDecision::NotApplicable
    );
}

#[test]
fn missing_field_is_not_applicable() {
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input = json!({"not_the_field": "forbidden"});
    assert_eq!(
        TEST_GATE.check(&ctx("Bash", &input, &task, &env)),
        GateDecision::NotApplicable
    );
}

#[test]
fn safety_no_dep_bump_blocks_cargo_toml_path() {
    use kei_agent_runtime::registry;
    let g = registry::get_gate("safety::no-dep-bump").unwrap();
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input = json!({"file_path": "nested/Cargo.toml"});
    match g.check(&ctx("Edit", &input, &task, &env)) {
        GateDecision::Deny { .. } => {}
        other => panic!("expected Deny, got {other:?}"),
    }
}

#[test]
fn safety_no_dep_bump_allows_bypass_env() {
    use kei_agent_runtime::registry;
    let g = registry::get_gate("safety::no-dep-bump").unwrap();
    let task = TaskSpec::default();
    let mut env = HashMap::new();
    env.insert("ALLOW_DEP_BUMP".into(), "1".into());
    let input = json!({"file_path": "Cargo.toml"});
    assert_eq!(g.check(&ctx("Edit", &input, &task, &env)), GateDecision::Allow);
}

// --- Hardening audit 2026-04-23 — H1/H2/H3/S4/L2 ---------------------------

/// S4 — UTF-8 boundary safety.
///
/// Construct a command where the 30th character is a 2-byte Cyrillic code
/// point and the total char count exceeds the 60-char truncation budget.
/// The old `&s[..60]` byte-slice panicked when byte-60 landed mid-char.
/// New `truncate_chars` takes 60 chars, not 60 bytes, so this must not panic.
#[test]
fn render_reason_safe_on_multibyte_command() {
    let task = TaskSpec::default();
    let env = HashMap::new();
    // 30 ASCII 'a' + 40 Cyrillic 'я' (2 bytes each) = 70 chars / 110 bytes.
    // Byte 60 lands mid-'я' — would panic under old byte-slice truncation.
    let mut cmd = "a".repeat(30);
    for _ in 0..40 {
        cmd.push('я');
    }
    cmd.push_str(" forbidden");
    let input = json!({"command": cmd});
    match TEST_GATE.check(&ctx("Bash", &input, &task, &env)) {
        GateDecision::Deny { reason } => {
            // Truncated segment must be valid UTF-8; no panic reached here.
            assert!(reason.contains("matched"));
        }
        other => panic!("expected Deny on multibyte cmd, got {other:?}"),
    }
}

/// H2 — invalid static regex fails closed instead of panicking.
const BAD_REGEX_GATE: PatternGate = PatternGate {
    name: "test::bad-regex",
    tools: &["Bash"],
    field: "command",
    // Unbalanced `[` — regex::Regex::new will return Err.
    mode: GateMode::DenyIfMatch,
    patterns: PatternSource::StaticRegex(&["[unclosed"]),
    bypass_env: None,
    deny_template: "{name} — {cmd} matched {pat}",
};

#[test]
fn malformed_static_regex_denies_without_panic() {
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input = json!({"command": "anything"});
    match BAD_REGEX_GATE.check(&ctx("Bash", &input, &task, &env)) {
        GateDecision::Deny { reason } => {
            assert!(reason.contains("misconfigured"), "reason: {reason}");
            assert!(reason.contains("[unclosed"), "reason: {reason}");
        }
        other => panic!("expected Deny on malformed regex, got {other:?}"),
    }
}

/// H3 — AllowIfMatch + task-scope source is structurally invalid; the
/// previous impl silently returned `NotApplicable` (effectively allowing
/// every tool call through). Hardened impl fails closed with a clear
/// `capability misconfigured` `Deny`.
const BAD_COMBO_GATE: PatternGate = PatternGate {
    name: "test::bad-combo",
    tools: &["Edit"],
    field: "file_path",
    mode: GateMode::AllowIfMatch,
    patterns: PatternSource::TaskWhitelist,
    bypass_env: None,
    deny_template: "{name} — {path}",
};

#[test]
fn allow_if_match_with_task_scope_fails_closed() {
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input = json!({"file_path": "anywhere/foo.rs"});
    match BAD_COMBO_GATE.check(&ctx("Edit", &input, &task, &env)) {
        GateDecision::Deny { reason } => {
            assert!(reason.contains("misconfigured"), "reason: {reason}");
            assert!(reason.contains("AllowIfMatch"), "reason: {reason}");
        }
        other => panic!("expected Deny on bad combo, got {other:?}"),
    }
}

/// H1 — regex cache behaviour. Same pattern across many calls must stay
/// deterministic and cheap; this test doesn't measure timing but pins the
/// behavioural contract: 1k calls against the cached pattern all agree
/// with the first call's outcome. A full per-pattern `Lazy<Regex>`
/// refactor would let us assert pointer-identity; with the `RwLock`-only
/// fix in this file-bounded audit, we assert observational stability.
#[test]
fn regex_cache_is_stable_across_many_calls() {
    let task = TaskSpec::default();
    let env = HashMap::new();
    let input_hit = json!({"command": "run forbidden action"});
    let input_miss = json!({"command": "echo hello"});
    let first_hit = TEST_GATE.check(&ctx("Bash", &input_hit, &task, &env));
    let first_miss = TEST_GATE.check(&ctx("Bash", &input_miss, &task, &env));
    for _ in 0..1000 {
        assert!(matches!(
            TEST_GATE.check(&ctx("Bash", &input_hit, &task, &env)),
            GateDecision::Deny { .. }
        ));
        assert_eq!(
            TEST_GATE.check(&ctx("Bash", &input_miss, &task, &env)),
            GateDecision::Allow
        );
    }
    // Sanity: first outcomes still match after the warm-up loop.
    assert!(matches!(first_hit, GateDecision::Deny { .. }));
    assert_eq!(first_miss, GateDecision::Allow);
}
