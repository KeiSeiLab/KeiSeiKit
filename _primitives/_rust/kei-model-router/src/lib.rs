//! kei-model-router — model selection for Claude Code Agent spawns.
//!
//! Reads three TOML registries (providers / models / agent-profiles) and
//! exposes two selection surfaces:
//!
//!   - `pick(profile_id, registry)` — registry-backed profile resolution.
//!   - `select(input, conn)` — empirical posterior + cost argmin.
//!
//! Constructor Pattern: one file = one responsibility.
//! Cubes:
//!   - `registry_types` — Provider / Model / Profile TOML structs
//!   - `registry`  — Registry loader + lookup methods
//!   - `pricing`   — cost_micro_cents + legacy Model enum
//!   - `dna_class` — task-class DNA extraction
//!   - `complexity` — τ-estimator (heuristic)
//!   - `posterior`  — Beta posterior from ledger
//!   - `kernel`     — DNA similarity kernel
//!   - `select`     — pick() types + thin delegation
//!   - `select_posterior` — empirical posterior argmin logic
//!   - `select_kernel`    — SQL kernel-smoothing fallback
//!   - `escalate`   — next_model() + legacy escalation ladder
//!   - `calibrate`  — offline kernel-weight calibration

pub mod calibrate;
pub mod complexity;
pub mod dna_class;
pub mod escalate;
pub mod kernel;
pub mod posterior;
pub mod pricing;
pub mod registry;
pub mod registry_types;
pub mod select;
pub(crate) mod select_kernel;
pub(crate) mod select_posterior;

// Registry API
pub use registry::Registry;
/// `RegistryModel` is the TOML wire record from `models.toml`.
/// It is distinct from the `Model` enum (canonical tier identifier).
pub use registry_types::Model as RegistryModel;
pub use registry_types::{Profile, Provider};

// Pricing API — `Model` is the canonical model enum used for posterior/escalation.
pub use pricing::{cost_micro_cents, Model, OPUS_47_TOKENIZER_OVERHEAD};

// Selection API
pub use select::{pick, select, Decision, DecisionInput};

// Escalation API
pub use escalate::{
    next_model, next_after_failure, EscalationDecision, EscalationResult,
    MAX_ESCALATION_DEPTH,
};

// Utility re-exports
pub use complexity::{ComplexityEstimate, Tier};
pub use kernel::{similarity, KernelWeights};
pub use posterior::Posterior;
