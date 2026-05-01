//! Verify the seed catalog loads, holds ≥12 models, and every row
//! carries either verified or placeholder pricing with a source_url
//! (RULE 0.4 invariant).

use std::path::PathBuf;

use kei_model::pricing::PricingStatus;
use kei_model::registry::Registry;

fn seed_catalog_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/models.toml")
}

#[test]
fn loads_at_least_twelve_models() {
    let reg = Registry::load(&seed_catalog_path()).expect("seed catalog must parse");
    assert!(
        reg.list_all().len() >= 12,
        "expected ≥12 seed models, got {}",
        reg.list_all().len()
    );
}

#[test]
fn pricing_invariants_per_rule_04() {
    let reg = Registry::load(&seed_catalog_path()).expect("seed catalog must parse");
    for m in reg.list_all() {
        // Every row must carry a source_url for traceability.
        assert!(
            m.pricing.source_url.is_some(),
            "model {} must carry a source_url for verification",
            m.id
        );
        match m.pricing.status {
            PricingStatus::Placeholder => {
                // Placeholder rows must NOT claim a real rate (zero or unset).
                assert_eq!(
                    m.pricing.input_per_mtok_micro, 0,
                    "model {} placeholder input rate must be 0",
                    m.id
                );
                assert_eq!(
                    m.pricing.output_per_mtok_micro, 0,
                    "model {} placeholder output rate must be 0",
                    m.id
                );
            }
            PricingStatus::Verified => {
                // Verified rows must carry a verified_at date for auditability.
                // Local-inference rows are allowed to have zero rates with verified status.
                assert!(
                    m.pricing.verified_at.is_some(),
                    "model {} verified pricing must carry verified_at date",
                    m.id
                );
            }
            PricingStatus::NeedsVerification => {
                // Transitional state — must have source_url (already asserted above).
            }
        }
    }
}

#[test]
fn covers_six_providers() {
    use kei_model::model::Provider;
    let reg = Registry::load(&seed_catalog_path()).expect("seed catalog must parse");
    let want = [
        Provider::Anthropic,
        Provider::Openai,
        Provider::Kimi,
        Provider::Mistral,
        Provider::Deepseek,
        Provider::Local,
    ];
    for p in want {
        assert!(
            !reg.by_provider(p).is_empty(),
            "missing provider in seed catalog: {}",
            p.as_str()
        );
    }
}

#[test]
fn source_path_round_trip() {
    let path = seed_catalog_path();
    let reg = Registry::load(&path).unwrap();
    assert_eq!(reg.source_path(), path);
}
