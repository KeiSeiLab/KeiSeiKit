//! kei-model — universal model registry + selector.
//!
//! Replaces hardcoded `MODEL` / `DEFAULT_MODEL` constants in kei-cortex,
//! kei-router, and kei-spawn with a SSoT TOML catalog (`data/models.toml`)
//! plus role-default routing (`data/selectors.toml`). Pure compute, no
//! async, no network — siblings depend on this primitive, not the other way.
//!
//! ## Subcommands
//!   * `list`       — filter catalog by provider/cap/status/role
//!   * `resolve`    — pick cheapest active model for role+budget+caps
//!   * `price`      — estimate micro-cent cost for a token budget
//!   * `providers`  — list distinct providers with active/deprecated counts
//!   * `fallback`   — walk fallback chain until None or cycle
//!
//! ## RULE 0.4 — Pricing
//! All seed pricing rows ship `status = "placeholder"` with rates of 0.
//! Real provider rates land in a follow-up verification commit. Callers
//! checking cost MUST consult `pricing.status` before quoting numbers.

pub mod cli;
pub mod fallback;
pub mod model;
pub mod pricing;
pub mod registry;
pub mod selector;

pub use fallback::chain;
pub use model::{Capability, Model, Provider, Status};
pub use pricing::{estimate, Pricing, PricingStatus};
pub use registry::Registry;
pub use selector::{resolve, Resolution};
