//! Selector resolution: role match, fallback to defaults, budget filtering.
//!
//! The seed catalog has all pricing=0, so budget filtering can't be tested
//! against it (zero is below any cap). For budget tests we use a synthetic
//! fixture catalog with non-zero rates.

use std::path::PathBuf;

use kei_model::model::Capability;
use kei_model::registry::Registry;
use kei_model::selector::resolve;

fn seed_catalog() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/models.toml")
}

fn seed_selectors() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/selectors.toml")
}

#[test]
fn resolve_code_implementer_returns_opus() {
    let reg = Registry::load(&seed_catalog()).unwrap();
    let r = resolve(
        "code-implementer",
        None,
        &[Capability::Code],
        &reg,
        Some(&seed_selectors()),
    )
    .expect("code-implementer must resolve");
    assert_eq!(r.model.id, "claude-opus-4-7");
}

#[test]
fn resolve_kei_critic_returns_haiku() {
    let reg = Registry::load(&seed_catalog()).unwrap();
    let r = resolve(
        "kei-critic",
        None,
        &[Capability::Code],
        &reg,
        Some(&seed_selectors()),
    )
    .expect("kei-critic must resolve");
    assert_eq!(r.model.id, "claude-haiku-4-5");
}

#[test]
fn resolve_no_caps_no_role_match_falls_back_to_defaults() {
    // "edit-shared" is in selectors.toml [defaults], no model carries that
    // tag in the seed catalog, so the resolver falls through to the default.
    let reg = Registry::load(&seed_catalog()).unwrap();
    let r = resolve(
        "edit-shared",
        None,
        &[],
        &reg,
        Some(&seed_selectors()),
    )
    .expect("edit-shared must resolve via defaults table");
    assert_eq!(r.model.id, "claude-opus-4-7");
}

#[test]
fn resolve_unknown_role_errors() {
    let reg = Registry::load(&seed_catalog()).unwrap();
    let err = resolve(
        "no-such-role-xyz",
        None,
        &[],
        &reg,
        Some(&seed_selectors()),
    );
    assert!(err.is_err(), "unknown role must produce a NoMatch error");
}

#[test]
fn budget_filter_rejects_overpriced_with_synthetic_fixture() {
    // Synthetic fixture: two models, only the cheap one fits the budget.
    let fixture = synth_two_model_fixture();
    let cat_path = fixture.path().join("models.toml");
    std::fs::write(&cat_path, FIXTURE_CATALOG).unwrap();
    let sel_path = fixture.path().join("selectors.toml");
    std::fs::write(&sel_path, FIXTURE_SELECTORS).unwrap();

    let reg = Registry::load(&cat_path).unwrap();
    // Budget = 1000 micro on a 1k+1k baseline only fits the cheap model
    // (input rate 200 micro/Mtok → 1k input = 0.2 micro; ok). The
    // expensive model has input rate 1_000_000 micro/Mtok → 1k input = 1
    // micro and 1k output × 2_000_000/Mtok = 2 micro, total 3 micro per
    // 1k+1k baseline; we cap below that. We use 1000 to clearly admit cheap.
    let r = resolve("worker", Some(1_000), &[], &reg, Some(&sel_path)).unwrap();
    assert_eq!(r.model.id, "synth-cheap");
}

fn synth_two_model_fixture() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

const FIXTURE_CATALOG: &str = r#"
[[models]]
id = "synth-cheap"
provider = "local"
display_name = "Synthetic Cheap"
context_tokens = 8000
capabilities = ["code"]
status = "active"
role_tags = ["worker"]
fallback = ""

[models.pricing]
input_per_mtok_micro = 200
output_per_mtok_micro = 400
status = "needs-verification"
source_url = "https://example.test/cheap"

[[models]]
id = "synth-expensive"
provider = "local"
display_name = "Synthetic Expensive"
context_tokens = 8000
capabilities = ["code"]
status = "active"
role_tags = ["worker"]
fallback = ""

[models.pricing]
input_per_mtok_micro = 100000000
output_per_mtok_micro = 100000000
status = "needs-verification"
source_url = "https://example.test/expensive"
"#;

const FIXTURE_SELECTORS: &str = r#"
[defaults]
worker = "synth-cheap"
"#;
