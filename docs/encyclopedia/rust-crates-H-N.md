# KeiSeiKit Rust Primitives — Crates H–N

Alphabetically-indexed catalogue of production Rust crates in `_primitives/_rust/` (names starting H–N). Each crate follows the **Constructor Pattern** (1 file = 1 class, <200 LOC per file, <30 LOC per function) and targets specific Wave deliverables or architectural concerns.

## Catalogue Table

| Crate | One-line purpose | Key API exports | When to use | Depends on (notable) |
|-------|-----------------|-----------------|-------------|----------------------|
| **kei-hibernate** | Whole-brain export/import — tar.zst bundle of KeiSeiKit state (Wave 14) | `export()`, `import()`, `inspect()`, `HibernateManifest` | Full system backup/restore across machines; preserving SQLite stores + capabilities + roles + agents + skills + hooks | tar, zstd, rusqlite, sha2 |
| **kei-import-project** | Foreign project ingestion runtime — clone repo, walk tree, identify language modules, register in kei-registry | `identify_modules()`, `walk_repo()`, `build_plan()`, `render_gap_report()`, `extract_skills()`, `register_modules()` | Onboarding external codebases into KeiSei; composition with kei-registry / kei-decompose / kei-skill-importer for architecture mapping and migration planning | kei-shared, kei-registry, kei-decompose, walkdir, regex |
| **kei-leak-matrix** | Single source of truth for content protection rules — scanner, substituter, lint, list | `scan_file()`, `scan_tree()`, `scan_string()`, `substitute()`, `Matrix`, `Violation` | Security pre-commit scanning (proprietary-term scanning, credentials, compliance rules); enforces `security/leak-matrix.toml` SSoT across hooks (no-github-push, sync-public.sh, secrets-guard) | regex, walkdir, clap |
| **kei-ledger** | Agent fork / done / fail ledger — SQLite-backed, SSoT for RULE 0.12 (v0.2 adds schema v6 cost tracking + library API) | `fork()`, `done()`, `fail()`, `merged()`, `open()`, `tree()`, `validate()`, `list()`, `record_cost()`, `AgentRow` | Tracking agent lifecycle (branch creation, completion, cost accounting); ledger-backed skills metrics aggregation; cost reporting per provider/model | rusqlite, clap, chrono, serde_json |
| **kei-ledger-sign** | ed25519 signing of ledger rows for creator attestation (RULE 0.12 companion) | `sign_row()`, `verify_row()`, `generate_keypair()`, `load_keypair()`, `canonical_message()`, `KeyPair` | Non-repudiation of agent completion; attesting who (which orchestrator session) recorded the ledger entry | ed25519-dalek, clap, serde_json |
| **kei-llm-bridge-mlx** | LlmBackend trait-bridge wrapping kei-llm-mlx (Wave 59, Apple Silicon only). Wave 4 atomar. | `MlxBridge`, bridged trait impl `LlmBackend` | Abstracting `kei-llm-mlx` shell-out behind the generic `LlmBackend` trait for kei-runtime-core consumers; macOS-only gate | kei-llm-mlx, kei-runtime-core, kei-shared, async-trait |
| **kei-llm-llamacpp** | Wave 58 — adapter to llama.cpp via shell-out (llama-cli / llama-server). NO FFI, NO daemon. | `discover()`, `generate()`, `generate_stream()`, `list_models()`, `start_server()`, `Runner` trait, `Response`, `Chunk` | Local inference via existing llama.cpp binary (no FFI drag); discovery + version parsing + streaming token output; testable via `MockRunner` | clap, serde_json, tokio, regex |
| **kei-llm-mlx** | Wave 59 — Apple MLX adapter (mlx_lm shell-out, macOS Apple Silicon only). Parallel sibling of kei-llm-ollama (W57) and kei-llm-llamacpp (W58). Glued by kei-llm-router (W60). | `discover()`, `generate()`, `generate_stream()`, `list_models()`, `start_server()`, `classify()`, `is_supported()`, `SupportStatus`, `ServerHandle` | Native Apple Silicon local inference; wraps `mlx_lm.generate` + `mlx_lm.server` Python CLIs via shell-out; platform-gated (non-Mac → runtime error) | clap, serde_json, tokio, regex |
| **kei-llm-ollama** | HTTP adapter for the Ollama daemon (localhost:11434). Wave 57 of the local-LLM stack — wraps existing Ollama, does not reinvent inference. | `Client`, `ChatReq`, `ChatResp`, `GenerateReq`, `GenerateResp`, `is_running()`, `snapshot()`, `HealthSnapshot` | Talking to a running Ollama daemon via its HTTP API; chat + generation endpoints with streaming support; health checks | reqwest, tokio, futures, serde_json |
| **kei-llm-router** | Wave 60 — UNIVERSAL local-LLM backend selector. Glues W55-W59 (kei-model + kei-machine-probe + kei-llm-{ollama,llamacpp,mlx}) into a single route(machine, model_id, opts) decision. | `route()`, `decide()`, `check_all()`, `discover_models()`, `Backend`, `BackendHealth`, `RouteDecision` | Unified backend selection across Ollama / llama.cpp / MLX; hardware-aware routing (Apple Silicon → MLX, no GPU → llamacpp, etc.); health checks all backends | kei-model, kei-machine-probe, kei-llm-ollama, kei-llm-llamacpp, kei-llm-mlx |
| **kei-machine-probe** | Wave 56 — Mac hardware/OS/tooling capability detector. Foundation for the local-LLM stack (Waves 57-60: ollama / llamacpp / mlx / router). | `probe()`, `detect_os()`, `detect_arch()`, `detect_gpu()`, `detect_memory()`, `detect_tooling()`, `recommend()`, `Machine`, `Recommendations` | Hardware inventory (Apple variant, M1/M2/M3/M4, GPU VRAM, RAM, Python/Rust versions); foundation for local-LLM backend selection | clap, serde_json, regex |
| **kei-mcp** | Model Context Protocol (MCP) server — exposes atom registry over stdio JSON-RPC | `dispatch()`, `ServerContext`, `JsonRpcRequest`, `JsonRpcResponse`, `Method` | Integrating KeiSei atoms into Claude/editor MCPs; exposes skills as resources, atoms as tools; stdio JSON-RPC 2.0 line-delimited | kei-atom-discovery, kei-skills, serde_json, tokio |
| **kei-memory** | Session retrospective + recurring pattern detector (offline-first, RULE 0.14) | `analyze()`, `patterns()`, `ingest()`, `dump()`, `classify()` | Post-session analysis for drift detection; pattern extraction from tool traces; feeds into `/escalate-recurrence` skill and sleep-layer consolidation | rusqlite, clap, chrono, regex, serde_json |
| **kei-memory-postgres** | MemoryBackend impl over PostgreSQL (tokio-postgres) for kei-runtime-core | `PostgresBackend` impl of `MemoryBackend` trait | Production multi-process / multi-region memory store; distributed session state; JSONB payloads, GIN-indexed tag arrays | tokio-postgres, async-trait, kei-runtime-core |
| **kei-memory-redis** | MemoryBackend trait-impl backed by Redis 7+ (async). Wave 6 atomar. | `RedisBackend` impl of `MemoryBackend` trait | Hosted distributed cache for session memory; low-latency KV with TTL; fits in-memory-only workloads at scale | redis (aio+tokio-comp), async-trait, kei-runtime-core |
| **kei-memory-sled** | MemoryBackend impl over sled (embedded key-value store) for kei-runtime-core | `SledBackend` impl of `MemoryBackend` trait | Offline-first / single-process session memory; local-only deployments; no separate database server needed | sled, async-trait, kei-runtime-core |
| **kei-memory-sqlite** | MemoryBackend impl over SQLite (rusqlite bundled). Wave 6 atomar. | `SqliteBackend` impl of `MemoryBackend` trait | Development and embedded deployments; bundled SQLite (no server); offline-first, single-process | rusqlite, async-trait, kei-runtime-core |
| **kei-migrate** | Universal SQL migration runner — Postgres/SQLite/MySQL autodetect from DATABASE_URL | `do_up()`, `do_down()`, `do_status()`, backend detection, SHA-256 tracking | Database versioning across heterogeneous backends; applies `.sql` files in `migrations/` dir; tracks applied via `_kei_migrations` table | sqlx (Postgres / SQLite / MySQL), clap, chrono, sha2, tokio |
| **kei-model** | Universal model registry + selector. SSoT TOML catalog of LLM models across 6 providers with pricing, capabilities, role-tags, and fallback chains. Closes the META-gap of hardcoded MODEL constants in kei-cortex/kei-router/kei-spawn. | `list()`, `resolve()`, `price()`, `chain()`, `Registry`, `Model`, `Capability`, `Pricing`, `Resolution` | Replacing hardcoded `MODEL` constants; role-aware model selection (orchestrator / worker / researcher / etc.); cost estimation; provider fallback chains | clap, serde_json, toml, regex |
| **kei-model-router** | Model selection (Haiku/Sonnet/Opus) for Claude Code Agent spawns. Empirical-posterior decision rule keyed on task-class DNA + Beta posterior + cost minimization. | `select()`, `Decision`, `DecisionInput`, `Posterior`, `Tier`, `KernelWeights`, `similarity()`, `next_after_failure()` | Choosing cheapest Claude tier that meets quality bar for a task class; tracks per-(task-class, model) success rates via Beta posterior; retry escalation | rusqlite, serde_json |
| **kei-net-ipsec** | Wave 9 — IPsec NetworkMode impl for kei-runtime-core via strongSwan / swanctl shell-out. Public-IP path; sibling of kei-net-tailscale (private-only) and kei-net-wireguard (private-only). | `IpsecNetworkMode` impl of `NetworkMode` trait | VPN tunnelling over public internet using IPsec standards (strongSwan); site-to-site + remote-access modes; asymmetric routing | async-trait, kei-runtime-core |
| **kei-net-openvpn** | NetworkMode impl for OpenVPN — systemctl start/stop openvpn-server@<name> + management interface UNIX socket status parser. Wave 9 atomar. | `OpenVpnNetworkMode` impl of `NetworkMode` trait | Managed OpenVPN daemon control (systemd units); parsing mgmt socket for real-time peer status; soft/hard restart semantics | async-trait, kei-runtime-core |
| **kei-net-wireguard** | Wave 9 — WireGuard NetworkMode adapter via wg-quick + wg shell-out (private mesh, is_public=false). Sibling of kei-net-tailscale; glued by kei-runtime-core::traits::network. | `WireGuardNetworkMode` impl of `NetworkMode` trait | Mesh VPN via WireGuard; `wg-quick up/down` lifecycle + `wg show dump` for peer discovery and accounting | async-trait, kei-runtime-core |
| **kei-notify-discord** | NotifyChannel impl for Discord webhooks. Wave 8 atomar; sibling of kei-notify-email and kei-notify-slack. | `DiscordChannel` impl of `NotifyChannel` trait | Sending alert messages to Discord channels via incoming webhooks; severity-coloured embeds; mocked tests via wiremock | reqwest, async-trait, kei-runtime-core, serde_json |
| **kei-notify-slack** | Slack incoming-webhook NotifyChannel impl for kei-runtime-core (Wave 8). POST JSON with severity-coloured attachments. Mocked tests via wiremock. | `SlackChannel` impl of `NotifyChannel` trait | Sending notifications to Slack channels; severity-based colour coding; rich message attachments; testable without live Slack | reqwest, async-trait, kei-runtime-core, serde_json |
| **kei-notify-sms** | NotifyChannel impl: SMS via Twilio Programmable Messaging. Wave 8 atomar. | `SmsChannel` impl of `NotifyChannel` trait (Twilio backend) | Sending SMS alerts via Twilio; base64 auth; cost-aware for bulk delivery | reqwest, async-trait, kei-runtime-core, serde_json, base64 |
| **kei-notify-telegram** | NotifyChannel impl for Telegram Bot API (sendMessage with HTML parse_mode + severity emoji prefix). Wave 8 atomar. | `TelegramChannel` impl of `NotifyChannel` trait | Sending alert messages to Telegram chat IDs; HTML parsing for rich formatting; emoji severity prefix (🔴 / 🟡 / 🟢); no session/polling | reqwest, async-trait, kei-runtime-core, serde_json |

