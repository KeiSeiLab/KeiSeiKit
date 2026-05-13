//! Registry loader — reads providers.toml, models.toml, agent-profiles.toml.
//!
//! Path resolution:
//!   1. `KEI_REGISTRIES_DIR` env var (if set)
//!   2. `~/Projects/KeiSeiKit-public/_blocks/registries/` (default)
//!
//! Types live in `registry_types.rs` (separate cube per Constructor Pattern).
//! This cube owns loading + lookup methods only.

use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

pub use crate::registry_types::{Model, Profile, Provider};
use crate::registry_types::{ModelsFile, ProfilesFile, ProvidersFile};

#[derive(Debug, Clone)]
pub struct Registry {
    pub providers: Vec<Provider>,
    pub models: Vec<Model>,
    pub profiles: Vec<Profile>,
}

impl Registry {
    /// Load all three TOML files from `dir`.
    pub fn load_from(dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let providers = parse_toml::<ProvidersFile>(&dir.join("providers.toml"))?.provider;
        let models = parse_toml::<ModelsFile>(&dir.join("models.toml"))?.model;
        let profiles =
            parse_toml::<ProfilesFile>(&dir.join("agent-profiles.toml"))?.profile;
        Ok(Self { providers, models, profiles })
    }

    /// Load from `KEI_REGISTRIES_DIR` or the project-default path.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from(&registries_dir())
    }

    pub fn provider_by_id(&self, id: &str) -> Option<&Provider> {
        self.providers.iter().find(|p| p.id == id)
    }

    pub fn model_by_id(&self, id: &str) -> Option<&Model> {
        self.models.iter().find(|m| m.id == id)
    }

    pub fn profile_by_id(&self, id: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.id == id)
    }

    /// All non-deprecated models for a provider, sorted by output cost ascending.
    pub fn models_for_provider(&self, provider_id: &str) -> Vec<&Model> {
        let mut ms: Vec<&Model> = self
            .models
            .iter()
            .filter(|m| m.provider_ref == provider_id && !m.is_deprecated())
            .collect();
        ms.sort_by_key(|m| m.cost_output_per_mtok_micro);
        ms
    }
}

fn registries_dir() -> PathBuf {
    if let Ok(v) = std::env::var("KEI_REGISTRIES_DIR") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(format!(
        "{home}/Projects/KeiSeiKit-public/_blocks/registries"
    ))
}

fn parse_toml<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let parsed: T = toml::from_str(&raw)
        .map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
    Ok(parsed)
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn reg() -> Registry {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()  // _rust/
            .parent().unwrap()  // _primitives/
            .parent().unwrap()  // KeiSeiKit-public/
            .join("_blocks/registries");
        Registry::load_from(&dir).expect("registry load failed")
    }

    #[test]
    fn loads_all_three_files() {
        let r = reg();
        assert!(!r.providers.is_empty(), "providers empty");
        assert!(!r.models.is_empty(), "models empty");
        assert!(!r.profiles.is_empty(), "profiles empty");
    }

    #[test]
    fn provider_by_id_anthropic() {
        let r = reg();
        let p = r.provider_by_id("anthropic").expect("anthropic missing");
        assert_eq!(p.display_name, "Anthropic");
    }

    #[test]
    fn model_by_id_sonnet() {
        let r = reg();
        let m = r.model_by_id("claude-sonnet-4-6").expect("sonnet missing");
        assert_eq!(m.provider_ref, "anthropic");
        assert_eq!(m.cost_input_per_mtok_micro, 300_000_000);
        assert_eq!(m.cost_output_per_mtok_micro, 1_500_000_000);
    }

    #[test]
    fn profile_by_id_code_implementer_rust() {
        let r = reg();
        let p = r.profile_by_id("code-implementer-rust").expect("profile missing");
        let (provider, model) = p.split_model_ref().expect("split failed");
        assert_eq!(provider, "anthropic");
        assert_eq!(model, "claude-sonnet-4-6");
    }

    #[test]
    fn models_for_provider_sorted_by_output_cost() {
        let r = reg();
        let ms = r.models_for_provider("anthropic");
        assert!(ms.len() >= 3, "expected >= 3 anthropic models");
        for w in ms.windows(2) {
            assert!(
                w[0].cost_output_per_mtok_micro <= w[1].cost_output_per_mtok_micro,
                "not sorted: {} > {}",
                w[0].id, w[1].id
            );
        }
    }

    #[test]
    fn deprecated_models_excluded_from_provider_list() {
        let r = reg();
        let ms = r.models_for_provider("anthropic");
        for m in ms {
            assert!(!m.is_deprecated(), "{} should not be deprecated", m.id);
        }
    }
}
