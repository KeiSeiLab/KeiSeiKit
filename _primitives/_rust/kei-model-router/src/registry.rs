//! Registry loader — providers.toml / models.toml / agent-profiles.toml.
//!
//! Path resolution: KEI_REGISTRIES_DIR env → disk default → embedded copy.
//! Finding 4: `include_str!()` embeds TOMLs at compile time (install-safe).
//! Finding 8: HOME unset → warning + embedded fallback, no garbled path.
//! Types in `registry_types.rs` (Constructor Pattern: types separate from loader).

use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::path::{Path, PathBuf};

pub use crate::registry_types::{Model, Profile, Provider};
use crate::registry_types::{ModelsFile, ProfilesFile, ProvidersFile};

/// User-tier override: что пользователь выбрал в onboarding мастере.
/// Парсится из `~/.claude/config/user-model-override.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct UserModelOverride {
    pub provider: String,
    pub model: String,
    #[serde(default)]
    pub transport: Option<String>,
}

// Embedded compile-time copies. Cargo tracks these as implicit dependencies:
// if the TOML changes, the crate is recompiled automatically.
const EMBEDDED_PROVIDERS: &str =
    include_str!("../../../../_blocks/registries/providers.toml");
const EMBEDDED_MODELS: &str =
    include_str!("../../../../_blocks/registries/models.toml");
const EMBEDDED_PROFILES: &str =
    include_str!("../../../../_blocks/registries/agent-profiles.toml");

#[derive(Debug, Clone)]
pub struct Registry {
    pub providers: Vec<Provider>,
    pub models: Vec<Model>,
    pub profiles: Vec<Profile>,
}

impl Registry {
    /// Load from `dir` on disk.
    pub fn load_from(dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            providers: parse_toml::<ProvidersFile>(&dir.join("providers.toml"))?.provider,
            models: parse_toml::<ModelsFile>(&dir.join("models.toml"))?.model,
            profiles: parse_toml::<ProfilesFile>(&dir.join("agent-profiles.toml"))?.profile,
        })
    }

    /// Load: KEI_REGISTRIES_DIR → disk default → embedded fallback.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(dir) = std::env::var("KEI_REGISTRIES_DIR") {
            return Self::load_from(&PathBuf::from(dir));
        }
        match disk_registries_dir() {
            Some(dir) if dir.exists() => Self::load_from(&dir),
            Some(_) | None => {
                if std::env::var("HOME").unwrap_or_default().is_empty() {
                    eprintln!("[kei-model-router] HOME unset; using embedded registry");
                }
                Self::load_embedded()
            }
        }
    }

    /// Parse the compile-time embedded TOML constants.
    pub fn load_embedded() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            providers: toml::from_str::<ProvidersFile>(EMBEDDED_PROVIDERS)
                .map_err(|e| format!("embedded providers.toml: {e}"))?.provider,
            models: toml::from_str::<ModelsFile>(EMBEDDED_MODELS)
                .map_err(|e| format!("embedded models.toml: {e}"))?.model,
            profiles: toml::from_str::<ProfilesFile>(EMBEDDED_PROFILES)
                .map_err(|e| format!("embedded agent-profiles.toml: {e}"))?.profile,
        })
    }

    /// Загружает user-tier override из `~/.claude/config/user-model-override.toml`.
    /// Файл пишется установщиком (install/lib-onboarding.sh::onboarding_write_config)
    /// при первичной настройке. Содержит выбор юзера: provider/model/transport.
    ///
    /// Priority в router: `--pinned` flag > этот файл > agent-profiles.toml::default_model_ref.
    /// Без него выбор провайдера в onboarding декоративен (HIGH аудит-1, 2026-05-17).
    pub fn load_user_override() -> Option<UserModelOverride> {
        let home = std::env::var("HOME").ok()?;
        if home.is_empty() {
            return None;
        }
        let path = PathBuf::from(format!("{home}/.claude/config/user-model-override.toml"));
        if !path.exists() {
            return None;
        }
        let raw = std::fs::read_to_string(&path).ok()?;
        toml::from_str::<UserModelOverride>(&raw).ok()
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

/// Returns the disk path derived from HOME, or None if HOME is empty/unset.
fn disk_registries_dir() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    if home.is_empty() {
        return None;
    }
    Some(PathBuf::from(format!(
        "{home}/Projects/KeiSeiKit-public/_blocks/registries"
    )))
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
        // После RULE 0.26 transport-расширения display_name стало
        // "Anthropic (Direct API)" чтобы отличать от "anthropic-bedrock".
        assert!(
            p.display_name.starts_with("Anthropic"),
            "got: {}", p.display_name
        );
    }

    #[test]
    fn user_override_parses_minimal_toml() {
        // Round-trip: TOML → UserModelOverride.
        let toml_src = r#"
            provider = "ollama-local"
            model = "llama-3.3-70b"
            transport = "local"
        "#;
        let ov: UserModelOverride = toml::from_str(toml_src).expect("parse failed");
        assert_eq!(ov.provider, "ollama-local");
        assert_eq!(ov.model, "llama-3.3-70b");
        assert_eq!(ov.transport.as_deref(), Some("local"));
    }

    #[test]
    fn user_override_transport_optional() {
        // transport — optional поле.
        let toml_src = r#"
            provider = "openai"
            model = "gpt-5"
        "#;
        let ov: UserModelOverride = toml::from_str(toml_src).expect("parse failed");
        assert_eq!(ov.provider, "openai");
        assert!(ov.transport.is_none());
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

    /// Finding 4: embedded registry must parse cleanly and match disk.
    #[test]
    fn embedded_registry_matches_disk() {
        let disk = reg();
        let emb = Registry::load_embedded().expect("embedded parse failed");
        assert_eq!(disk.models.len(), emb.models.len(),
            "disk and embedded model count differ");
        assert_eq!(disk.providers.len(), emb.providers.len(),
            "disk and embedded provider count differ");
        assert_eq!(disk.profiles.len(), emb.profiles.len(),
            "disk and embedded profile count differ");
    }
}
