//! trait_patterns — static dictionary of kei-runtime-core trait signatures.
//!
//! Each `TraitPattern` describes one runtime trait by its required method
//! names, optional forbidden-dep cues, and indicator keywords that raise
//! confidence even when method names are abbreviated or wrapped.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

// ─────────────────────────── public types ──────────────────────────────────

/// Every trait defined in kei-runtime-core.
///
/// 11 traits found in kei-runtime-core/src/traits/:
/// compute, auth, notify, git, llm, service, memory, scheduler, network,
/// backup, cost, observability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum TraitKind {
    /// `ComputeProvider` — VM lifecycle (create/destroy/status/resize).
    ComputeProvider,
    /// `AuthProvider` — identity challenge/verify/revoke.
    AuthProvider,
    /// `NotifyChannel` — push notifications (send).
    NotifyChannel,
    /// `GitBackend` — repo operations (clone/push/mirror/ensure_repo).
    GitBackend,
    /// `LlmBackend` — text completion (complete/context_window).
    LlmBackend,
    /// `ServiceManager` — OS service lifecycle (install/start/stop/status).
    ServiceManager,
    /// `MemoryBackend` — key-value memory store (store/query/compact).
    MemoryBackend,
    /// `Scheduler` — cron / one-shot task registration (register/cancel/list).
    Scheduler,
    /// `NetworkMode` — VPN / tunnel management (configure/teardown/peers).
    NetworkMode,
    /// `Backup` — snapshot push/restore/prune.
    Backup,
    /// `CostGuard` — spend tracking and hard-kill budget management.
    CostGuard,
    /// `Observability` — structured log + metric emission (log/metric/flush).
    Observability,
}

/// Static description of one runtime trait's detection fingerprint.
pub struct TraitPattern {
    pub kind: TraitKind,
    /// Method names that MUST appear in the source for a confident match.
    pub required_methods: &'static [&'static str],
    /// Crate names in `use` paths that disqualify this pattern (set to `[]`
    /// unless a dep clash is known to cause false positives).
    pub forbidden_deps: &'static [&'static str],
    /// Free-text keywords in source that raise keyword-component confidence.
    pub indicator_keywords: &'static [&'static str],
}

// ─────────────────────────── pattern table ──────────────────────────────────

/// All 12 trait patterns (11 actual traits + kei-runtime-core has 11 traits;
/// count reflects actual discovered trait count = 11).
pub fn all_patterns() -> &'static [TraitPattern] {
    PATTERNS
}

static PATTERNS: &[TraitPattern] = &[
    TraitPattern {
        kind: TraitKind::ComputeProvider,
        required_methods: &["create", "destroy", "status", "provider_name",
                             "cost_per_hour_microcents"],
        forbidden_deps: &[],
        indicator_keywords: &["VmSpec", "VmHandle", "VmStatus", "compute",
                               "provision", "region", "tier", "cloud"],
    },
    TraitPattern {
        kind: TraitKind::AuthProvider,
        required_methods: &["issue_challenge", "verify", "revoke", "is_passwordless"],
        forbidden_deps: &[],
        indicator_keywords: &["AuthChallenge", "AuthSession", "oauth", "oidc",
                               "passwordless", "magic_link", "webauthn"],
    },
    TraitPattern {
        kind: TraitKind::NotifyChannel,
        required_methods: &["send", "channel_name", "supports_batching"],
        forbidden_deps: &[],
        indicator_keywords: &["Notification", "NotifySeverity", "telegram",
                               "discord", "slack", "sms", "email", "notify"],
    },
    TraitPattern {
        kind: TraitKind::GitBackend,
        required_methods: &["ensure_repo", "clone", "push", "mirror",
                             "supports_auto_create"],
        forbidden_deps: &[],
        indicator_keywords: &["GitRemote", "CommitMeta", "forgejo", "gitea",
                               "gitlab", "bitbucket", "GitAuthKind"],
    },
    TraitPattern {
        kind: TraitKind::LlmBackend,
        required_methods: &["complete", "backend_name", "model_name",
                             "pricing_per_mtok", "context_window",
                             "supports_caching", "supports_batch"],
        forbidden_deps: &[],
        indicator_keywords: &["CompletionOpts", "CompletionResponse", "ollama",
                               "llamacpp", "mlx", "openai", "anthropic",
                               "tokens_input", "tokens_output"],
    },
    TraitPattern {
        kind: TraitKind::ServiceManager,
        required_methods: &["install", "uninstall", "start", "stop",
                             "status", "enable_at_boot", "manager_name"],
        forbidden_deps: &[],
        indicator_keywords: &["ServiceUnit", "ServiceStatus", "RestartPolicy",
                               "systemd", "launchd", "TimerSpec"],
    },
    TraitPattern {
        kind: TraitKind::MemoryBackend,
        required_methods: &["store", "query", "compact", "mirror_to_remote",
                             "backend_name"],
        forbidden_deps: &[],
        indicator_keywords: &["MemoryItem", "MemoryQuery", "sqlite", "redis",
                               "postgres", "sled", "memory", "rusqlite"],
    },
    TraitPattern {
        kind: TraitKind::Scheduler,
        required_methods: &["register", "cancel", "list", "next_fire",
                             "scheduler_name"],
        forbidden_deps: &[],
        indicator_keywords: &["ScheduledTask", "ScheduleKind", "cron",
                               "schedule", "jitter", "on_calendar"],
    },
    TraitPattern {
        kind: TraitKind::NetworkMode,
        required_methods: &["configure", "teardown", "peers", "is_public",
                             "mode_name"],
        forbidden_deps: &[],
        indicator_keywords: &["NetworkConfig", "PeerStatus", "wireguard",
                               "tailscale", "vpn", "ipsec", "openvpn"],
    },
    TraitPattern {
        kind: TraitKind::Backup,
        required_methods: &["push", "list", "restore", "prune_older_than",
                             "destination_name"],
        forbidden_deps: &[],
        indicator_keywords: &["Snapshot", "backup", "snapshot", "prune",
                               "s3", "b2", "rclone"],
    },
    TraitPattern {
        kind: TraitKind::CostGuard,
        required_methods: &["record_spend", "current", "reset", "install",
                             "guard_name"],
        forbidden_deps: &[],
        indicator_keywords: &["CostBudget", "CostVerdict", "CostScope",
                               "microcents", "hard_kill", "soft_alert"],
    },
    TraitPattern {
        kind: TraitKind::Observability,
        required_methods: &["log", "metric", "flush", "sink_name",
                             "supports_structured"],
        forbidden_deps: &[],
        indicator_keywords: &["LogEvent", "Metric", "LogLevel", "observability",
                               "otlp", "prometheus", "grafana", "tracing"],
    },
];

// ─────────────────────────── tests ──────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn pattern_count_matches_trait_count() {
        assert_eq!(all_patterns().len(), 12);
    }

    #[test]
    fn all_patterns_have_nonempty_required_methods() {
        for p in all_patterns() {
            assert!(!p.required_methods.is_empty(),
                "TraitKind::{:?} has no required_methods", p.kind);
        }
    }

    #[test]
    fn all_patterns_have_nonempty_keywords() {
        for p in all_patterns() {
            assert!(!p.indicator_keywords.is_empty(),
                "TraitKind::{:?} has no indicator_keywords", p.kind);
        }
    }

    #[test]
    fn trait_kinds_are_unique() {
        let mut seen = HashSet::new();
        for p in all_patterns() {
            let key = format!("{:?}", p.kind);
            assert!(seen.insert(key.clone()), "duplicate TraitKind: {key}");
        }
    }
}
