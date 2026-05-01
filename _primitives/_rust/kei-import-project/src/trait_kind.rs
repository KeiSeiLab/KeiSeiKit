//! Trait-kind parsing + enumeration helpers.
//!
//! Extracted from `trait_patterns.rs` to keep that file under the
//! Constructor Pattern 200-LOC ceiling. The 12 patterns + their
//! supporting structs live in `trait_patterns.rs`; this module owns
//! the case-insensitive `from_str_ci` parser and the `all()`
//! enumeration helper.

use crate::trait_patterns::TraitKind;

impl TraitKind {
    /// Parse a case-insensitive kebab-case name into a `TraitKind`.
    pub fn from_str_ci(s: &str) -> Option<TraitKind> {
        match s.to_lowercase().replace('-', "").as_str() {
            "computeprovider" | "compute" => Some(TraitKind::ComputeProvider),
            "authprovider" | "auth" => Some(TraitKind::AuthProvider),
            "notifychannel" | "notify" => Some(TraitKind::NotifyChannel),
            "gitbackend" | "git" => Some(TraitKind::GitBackend),
            "llmbackend" | "llm" => Some(TraitKind::LlmBackend),
            "servicemanager" | "service" => Some(TraitKind::ServiceManager),
            "memorybackend" | "memory" => Some(TraitKind::MemoryBackend),
            "scheduler" => Some(TraitKind::Scheduler),
            "networkmode" | "network" => Some(TraitKind::NetworkMode),
            "backup" => Some(TraitKind::Backup),
            "costguard" | "cost" => Some(TraitKind::CostGuard),
            "observability" => Some(TraitKind::Observability),
            _ => None,
        }
    }

    /// All 12 trait kinds in definition order.
    pub fn all() -> &'static [TraitKind] {
        ALL_KINDS
    }
}

static ALL_KINDS: &[TraitKind] = &[
    TraitKind::ComputeProvider,
    TraitKind::AuthProvider,
    TraitKind::NotifyChannel,
    TraitKind::GitBackend,
    TraitKind::LlmBackend,
    TraitKind::ServiceManager,
    TraitKind::MemoryBackend,
    TraitKind::Scheduler,
    TraitKind::NetworkMode,
    TraitKind::Backup,
    TraitKind::CostGuard,
    TraitKind::Observability,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_ci_kebab() {
        assert_eq!(TraitKind::from_str_ci("compute-provider"), Some(TraitKind::ComputeProvider));
        assert_eq!(TraitKind::from_str_ci("memory-backend"), Some(TraitKind::MemoryBackend));
    }

    #[test]
    fn from_str_ci_short_alias() {
        assert_eq!(TraitKind::from_str_ci("compute"), Some(TraitKind::ComputeProvider));
        assert_eq!(TraitKind::from_str_ci("auth"), Some(TraitKind::AuthProvider));
    }

    #[test]
    fn from_str_ci_unknown() {
        assert_eq!(TraitKind::from_str_ci("nope"), None);
    }

    #[test]
    fn all_returns_12() {
        assert_eq!(TraitKind::all().len(), 12);
    }
}