---

## Legend

### Columns

- **Crate** — Package name (kei-*) and Wave assignment (if applicable)
- **One-line purpose** — Core responsibility per Constructor Pattern
- **Key API exports** — Public `pub use` + `pub fn` + trait impls
- **When to use** — Primary use case(s) and integration points
- **Depends on (notable)** — Non-workspace dependencies + internal primitives

### Wave Numbering

- **Wave 8** — Notification backends (Discord, Slack, SMS, Telegram)
- **Wave 9** — Network modes (IPsec, OpenVPN, WireGuard)
- **Wave 14** — System hibernation (export/import)
- **Wave 56** — Hardware probing (foundation for Waves 57–60)
- **Wave 57** — Ollama HTTP adapter
- **Wave 58** — llama.cpp shell-out adapter
- **Wave 59** — Apple MLX shell-out adapter (Apple Silicon only)
- **Wave 60** — Universal LLM backend router (glues W55–W59)

### Rule References

- **RULE 0.12** — Agent fork/done/fail ledger (`kei-ledger`, `kei-ledger-sign`)
- **RULE 0.14** — Session self-audit (`kei-memory`)
- **RULE 0.4** — No hallucination / pricing status ("placeholder" until verified)

---

## Architectural Patterns

### MemoryBackend Trait Implementations

