//! Filter registry by provider / cap / status / role-tag.

use std::path::PathBuf;

use kei_model::model::{Capability, Provider, Status};
use kei_model::registry::Registry;

fn reg() -> Registry {
    Registry::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/models.toml")).unwrap()
}

#[test]
fn by_provider_anthropic_returns_three() {
    let r = reg();
    let anth = r.by_provider(Provider::Anthropic);
    assert_eq!(anth.len(), 3, "expected 3 Anthropic seeds");
    for m in anth {
        assert_eq!(m.provider, Provider::Anthropic);
    }
}

#[test]
fn by_cap_vision_subset() {
    let r = reg();
    let vis = r.by_cap(Capability::Vision);
    assert!(!vis.is_empty(), "at least one model should declare vision");
    for m in vis {
        assert!(m.capabilities.contains(&Capability::Vision));
    }
}

#[test]
fn by_status_active_is_full_seed() {
    let r = reg();
    let active = r.by_status(Status::Active);
    // All seed rows are active.
    assert_eq!(active.len(), r.list_all().len());
}

#[test]
fn by_role_tag_complex_reasoning_nonempty() {
    let r = reg();
    let cr = r.by_role_tag("complex-reasoning");
    assert!(!cr.is_empty(), "complex-reasoning role tag should match ≥1 model");
}

#[test]
fn get_by_id_known_and_unknown() {
    let r = reg();
    assert!(r.get("claude-opus-4-7").is_some());
    assert!(r.get("nonexistent-model-xyz").is_none());
}
