//! skeleton_table — static method-signature table for skeleton generation.
//!
//! One `TraitMeta` entry per kei-runtime-core trait, with verbatim fn
//! signatures copied from the actual trait files.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use crate::trait_patterns::TraitKind;

/// One method entry: name, full fn signature opening brace, and a
/// one-line semantic hint for the TODO comment.
pub struct MethodEntry {
    pub name: &'static str,
    pub sig: &'static str,
    pub todo_hint: &'static str,
}

/// Metadata for one runtime trait.
pub struct TraitMeta {
    pub trait_name: &'static str,
    pub use_imports: &'static str,
    pub methods: &'static [MethodEntry],
    pub kind: TraitKind,
}

/// Return the static trait metadata for `kind`.
pub fn trait_meta(kind: TraitKind) -> &'static TraitMeta {
    TRAIT_TABLE.iter().find(|m| m.kind == kind)
        .expect("every TraitKind has a static entry in skeleton_table")
}

static TRAIT_TABLE: &[TraitMeta] = &[
    TraitMeta { kind: TraitKind::ComputeProvider,
        trait_name: "ComputeProvider",
        use_imports: "use kei_runtime_core::traits::compute::{VmSpec, VmHandle, VmStatus};\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "provider_name", sig: "fn provider_name(&self) -> &'static str {", todo_hint: "return a unique string identifying this provider" },
            MethodEntry { name: "create", sig: "async fn create(&self, spec: &VmSpec) -> Result<VmHandle> {", todo_hint: "provision a new VM from spec; return populated VmHandle" },
            MethodEntry { name: "destroy", sig: "async fn destroy(&self, h: &VmHandle) -> Result<()> {", todo_hint: "terminate VM identified by h.external_id" },
            MethodEntry { name: "resize", sig: "async fn resize(&self, h: &VmHandle, new_tier: &str) -> Result<VmHandle> {", todo_hint: "resize VM to new_tier, return updated handle" },
            MethodEntry { name: "status", sig: "async fn status(&self, h: &VmHandle) -> Result<VmStatus> {", todo_hint: "poll provider API, map to VmStatus enum" },
            MethodEntry { name: "stop", sig: "async fn stop(&self, h: &VmHandle) -> Result<()> {", todo_hint: "stop (but do not destroy) the VM" },
            MethodEntry { name: "start", sig: "async fn start(&self, h: &VmHandle) -> Result<()> {", todo_hint: "start a stopped VM" },
            MethodEntry { name: "cost_per_hour_microcents", sig: "fn cost_per_hour_microcents(&self, tier: &str) -> u64 {", todo_hint: "return USD micro-cents per hour for this tier" },
        ],
    },
    TraitMeta { kind: TraitKind::AuthProvider,
        trait_name: "AuthProvider",
        use_imports: "use kei_runtime_core::traits::auth::{AuthChallenge, AuthSession};\nuse kei_runtime_core::dna::Dna;\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "provider_name", sig: "fn provider_name(&self) -> &'static str {", todo_hint: "unique identifier for this auth provider" },
            MethodEntry { name: "issue_challenge", sig: "async fn issue_challenge(&self, c: &AuthChallenge) -> Result<()> {", todo_hint: "send magic link, generate OAuth redirect, etc." },
            MethodEntry { name: "verify", sig: "async fn verify(&self, c: &AuthChallenge) -> Result<AuthSession> {", todo_hint: "verify challenge response, return session on success" },
            MethodEntry { name: "revoke", sig: "async fn revoke(&self, session: &Dna) -> Result<()> {", todo_hint: "invalidate the given session DNA" },
            MethodEntry { name: "is_passwordless", sig: "fn is_passwordless(&self) -> bool {", todo_hint: "return true if this provider never uses passwords" },
        ],
    },
    TraitMeta { kind: TraitKind::NotifyChannel,
        trait_name: "NotifyChannel",
        use_imports: "use kei_runtime_core::traits::notify::Notification;\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "channel_name", sig: "fn channel_name(&self) -> &'static str {", todo_hint: "unique identifier for this notification channel" },
            MethodEntry { name: "send", sig: "async fn send(&self, n: &Notification) -> Result<()> {", todo_hint: "deliver notification; respect n.severity and body_html" },
            MethodEntry { name: "supports_batching", sig: "fn supports_batching(&self) -> bool {", todo_hint: "return true if channel supports digest / batch sends" },
        ],
    },
    TraitMeta { kind: TraitKind::GitBackend,
        trait_name: "GitBackend",
        use_imports: "use kei_runtime_core::traits::git::{GitRemote, CommitMeta};\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "provider_name", sig: "fn provider_name(&self) -> &'static str {", todo_hint: "identifier for this git host provider" },
            MethodEntry { name: "ensure_repo", sig: "async fn ensure_repo(&self, remote: &GitRemote) -> Result<()> {", todo_hint: "create repo via API if absent; no-op if present" },
            MethodEntry { name: "clone", sig: "async fn clone(&self, remote: &GitRemote, dest: &std::path::Path) -> Result<()> {", todo_hint: "clone remote to dest path on disk" },
            MethodEntry { name: "push", sig: "async fn push(&self, dir: &std::path::Path, remote: &GitRemote) -> Result<CommitMeta> {", todo_hint: "push local dir to remote, return HEAD commit metadata" },
            MethodEntry { name: "mirror", sig: "async fn mirror(&self, src: &GitRemote, dst: &GitRemote) -> Result<()> {", todo_hint: "mirror all branches from src to dst" },
            MethodEntry { name: "supports_auto_create", sig: "fn supports_auto_create(&self) -> bool {", todo_hint: "true if this backend can create repos via API" },
        ],
    },
    TraitMeta { kind: TraitKind::LlmBackend,
        trait_name: "LlmBackend",
        use_imports: "use kei_runtime_core::traits::llm::{Message, CompletionOpts, CompletionResponse};\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "backend_name", sig: "fn backend_name(&self) -> &'static str {", todo_hint: "unique name for this LLM backend" },
            MethodEntry { name: "model_name", sig: "fn model_name(&self) -> &str {", todo_hint: "model identifier string (e.g. 'llama3:8b')" },
            MethodEntry { name: "complete", sig: "async fn complete(&self, messages: &[Message], opts: &CompletionOpts) -> Result<CompletionResponse> {", todo_hint: "call provider API with messages, return completion" },
            MethodEntry { name: "pricing_per_mtok", sig: "fn pricing_per_mtok(&self) -> (f64, f64) {", todo_hint: "return (input_usd_per_mtok, output_usd_per_mtok)" },
            MethodEntry { name: "supports_caching", sig: "fn supports_caching(&self) -> bool {", todo_hint: "true if provider caches prompt prefixes" },
            MethodEntry { name: "supports_batch", sig: "fn supports_batch(&self) -> bool {", todo_hint: "true if provider accepts batch inference" },
            MethodEntry { name: "context_window", sig: "fn context_window(&self) -> u32 {", todo_hint: "maximum context tokens for the configured model" },
        ],
    },
    TraitMeta { kind: TraitKind::ServiceManager,
        trait_name: "ServiceManager",
        use_imports: "use kei_runtime_core::traits::service::{ServiceUnit, ServiceStatus};\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "manager_name", sig: "fn manager_name(&self) -> &'static str {", todo_hint: "e.g. 'systemd', 'launchd'" },
            MethodEntry { name: "install", sig: "async fn install(&self, unit: &ServiceUnit) -> Result<()> {", todo_hint: "write unit file and register with the init system" },
            MethodEntry { name: "uninstall", sig: "async fn uninstall(&self, name: &str) -> Result<()> {", todo_hint: "stop, disable, and remove the named service unit" },
            MethodEntry { name: "start", sig: "async fn start(&self, name: &str) -> Result<()> {", todo_hint: "start the named service" },
            MethodEntry { name: "stop", sig: "async fn stop(&self, name: &str) -> Result<()> {", todo_hint: "stop the named service" },
            MethodEntry { name: "status", sig: "async fn status(&self, name: &str) -> Result<ServiceStatus> {", todo_hint: "query init system, map to ServiceStatus enum" },
            MethodEntry { name: "enable_at_boot", sig: "async fn enable_at_boot(&self, name: &str) -> Result<()> {", todo_hint: "configure service to start automatically on boot" },
        ],
    },
    TraitMeta { kind: TraitKind::MemoryBackend,
        trait_name: "MemoryBackend",
        use_imports: "use kei_runtime_core::traits::memory::{MemoryItem, MemoryQuery};\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "backend_name", sig: "fn backend_name(&self) -> &'static str {", todo_hint: "unique name for this memory backend" },
            MethodEntry { name: "store", sig: "async fn store(&self, item: &MemoryItem) -> Result<()> {", todo_hint: "persist item; upsert on item.dna collision" },
            MethodEntry { name: "query", sig: "async fn query(&self, q: &MemoryQuery) -> Result<Vec<MemoryItem>> {", todo_hint: "filter items by kind/key_prefix/tags/since_ms" },
            MethodEntry { name: "compact", sig: "async fn compact(&self, since_ms: i64) -> Result<usize> {", todo_hint: "Phase B REM: consolidate items since since_ms" },
            MethodEntry { name: "mirror_to_remote", sig: "async fn mirror_to_remote(&self, dest_url: &str) -> Result<()> {", todo_hint: "push memory diffs to remote git host (sleep-sync)" },
        ],
    },
    TraitMeta { kind: TraitKind::Scheduler,
        trait_name: "Scheduler",
        use_imports: "use kei_runtime_core::traits::scheduler::ScheduledTask;\nuse kei_runtime_core::dna::Dna;\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "scheduler_name", sig: "fn scheduler_name(&self) -> &'static str {", todo_hint: "unique name for this scheduler backend" },
            MethodEntry { name: "register", sig: "async fn register(&self, task: &ScheduledTask) -> Result<()> {", todo_hint: "persist task and arm its trigger (cron/at/webhook/etc)" },
            MethodEntry { name: "cancel", sig: "async fn cancel(&self, dna: &Dna) -> Result<()> {", todo_hint: "disarm and delete the task with this DNA" },
            MethodEntry { name: "list", sig: "async fn list(&self) -> Result<Vec<ScheduledTask>> {", todo_hint: "return all registered tasks" },
            MethodEntry { name: "next_fire", sig: "async fn next_fire(&self) -> Result<ScheduledTask> {", todo_hint: "block until next task fires; return the task" },
        ],
    },
    TraitMeta { kind: TraitKind::NetworkMode,
        trait_name: "NetworkMode",
        use_imports: "use kei_runtime_core::traits::network::{NetworkConfig, PeerStatus};\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "mode_name", sig: "fn mode_name(&self) -> &'static str {", todo_hint: "e.g. 'wireguard', 'tailscale', 'ipsec'" },
            MethodEntry { name: "configure", sig: "async fn configure(&self, cfg: &NetworkConfig) -> Result<()> {", todo_hint: "apply network config (keys, peers, firewall rules)" },
            MethodEntry { name: "teardown", sig: "async fn teardown(&self) -> Result<()> {", todo_hint: "bring down all tunnels/interfaces managed by this mode" },
            MethodEntry { name: "peers", sig: "async fn peers(&self) -> Result<Vec<PeerStatus>> {", todo_hint: "return current peer status list from kernel/daemon" },
            MethodEntry { name: "is_public", sig: "fn is_public(&self) -> bool {", todo_hint: "true if this mode exposes a public IP" },
        ],
    },
    TraitMeta { kind: TraitKind::Backup,
        trait_name: "Backup",
        use_imports: "use kei_runtime_core::traits::backup::Snapshot;\nuse kei_runtime_core::dna::Dna;\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "destination_name", sig: "fn destination_name(&self) -> &'static str {", todo_hint: "e.g. 'b2', 's3', 'rclone-gdrive'" },
            MethodEntry { name: "push", sig: "async fn push(&self, local_path: &std::path::Path, parent_dna: &Dna) -> Result<Snapshot> {", todo_hint: "upload local_path to remote; return snapshot metadata" },
            MethodEntry { name: "list", sig: "async fn list(&self, prefix: &str) -> Result<Vec<Snapshot>> {", todo_hint: "list all snapshots matching key prefix" },
            MethodEntry { name: "restore", sig: "async fn restore(&self, snap: &Snapshot, dest: &std::path::Path) -> Result<()> {", todo_hint: "download snap to dest path on disk" },
            MethodEntry { name: "prune_older_than", sig: "async fn prune_older_than(&self, ms: i64) -> Result<usize> {", todo_hint: "delete snapshots older than ms epoch; return count" },
        ],
    },
    TraitMeta { kind: TraitKind::CostGuard,
        trait_name: "CostGuard",
        use_imports: "use kei_runtime_core::traits::cost::{CostBudget, CostVerdict};\nuse kei_runtime_core::dna::Dna;\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "guard_name", sig: "fn guard_name(&self) -> &'static str {", todo_hint: "unique name for this cost guard implementation" },
            MethodEntry { name: "record_spend", sig: "async fn record_spend(&self, budget: &Dna, microcents: u64) -> Result<CostVerdict> {", todo_hint: "atomically add microcents; return Ok/SoftAlert/HardKill" },
            MethodEntry { name: "current", sig: "async fn current(&self, budget: &Dna) -> Result<CostBudget> {", todo_hint: "fetch current budget state from store" },
            MethodEntry { name: "reset", sig: "async fn reset(&self, budget: &Dna) -> Result<()> {", todo_hint: "zero out current_microcents on the budget" },
            MethodEntry { name: "install", sig: "async fn install(&self, b: &CostBudget) -> Result<Dna> {", todo_hint: "persist new budget configuration; return its DNA" },
        ],
    },
    TraitMeta { kind: TraitKind::Observability,
        trait_name: "Observability",
        use_imports: "use kei_runtime_core::traits::observability::{LogEvent, Metric};\nuse kei_runtime_core::error::Result;\nuse async_trait::async_trait;\n",
        methods: &[
            MethodEntry { name: "sink_name", sig: "fn sink_name(&self) -> &'static str {", todo_hint: "e.g. 'stdout', 'otlp', 'prometheus'" },
            MethodEntry { name: "log", sig: "async fn log(&self, event: &LogEvent) -> Result<()> {", todo_hint: "emit structured log event to this sink" },
            MethodEntry { name: "metric", sig: "async fn metric(&self, m: &Metric) -> Result<()> {", todo_hint: "record numeric metric with tags and timestamp" },
            MethodEntry { name: "flush", sig: "async fn flush(&self) -> Result<()> {", todo_hint: "flush any buffered events/metrics to the backend" },
            MethodEntry { name: "supports_structured", sig: "fn supports_structured(&self) -> bool {", todo_hint: "true if sink supports structured fields (JSON/OTLP)" },
        ],
    },
];