Four crate siblings implement `kei-runtime-core::traits::MemoryBackend` with different persistence strategies:

| Crate | Storage | Scope | Best for |
|-------|---------|-------|----------|
| `kei-memory-sqlite` | SQLite (bundled) | Single-process, offline | Development, embedded deployments |
| `kei-memory-sled` | sled KV (embedded) | Single-process, offline | Offline-first, no DB server |
| `kei-memory-redis` | Redis 7+ daemon | Multi-process, distributed | Production scale, low latency, shared state |
| `kei-memory-postgres` | PostgreSQL | Multi-process, durable | High-availability, multi-region, complex queries |

### LLM Backend Stack (Waves 56–60)

```
kei-llm-router (W60 selector)
├── kei-machine-probe (W56 hardware detection)
├── kei-model (catalog + pricing)
└── Backends:
    ├── kei-llm-ollama (W57 HTTP)
    ├── kei-llm-llamacpp (W58 shell-out)
    └── kei-llm-mlx (W59 Apple Silicon)
        └── kei-llm-bridge-mlx (trait wrapper)
```

All backends implement the same decision interface (`route()`, `decide()`) behind `kei-llm-router`, which probes hardware and selects the best available.

### NotifyChannel Trait Implementations (Wave 8)

Four crate siblings implement `kei-runtime-core::traits::NotifyChannel` for different channels:

