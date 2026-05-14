//! TOML wire types for the three registry files.
//!
//! One module per layer (providers, models, profiles). Kept separate from
//! Registry loading logic so the struct definitions are easy to navigate.
//!
//! Constructor Pattern: types-before-implementation. This cube defines
//! WHAT; `registry.rs` defines HOW to load and look them up.

use serde::Deserialize;

// ──────────────────────────────────────────────────────────────────────────────
// Layer 1: providers.toml
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct Provider {
    pub id: String,
    pub display_name: String,
    pub endpoint: String,
    pub auth_scheme: String,
    pub auth_env: String,
    pub retry_max: u32,
    pub retry_backoff_ms: u32,
    pub rate_limit_rpm: u32,
    pub billing_currency: String,
    pub notes: String,
    #[serde(default)]
    pub api_version_header: String,
    #[serde(default)]
    pub api_version_value: String,
}

// ──────────────────────────────────────────────────────────────────────────────
// Layer 2: models.toml
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct Model {
    pub provider_ref: String,
    pub id: String,
    pub slug: String,
    pub display_name: String,
    pub context_window: u64,
    /// Microcents per 1M input tokens. Aligns with kei-ledger.cost_micro_cents.
    pub cost_input_per_mtok_micro: u64,
    /// Microcents per 1M output tokens.
    pub cost_output_per_mtok_micro: u64,
    pub cache_write_5m_per_mtok_micro: u64,
    pub cache_read_per_mtok_micro: u64,
    #[serde(default)]
    pub verified_at: String,
    /// Empty string = live. Non-empty = deprecated since that date.
    #[serde(default)]
    pub deprecated_at: String,
    #[serde(default)]
    pub notes: String,
}

impl Model {
    /// True when this model should be excluded from new invocations.
    pub fn is_deprecated(&self) -> bool {
        !self.deprecated_at.is_empty()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Layer 3: agent-profiles.toml
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub id: String,
    pub role: String,
    pub caps: String,
    /// Format: `<provider_id>/<model_id>`, e.g. `anthropic/claude-sonnet-4-6`.
    pub default_model_ref: String,
    pub description: String,
    #[serde(default)]
    pub manifest_path: String,
}

impl Profile {
    /// Split `default_model_ref` into `(provider_id, model_id)`.
    /// Returns `None` if the format is not `<provider>/<model>`.
    pub fn split_model_ref(&self) -> Option<(&str, &str)> {
        self.default_model_ref.split_once('/')
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// TOML envelope types (package-private; only used by registry.rs loader)
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub(crate) struct ProvidersFile {
    pub provider: Vec<Provider>,
}

#[derive(Deserialize)]
pub(crate) struct ModelsFile {
    pub model: Vec<Model>,
}

#[derive(Deserialize)]
pub(crate) struct ProfilesFile {
    pub profile: Vec<Profile>,
}
