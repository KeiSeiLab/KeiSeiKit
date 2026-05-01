//! Registry smoke tests — every declared capability name resolves; unknown
//! names return None; gate-only and verify-only capabilities route correctly.

use kei_agent_runtime::registry;

#[test]
fn all_registered_names_resolve() {
    for name in registry::all_names() {
        assert!(
            registry::get(name).is_some(),
            "registry::get({name}) returned None"
        );
    }
}

#[test]
fn unknown_names_return_none() {
    assert!(registry::get("bogus::nothing").is_none());
    assert!(registry::get_gate("bogus::nothing").is_none());
    assert!(registry::get_verify("bogus::nothing").is_none());
    assert!(registry::get("").is_none());
}

#[test]
fn gate_only_capabilities_route_to_gate_table() {
    let cap = registry::get_gate("tools::deny-tools").expect("deny-tools gate");
    assert_eq!(cap.name(), "tools::deny-tools");
    // deny-tools has no verify module — get_verify must miss
    assert!(registry::get_verify("tools::deny-tools").is_none());
}

#[test]
fn verify_only_capabilities_route_to_verify_table() {
    let cap = registry::get_verify("quality::cargo-check-green").expect("ccg verify");
    assert_eq!(cap.name(), "quality::cargo-check-green");
    assert!(registry::get_gate("quality::cargo-check-green").is_none());
}

#[test]
fn dual_capabilities_register_in_both_tables() {
    // scope::* have both gate and verify impls under the same name
    assert!(registry::get_gate("scope::files-whitelist").is_some());
    assert!(registry::get_verify("scope::files-whitelist").is_some());
    assert!(registry::get_gate("scope::files-denylist").is_some());
    assert!(registry::get_verify("scope::files-denylist").is_some());
    assert!(registry::get_gate("safety::no-dep-bump").is_some());
    assert!(registry::get_verify("safety::no-dep-bump").is_some());
}

#[test]
fn registry_total_count_matches_spec() {
    // 11 unique names in inventory; 3 of them (scope whitelist, scope
    // denylist, safety::no-dep-bump) are dual gate+verify.
    assert_eq!(registry::all_names().len(), 11);
}

/// v0.17 — deprecated aliases still resolve. Old callers querying
/// `tools::read-only` / `tools::cargo-only-bash` must land on the new
/// impl without breakage. The returned `Capability::name()` reports the
/// CANONICAL name, so callers that stored the string are now on the
/// migration path.
#[test]
fn deprecated_aliases_resolve_to_new_names() {
    for (old, new) in registry::deprecated_aliases() {
        let cap = registry::get(old)
            .unwrap_or_else(|| panic!("alias {old} must resolve to some capability"));
        assert_eq!(
            cap.name(),
            new,
            "alias {old} should resolve to impl reporting canonical name {new}"
        );
        // Old name must also work through the typed entry points so
        // hook binaries that call `get_gate` / `get_verify` directly
        // see the same resolution.
        if registry::get_gate(new).is_some() {
            assert!(
                registry::get_gate(old).is_some(),
                "alias {old} must resolve through get_gate"
            );
        }
        if registry::get_verify(new).is_some() {
            assert!(
                registry::get_verify(old).is_some(),
                "alias {old} must resolve through get_verify"
            );
        }
    }
}