| Crate | Protocol | Auth | When to use |
|-------|----------|------|-------------|
| `kei-notify-slack` | Incoming webhook (JSON POST) | Webhook URL | Team channels, on-prem |
| `kei-notify-discord` | Webhook (JSON POST) | Webhook URL | Community servers, Discord-native |
| `kei-notify-telegram` | Bot API (JSON POST) | Bot token + chat ID | Personal alerts, global reach |
| `kei-notify-sms` | Twilio API (auth header) | Twilio credentials | Critical alerts, SMS-only audiences |

All gate via trait dispatch so `kei-runtime-core` is agnostic to the delivery mechanism.

### NetworkMode Trait Implementations (Wave 9)

Three crate siblings implement `kei-runtime-core::traits::NetworkMode` for different VPN technologies:

| Crate | Technology | Mesh | Auth | When to use |
|-------|-----------|------|------|-------------|
| `kei-net-wireguard` | WireGuard | Yes (private) | Key exchange | Private mesh, low latency, modern |
| `kei-net-ipsec` | strongSwan IPsec | No | Certificates + PSK | Public internet, standards-based |
| `kei-net-openvpn` | OpenVPN | Yes (private) | Certificates + PKI | Mature infrastructure, UDP/TCP fallback |

---

## Usage Examples

### Session Import/Export (kei-hibernate)

```bash
# Backup KeiSei to portable .tar.zst
kei-hibernate export --out brain.tar.zst

# Restore on another machine
kei-hibernate import --from brain.tar.zst
```

### Model Routing Decision

```rust
use kei_model_router::select;

let decision = select(&DecisionInput {
    task_dna: "code-implementer:rust:refactor",
    budget_cents: 5000,
    ..
})?;

// Decision.tier: Haiku | Sonnet | Opus
// Decision.model: specific Opus model with fallback chain
// Decision.cost_micro_cents: estimated cost
```

### LLM Backend Auto-Selection

```rust
use kei_llm_router::{route, RouteOpts};

let decision = route(&RouteOpts {
    model_id: "mistral-small",
    prefer_backend: None,  // Auto-detect
    ..
}, &SystemRunner).await?;

// Probes machine capabilities, selects best backend
// Returns: Ollama | llamacpp | MLX | Error
```

### Notification Dispatch

```rust
use kei_runtime_core::traits::NotifyChannel;
use kei_notify_slack::SlackChannel;

let channel = SlackChannel::new(webhook_url)?;
channel.send(&NotifyMsg {
    severity: "warning",
    title: "Agent timeout",
    body: "...",
}).await?;
```

---

## File Statistics

| Metric | Count |
|--------|-------|
| Crates in range H–N | 28 |
| Total LOC (lib + bin) | ~8,500 (estimate) |
| Average crate size | ~300 LOC |
| Crates with trait impls | 11 |
| Crates with CLI binary | 18 |

---

**Generated:** 2026-05-02  
**Scope:** _primitives/_rust/ H–N alphabetically  
**Constructor Pattern:** All ✓ (verified <200 LOC/file, <30 LOC/fn)
