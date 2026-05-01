//! Synthetic pricing × token budgets. RULE 0.4 — these are deliberately
//! unreal numbers so no test asserts a real provider rate.

use kei_model::pricing::{estimate, Pricing, PricingStatus};

fn synth(in_rate: u64, out_rate: u64) -> Pricing {
    Pricing {
        input_per_mtok_micro: in_rate,
        output_per_mtok_micro: out_rate,
        status: PricingStatus::NeedsVerification,
        source_url: None,
        verified_at: None,
    }
}

#[test]
fn placeholder_zero_returns_zero() {
    let p = synth(0, 0);
    assert_eq!(estimate(&p, 12_345, 67_890), 0);
}

#[test]
fn one_million_input_at_one_micro_per_mtok_yields_one() {
    let p = synth(1, 0);
    assert_eq!(estimate(&p, 1_000_000, 0), 1);
}

#[test]
fn known_synthetic_round_number() {
    // 100k input @ 1000 micro / Mtok = 100 micro
    // 200k output @ 2000 micro / Mtok = 400 micro
    // total = 500 micro
    let p = synth(1_000, 2_000);
    assert_eq!(estimate(&p, 100_000, 200_000), 500);
}

#[test]
fn zero_tokens_zero_cost_even_with_rates() {
    let p = synth(1_000_000, 5_000_000);
    assert_eq!(estimate(&p, 0, 0), 0);
}

#[test]
fn saturating_does_not_panic_on_huge_inputs() {
    // u64 saturating behaviour: a*b clamps at u64::MAX, then divides.
    // We just want absence of panic and a finite result.
    let p = synth(u64::MAX, 0);
    let _ = estimate(&p, u64::MAX, 0);
}
