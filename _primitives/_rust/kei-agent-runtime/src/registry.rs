//! Registry — `&str → &'static dyn Capability` lookup for all 14
//! capability implementations.
//!
//! `get(name)` is the single dispatch point used by both the
//! `kei-agent-runtime verify` binary and the `kei-capability` hook adapter.
//!
//! ## Aliases (v0.17)
//!
//! Two capabilities were renamed in v0.17 for clarity. Their old names
//! still resolve here via a small alias table; a deprecation warning is
//! emitted to stderr on lookup (once per process via `OnceLock`).
//!
//! - `tools::read-only` → `tools::deny-tools`
//! - `tools::cargo-only-bash` → `tools::bash-allowlist`
//!
//! Alias resolution is transparent: `get()` / `get_gate()` / `get_verify()`
//! return the new implementation when queried with the old name. The new
//! name is what the impl reports via `Capability::name()`.
//!
//! ## Convergence wave v0.18
//!
//! 5 of 6 gates + 3 of 8 verifies are now `const PatternGate { … }` /
//! `const CommandVerify { … }` declarations. Registry points at the
//! const by reference (`&POLICY_NO_GIT_OPS_GATE`) — same `&'static dyn Capability`
//! dispatch shape as before.

use crate::capability::Capability;
use crate::gates;
use crate::verifies;
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

/// Alias table — (old name → new name). Checked before every resolution.
/// v0.17 renames: `tools::read-only` and `tools::cargo-only-bash`.
fn alias_target(name: &str) -> Option<&'static str> {
    match name {
        "tools::read-only" => Some("tools::deny-tools"),
        "tools::cargo-only-bash" => Some("tools::bash-allowlist"),
        _ => None,
    }
}

/// Resolve an alias (if any) and emit a one-shot deprecation warning.
/// Returns the canonical name the caller should look up.
fn resolve_alias(name: &str) -> &str {
    match alias_target(name) {
        Some(target) => {
            warn_deprecated_once(name, target);
            target
        }
        None => name,
    }
}

/// Log a deprecation warning to stderr at most once per (old, new) pair
/// per process. Non-fatal; aliases still resolve.
fn warn_deprecated_once(old: &str, new: &str) {
    static SEEN: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    let seen = SEEN.get_or_init(|| Mutex::new(HashSet::new()));
    let mut guard = match seen.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    if guard.insert(old.to_string()) {
        eprintln!(
            "[kei-agent-runtime] deprecation: capability `{old}` is an alias for `{new}` (v0.17); \
             update your role.toml / task.toml to use the new name. Alias retained through v2."
        );
    }
}

/// Look up a capability by its canonical `<category>::<slug>` name.
/// Returns `None` if the name is unknown. Gate-only and verify-only
/// capabilities share the same name; registry returns the *gate* impl for
/// 6 capabilities that have gates, and the *verify* impl for 8 that have
/// verifies. The two lookups below partition cleanly — no name holds both
/// a gate and a verify in this phase's inventory.
///
/// Deprecated aliases (see module docs) are resolved transparently and
/// a one-shot stderr warning is emitted.
pub fn get(name: &str) -> Option<&'static dyn Capability> {
    let canonical = resolve_alias(name);
    if let Some(c) = get_gate_canonical(canonical) {
        return Some(c);
    }
    get_verify_canonical(canonical)
}

/// Look up only the gate-side impl. Used by `kei-capability check`.
/// Aliases resolve transparently.
pub fn get_gate(name: &str) -> Option<&'static dyn Capability> {
    let canonical = resolve_alias(name);
    get_gate_canonical(canonical)
}

/// Look up only the verify-side impl. Used by `kei-capability verify`.
/// Aliases resolve transparently.
pub fn get_verify(name: &str) -> Option<&'static dyn Capability> {
    let canonical = resolve_alias(name);
    get_verify_canonical(canonical)
}

/// Gate-only lookup by canonical name (no alias resolution, no warning).
fn get_gate_canonical(name: &str) -> Option<&'static dyn Capability> {
    static TOOLS_DENY_TOOLS: gates::tools_deny_tools::DenyTools = gates::tools_deny_tools::DenyTools;
    match name {
        "policy::no-git-ops" => Some(&gates::policy_no_git_ops::NO_GIT_OPS),
        "scope::files-whitelist" => Some(&gates::scope_files_whitelist::FILES_WHITELIST),
        "scope::files-denylist" => Some(&gates::scope_files_denylist::FILES_DENYLIST),
        "safety::no-dep-bump" => Some(&gates::safety_no_dep_bump::NO_DEP_BUMP_GATE),
        "tools::deny-tools" => Some(&TOOLS_DENY_TOOLS),
        "tools::bash-allowlist" => Some(&gates::tools_bash_allowlist::BASH_ALLOWLIST),
        _ => None,
    }
}

/// Verify-only lookup by canonical name (no alias resolution, no warning).
fn get_verify_canonical(name: &str) -> Option<&'static dyn Capability> {
    static CP: verifies::quality_constructor_pattern::ConstructorPattern =
        verifies::quality_constructor_pattern::ConstructorPattern;
    static WL_V: verifies::scope_files_whitelist::FilesWhitelistVerify =
        verifies::scope_files_whitelist::FilesWhitelistVerify;
    static DL_V: verifies::scope_files_denylist::FilesDenylistVerify =
        verifies::scope_files_denylist::FilesDenylistVerify;
    static RF: verifies::output_report_format::ReportFormat =
        verifies::output_report_format::ReportFormat;
    static SG: verifies::output_severity_grade::SeverityGrade =
        verifies::output_severity_grade::SeverityGrade;
    match name {
        "quality::constructor-pattern" => Some(&CP),
        "quality::cargo-check-green" => Some(&verifies::quality_cargo_check_green::CARGO_CHECK_GREEN),
        "quality::tests-green" => Some(&verifies::quality_tests_green::TESTS_GREEN),
        "safety::no-dep-bump" => Some(&verifies::safety_no_dep_bump::NO_DEP_BUMP_VERIFY),
        "scope::files-whitelist" => Some(&WL_V),
        "scope::files-denylist" => Some(&DL_V),
        "output::report-format" => Some(&RF),
        "output::severity-grade" => Some(&SG),
        _ => None,
    }
}

/// All known canonical capability names (union of gate + verify). Used by
/// smoke tests. Deprecated aliases are NOT included — see `deprecated_aliases()`.
pub fn all_names() -> Vec<&'static str> {
    vec![
        "policy::no-git-ops",
        "scope::files-whitelist",
        "scope::files-denylist",
        "safety::no-dep-bump",
        "tools::deny-tools",
        "tools::bash-allowlist",
        "quality::constructor-pattern",
        "quality::cargo-check-green",
        "quality::tests-green",
        "output::report-format",
        "output::severity-grade",
    ]
}

/// List of (old-name, new-name) pairs still honored as aliases. Used by
/// smoke tests to assert every deprecated name still resolves.
pub fn deprecated_aliases() -> Vec<(&'static str, &'static str)> {
    vec![
        ("tools::read-only", "tools::deny-tools"),
        ("tools::cargo-only-bash", "tools::bash-allowlist"),
    ]
}
