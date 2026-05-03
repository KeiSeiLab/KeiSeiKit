# KeiSeiKit Rust Crates: A–G Catalogue

> Generated 2026-05-02. Non-kei-prefixed + kei-[a-g]* crates from `_primitives/_rust/`.
> Constructor Pattern: each crate = one responsibility; largest function <30 LOC, file <200 LOC.

## Non-kei-prefixed Crates

| Crate | One-line Purpose | Key API Exports | When to Use | Depends On |
|-------|-----------------|-----------------|-------------|-----------|
| **firewall-diff** | Compare intended ufw rules (YAML) against live firewall state (from `ufw status numbered` output) | `diff::DiffResult`, `intent::Rule`, `ufw::Parser` | Defensive security audit before deploying firewall rules; CI gate for ufw config drift detection | clap, serde_yaml, serde_json, tempfile |
| **frustration-matrix** | Regex-based chatlog scanner to compute per-user frustration score (longitudinal matrix, no ML) | `firmware::Firmware`, `categories::Category`, `jsonl::Parser`, `firmware_corpus::Corpus` | User experience monitoring; feedback loop training for `kei-frustration-loop`; JSONL chatlog ingestion | regex, walkdir, serde, flate2, clap, anyhow |
| **mock-render** | WYSIWYD (What You See Is What's Deployed) enforcer for block-builder site: screenshot → hash lock → source-mutation detect | `render::Render`, `hash::ContentHash`, `state::State` | Preventing "design in UI, deploy different code" bugs; screenshot verification before locking a site section | serde_json, sha2, tempfile |
| **ssh-check** | Pre-deploy sshd_config linter — parses `/etc/ssh/sshd_config` + drop-ins, enforces hardened baseline rules | `rules::RuleSet`, `parse::Directive`, `check::Violation` | Hardened SSH baseline validation before server deploy; CI gate for sshd_config compliance; multi-config merge (last-wins) | clap, serde_json, tempfile |
| **tokens-sync** | Single-source-of-truth emitter for Tailwind config + CSS custom properties from JSON design-tokens file | `parse::TokensSchema`, `emit::TailwindConfig`, `emit::CssVariables` | Eliminating color/spacing/font drift between CSS and Tailwind sides; one-way regeneration on token update | serde_json, image (for visual-diff companion) |
| **visual-diff** | Pixel-level PNG comparator for WYSIWYD drift detection — outputs difference % and red-overlay diff image | `diff::PixelDiff`, `diff::ThresholdResult` | Screenshot regression testing in CI; WYSIWYD enforcement alongside mock-render; smoke-test for design system changes | image, tempfile |

---

## kei-agent-runtime

| Field | Value |
|-------|-------|
| **One-line Purpose** | Agent substrate v1 runtime: capability registry, compose/spawn/verify gates + on-return verification layer |
| **Key API Exports** | `Capability`, `CapabilityRegistry`, `AgentInvocation`, `PreToolUseGate`, `PostReturnVerify`, `ComposedPrompt`, `SimulatedMerge` |
| **When to Use** | Orchestrator spawning code/research/critic agents; RULE 0.12 agent-git-model fork/verify ceremony; capability gating (auth, pre-commit, post-deploy checks) |
| **Depends On** | clap, serde, toml, regex, sha2, rand, kei-shared, anyhow, thiserror, once_cell, walkdir, tempfile |

---

## kei-artifact

| Field | Value |
|-------|-------|
| **One-line Purpose** | Typed artifact handoff pipeline — BMAD-style document pass-between agents with JSON Schema validation |
| **Key API Exports** | `Artifact`, `ArtifactStore`, `Validator`, `HandoffLog` |
| **When to Use** | Agent-to-agent workflow continuity; artifact versioning across RULE 0.12 multi-phase projects; validating output before merge |
| **Depends On** | rusqlite, clap, serde, serde_json, sha2, chrono, anyhow, tempfile |

---

## kei-atom-discovery

| Field | Value |
|-------|-------|
| **One-line Purpose** | Shared atom discovery + frontmatter YAML parsing + safe path join utilities |
| **Key API Exports** | `discover_atoms()`, `parse_frontmatter()`, `safe_join()` |
| **When to Use** | Project skeleton scanning; module initialization in `kei-forge` / `kei-cache` / any crate discovering source structure; safe cross-platform path handling |
| **Depends On** | serde, serde_yaml_ng, walkdir, thiserror, tempfile |

---

## kei-auth

| Field | Value |
|-------|-------|
| **One-line Purpose** | Multi-tenant session tokens with scopes + HMAC-signed expiry (SQLite backend) |
| **Key API Exports** | `Token`, `TokenStore`, `Scope`, `ExpiryClaim` |
| **When to Use** | Session management for kei-cortex; token issuance for agent API clients; stateless expiry verification (no DB round-trip on validate) |
| **Depends On** | rusqlite, clap, serde, serde_json, chrono, hmac, sha2, base64, rand, anyhow, tempfile |

---

## kei-auth-apple

| Field | Value |
|-------|-------|
| **One-line Purpose** | Sign in with Apple AuthProvider impl for kei-runtime-core — OAuth code → token endpoint → unverified id_token claim decode |
| **Key API Exports** | `AppleAuthProvider`, `AppleCode`, `AppleClaim` |
| **When to Use** | macOS/iOS user authentication in kei-cortex; SSO against Apple ecosystem (Wave 7 atomar; siblings: kei-auth-google, kei-auth-magiclink, kei-auth-webauthn) |
| **Depends On** | async-trait, serde, serde_json, tokio, reqwest, base64, jsonwebtoken, sha2, subtle, kei-runtime-core, wiremock |

---

## kei-auth-google

| Field | Value |
|-------|-------|
| **One-line Purpose** | AuthProvider impl for Google OAuth 2.0 + OIDC (Wave 7 atomar) |
| **Key API Exports** | `GoogleAuthProvider`, `GoogleCode`, `GoogleClaim` |
| **When to Use** | Google SSO login flows in kei-cortex; OIDC token validation; sibling to kei-auth-{apple,magiclink,webauthn} in multi-provider auth strategy |
| **Depends On** | async-trait, serde, serde_json, tokio, reqwest, sha2, subtle, base64, kei-runtime-core, wiremock |

---

## kei-auth-magiclink

| Field | Value |
|-------|-------|
| **One-line Purpose** | AuthProvider impl for passwordless email magic-link tokens (HMAC-SHA256, stateless) — Wave 7 atomar |
| **Key API Exports** | `MagiclinkAuthProvider`, `MagiclinkToken`, `MagiclinkProof` |
| **When to Use** | Passwordless email-based auth in kei-cortex; no password DB needed; fallback auth when SSO unavailable; sibling to kei-auth-{google,apple,webauthn} |
| **Depends On** | async-trait, serde, serde_json, tokio, sha2, hmac, base64, subtle, kei-runtime-core |

---

## kei-auth-webauthn

| Field | Value |
|-------|-------|
| **One-line Purpose** | WebAuthn passkey AuthProvider impl for kei-runtime-core (Wave 7 atomar) — wraps webauthn-rs 0.5; stateless ceremony APIs |
| **Key API Exports** | `WebAuthnAuthProvider`, `PasskeyChallenge`, `PasskeyProof` |
| **When to Use** | Hardware passkey / biometric login (Touch ID, Windows Hello, YubiKey); passwordless phishing-resistant auth; sibling to kei-auth-{google,apple,magiclink} |
| **Depends On** | async-trait, serde, serde_json, tokio, url, webauthn-rs, uuid, kei-runtime-core |

---

## kei-backend-daytona

| Field | Value |
|-------|-------|
| **One-line Purpose** | Daytona serverless backend with hibernation (HERMES-MIGRATION P1.2) — resume-or-create sandboxes via REST |
| **Key API Exports** | `DaytonaClient`, `Sandbox`, `HibernationState` |
| **When to Use** | Managed sandbox compute for HERMES workflows; cold-start + resume patterns; sibling to kei-compute-{vultr,digitalocean,linode,baremetal} |
| **Depends On** | tokio, reqwest, serde, serde_json, anyhow, thiserror, async-trait, wiremock |

---

## kei-brain-view

| Field | Value |
|-------|-------|
| **One-line Purpose** | Read-only TUI/CLI visualizer of kei-ledger taxonomy graph + agent lineage (Wave 14) |
| **Key API Exports** | `BrainTui`, `LedgerGraph`, `AgentLineage` |
| **When to Use** | Debugging agent DAGs; visualizing taxonomy after deep-sleep conflicts; RULE 0.12 agent-git-model branch lineage inspection |
| **Depends On** | rusqlite, clap, serde, thiserror, kei-dna-index, tempfile |

---

## kei-cache

| Field | Value |
|-------|-------|
| **One-line Purpose** | Atom result cache — deterministic wrapping of pure (query/transform) atom invocations |
| **Key API Exports** | `Cache`, `CacheKey`, `InvalidationTrigger` |
| **When to Use** | Avoiding re-computation of expensive deterministic atoms (e.g. graph transforms); cache invalidation on input change via `kei-diff` structural checks |
| **Depends On** | rusqlite, clap, serde, serde_json, sha2, anyhow, thiserror, kei-atom-discovery, tempfile |

---

## kei-capability

| Field | Value |
|-------|-------|
| **One-line Purpose** | Hook-protocol CLI adapter — routes PreToolUse check + on-return verify to kei-agent-runtime capabilities |
| **Key API Exports** | `CapabilityCheck`, `CapabilityVerify`, `HookContext` |
| **When to Use** | Wiring RULE 0.12 gates into `~/.claude/hooks/` scripts; CI integration for capability verification; Pre/Post-Tool-Use enforcement |
| **Depends On** | kei-agent-runtime, clap, serde, serde_json, anyhow, toml, tempfile |

---

## kei-changelog

| Field | Value |
|-------|-------|
| **One-line Purpose** | Git-cliff-style CHANGELOG.md generator from Conventional Commits |
| **Key API Exports** | `ChangelogBuilder`, `ConventionalEntry`, `ReleaseNote` |
| **When to Use** | Auto-generating CHANGELOG from git log on release; Conventional Commit parsing for structured release notes |
| **Depends On** | clap, anyhow, chrono, git2, regex |

---

## kei-chat-store

| Field | Value |
|-------|-------|
| **One-line Purpose** | Session persistence for Claude conversations — port of LBM internal/chat |
| **Key API Exports** | `ChatStore`, `Message`, `Session`, `ThreadId` |
| **When to Use** | Persisting Claude conversation history for UI replay; session retrieval across kei-cortex restart; chatlog analysis for frustration-matrix |
| **Depends On** | kei-entity-store, rusqlite, clap, serde, serde_json, chrono, uuid, anyhow, tempfile |

---

## kei-compute-baremetal

| Field | Value |
|-------|-------|
| **One-line Purpose** | ComputeProvider impl for user-owned bare-metal boxes — registers SSH connection, runs cloud-init equivalent, status-pings via SSH |
| **Key API Exports** | `BareMetalComputeProvider`, `SshConnection`, `CloudInitPayload` |
| **When to Use** | Registering existing hardware (no cloud API); SSH-based provisioning; sibling to kei-compute-{vultr,digitalocean,linode} for multi-cloud flexibility |
| **Depends On** | async-trait, clap, serde, serde_json, thiserror, tokio, kei-runtime-core, kei-shared |

---

## kei-compute-digitalocean

| Field | Value |
|-------|-------|
| **One-line Purpose** | DigitalOcean ComputeProvider impl for kei-runtime-core (Wave 2 redo) — REST v2 + bearer-token auth |
| **Key API Exports** | `DigitalOceanComputeProvider`, `Droplet`, `SizeSlug` |
| **When to Use** | DigitalOcean droplet lifecycle (create/destroy/halt/start/resize); mocked tests via wiremock; sibling to kei-compute-{vultr,linode,baremetal} |
| **Depends On** | async-trait, serde, serde_json, thiserror, tokio, reqwest, kei-runtime-core, wiremock |

---

## kei-compute-linode

| Field | Value |
|-------|-------|
| **One-line Purpose** | ComputeProvider impl for Linode (Akamai Cloud) v4 API — Wave 2 atomar |
| **Key API Exports** | `LinodeComputeProvider`, `Instance`, `InstanceType` |
| **When to Use** | Linode instance provisioning; tier validation (g6-nanode-1, g6-standard-*, g6-dedicated-*); sibling to kei-compute-{vultr,digitalocean,baremetal} |
| **Depends On** | kei-runtime-core, kei-shared, async-trait, serde, serde_json, thiserror, tokio, reqwest, clap, base64, wiremock, tempfile |

---

## kei-compute-vultr

| Field | Value |
|-------|-------|
| **One-line Purpose** | Vultr Cloud (v2 API) implementation of the kei-runtime-core ComputeProvider trait |
| **Key API Exports** | `VultrComputeProvider`, `Instance`, `InstanceType` |
| **When to Use** | Vultr instance creation/destroy/halt/start/resize; base64 cloud-init dispatch; vc2/vhf tier validation; sibling to kei-compute-{digitalocean,linode,baremetal} |
| **Depends On** | async-trait, clap, reqwest, serde, serde_json, thiserror, tokio, kei-runtime-core, kei-shared, wiremock |

---

## kei-conflict-scan

| Field | Value |
|-------|-------|
| **One-line Purpose** | Deep-sleep conflict scanner — detects rules/hooks/blocks conflicts, orphan files, Constructor Pattern violations (v0.13.0) |
| **Key API Exports** | `ConflictScan`, `Conflict`, `ConflictCategory` |
| **When to Use** | RULE 0.15 Phase C deep-sleep consolidation; automated conflict detection before refactor branches; CI gate for RULE 0.13 orchestrator compliance |
| **Depends On** | clap, serde, serde_json, regex, walkdir, anyhow, tempfile |

---

## kei-content-store

| Field | Value |
|-------|-------|
| **One-line Purpose** | Asset + prompt + campaign registry — port of LBM internal/content |
| **Key API Exports** | `ContentStore`, `Asset`, `Campaign`, `Prompt` |
| **When to Use** | Managing reusable prompts, assets, campaigns across kei-cortex; content versioning; sibling to kei-chat-store, kei-entity-store |
| **Depends On** | kei-entity-store, rusqlite, clap, serde, serde_json, chrono, sha2, anyhow, tempfile |

---

## kei-cortex

| Field | Value |
|-------|-------|
| **One-line Purpose** | Local HTTP daemon exposing cortex state for UI consumption — kei-cortex-default role + token telemetry + per-turn event tracking |
| **Key API Exports** | `CortexServer`, `CortexState`, `MessageHandler`, `TokenTracker`, `ModelRegistry` |
| **When to Use** | Central coordination server for KeiSei agents; UI backend; chat state persistence; model routing; foundational for Wave 55 + Phase 2 telemetry |
| **Depends On** | axum, tokio, tower, tower-http, serde, serde_json, clap, thiserror, rusqlite, reqwest, uuid, async-stream, toml, bytes, dashmap, walkdir, regex, portable-pty, shell-words, url, lru, nix, chrono, kei-pet, kei-router, kei-shared, kei-ledger, kei-model, kei-token-tracker, tempfile |

---

## kei-cron-scheduler

| Field | Value |
|-------|-------|
| **One-line Purpose** | P4.2 — Hermes-equivalent cron/at/interval scheduler with JSON persistence (fcntl-locked read-modify-write SSoT) |
| **Key API Exports** | `Scheduler`, `ScheduledJob`, `CronExpression`, `RunnerHandle` |
| **When to Use** | Background task scheduling without DB; portable flat-file config backup; Hermes-parity; sibling to kei-scheduler (SQL-backed, queryable) |
| **Depends On** | tokio, serde, serde_json, anyhow, thiserror, chrono, cron, fs2, tempfile, pretty_assertions |

---

## kei-crossdomain

| Field | Value |
|-------|-------|
| **One-line Purpose** | Typed-edge cross-domain store — port of LBM internal/crossdomain |
| **Key API Exports** | `CrossdomainStore`, `Domain`, `Edge`, `Relationship` |
| **When to Use** | Managing cross-domain relationships; typed edges between entities; sibling to kei-entity-store, kei-content-store, kei-chat-store |
| **Depends On** | kei-entity-store, rusqlite, clap, serde, serde_json, chrono, anyhow, tempfile |

---

## kei-curator

| Field | Value |
|-------|-------|
| **One-line Purpose** | Edge-decay + orphan-prune graph hygiene — port of LBM internal/curator |
| **Key API Exports** | `Curator`, `DecayPolicy`, `PruneReport` |
| **When to Use** | Removing stale edges in agent lineage graphs; cleaning up orphan tasks; graph compaction after deep-sleep; data maintenance cron job |
| **Depends On** | rusqlite, clap, serde, serde_json, chrono, anyhow, tempfile |

---

## kei-db-contract

| Field | Value |
|-------|-------|
| **One-line Purpose** | Diff SQL migration schemas against TypeScript type declarations to catch frontend ↔ DB drift |
| **Key API Exports** | `SchemaDiff`, `SqlParser`, `TypescriptParser`, `DriftReport` |
| **When to Use** | Pre-deploy type safety check; CI gate for schema-code consistency; catch column-rename bugs before deployment |
| **Depends On** | clap, serde, serde_json, regex, walkdir, anyhow, sqlparser, tempfile |

---

## kei-decision

| Field | Value |
|-------|-------|
| **One-line Purpose** | Linking layer between research output (MASTER-REPORT.md) and decision execution (kei-spawn task.toml + kei-ledger pre-fork) |
| **Key API Exports** | `DecisionParser`, `ActionPlan`, `ExecutionStep` |
| **When to Use** | Parsing /research output into actionable task.toml; automating RULE 0.12 fork ceremony; closing research → action pipeline without manual orchestrator work |
| **Depends On** | clap, serde, serde_json, toml, regex, anyhow, walkdir, tempfile |

---

## kei-decompose

| Field | Value |
|-------|-------|
| **One-line Purpose** | Universal decomposition layer — turns ANY MD output (research/audit/sleep/architecture/new-project) into kei-spawn-compatible task.toml + dispatch |
| **Key API Exports** | `FormatParser`, `Action`, `ActionNormalizer` |
| **When to Use** | Wave 50 META solution: unified MD format handler for 6+ ad-hoc formats; closing analysis → execution; sibling to kei-decision (research-specific) |
| **Depends On** | clap, serde, serde_json, toml, regex, anyhow, walkdir, rusqlite, kei-registry, tempfile |

---

## kei-diff

| Field | Value |
|-------|-------|
| **One-line Purpose** | Structural JSON diff (RFC 6902 subset: add/remove/replace) — pure computation primitive for drift detection + cache invalidation |
| **Key API Exports** | `JsonDiff`, `Patch`, `PatchOp` |
| **When to Use** | Computing cache invalidation in kei-cache; structural change detection in kei-db-contract; WYSIWYD drift in mock-render companion |
| **Depends On** | serde, serde_json |

---

## kei-discover

| Field | Value |
|-------|-------|
| **One-line Purpose** | Wave 14 — federated marketplace discovery stub for KeiSei primitives (metadata-only) |
| **Key API Exports** | `DiscoveryIndex`, `PrimitiveMetadata`, `SearchQuery` |
| **When to Use** | Marketplace primitive cataloguing; future discovery UI; RULE 0.12 agent-git-model taxonomy integration (deferred implementation) |
| **Depends On** | kei-entity-store, rusqlite, clap, serde, serde_json, thiserror, tempfile |

---

## kei-dna-index

| Field | Value |
|-------|-------|
| **One-line Purpose** | Read-only adjacency/cluster/precedent index over kei-ledger DNAs (agent taxonomy graphs) |
| **Key API Exports** | `DnaIndex`, `Adjacency`, `Cluster`, `Precedent` |
| **When to Use** | Navigating agent DAGs in kei-brain-view; deriving agent relationship graphs; RULE 0.12 agent-git-model post-merge analysis |
| **Depends On** | rusqlite, clap, serde, serde_json, thiserror, kei-shared, tempfile |

---

## kei-entity-store

| Field | Value |
|-------|-------|
| **One-line Purpose** | Convergence-Layer-A engine: schema-driven SQLite CRUD + graph verbs shared across kei-*-store crates |
| **Key API Exports** | `EntityStore`, `Entity`, `Schema`, `GraphVerb` |
| **When to Use** | Base class for kei-{chat,content,crossdomain,task}-store; schema validation; sibling APIs; shared migrations |
| **Depends On** | rusqlite, serde, serde_json, chrono, anyhow, thiserror, tempfile |

---

## kei-export-trajectories

| Field | Value |
|-------|-------|
| **One-line Purpose** | Export agent trajectories to ShareGPT JSONL format for downstream model training |
| **Key API Exports** | `TrajectoryExporter`, `ShareGptEntry` |
| **When to Use** | Converting kei-ledger agent chatlogs to ML-friendly format; preparing trajectory datasets; training specialized models on agent behavior |
| **Depends On** | rusqlite, serde, serde_json, anyhow, clap, chrono, tempfile |

---

## kei-fork

| Field | Value |
|-------|-------|
| **One-line Purpose** | Managed git-worktree + ledger lifecycle for agent spawns (Wave 15 foundation) — RULE 0.12 agent-git-model orchestration |
| **Key API Exports** | `ForkManager`, `WorktreeHandle`, `LedgerRow` |
| **When to Use** | Creating isolated agent branches (RULE 0.12); managing fork/merge ceremonies; ledger row creation on spawn |
| **Depends On** | kei-agent-runtime, rusqlite, clap, serde, serde_json, toml, thiserror, chrono, tempfile |

---

## kei-frustration-loop

| Field | Value |
|-------|-------|
| **One-line Purpose** | Per-user frustration learning loop — feedback ingestion + auto-retrain trigger + nightly Phase-0 cron hook |
| **Key API Exports** | `FrustrationLoop`, `FeedbackIngestor`, `RetrainTrigger` |
| **When to Use** | Consuming `frustration-matrix` firmware for online user experience learning; Phase-0 nightly retraining from accumulated feedback |
| **Depends On** | clap, serde, serde_json, anyhow, flate2, walkdir, frustration-matrix, tempfile |

---

## kei-gateway

| Field | Value |
|-------|-------|
| **One-line Purpose** | P4.1 — Unified messaging gateway: platform adapters (CLI, Telegram, Discord, Slack), sessions, agent cache, delivery router |
| **Key API Exports** | `Gateway`, `PlatformAdapter`, `Session`, `MessageRouter` |
| **When to Use** | Multi-platform agent I/O; Telegram/Discord/Slack bridges (stubs in MVP, full impl P4.1.b); stateful session management; message delivery routing |
| **Depends On** | tokio, async-trait, serde, serde_json, anyhow, chrono, thiserror, lru, sqlx, blake3, dashmap, teloxide (opt), serenity (opt), slack-morphism (opt), tokio-tungstenite (opt), tempfile, pretty_assertions |

---

## kei-gdrive-import

| Field | Value |
|-------|-------|
| **One-line Purpose** | Project-folder classifier for one-shot Google Drive → Forgejo import — detects build manifests (Cargo.toml, package.json, pyproject.toml, etc.) |
| **Key API Exports** | `ProjectClassifier`, `Classification` |
| **When to Use** | Bulk-importing Google Drive folders into git; auto-detecting Rust/Node/Python/Go/Java projects; pre-migration validation |
| **Depends On** | clap, serde, serde_json, anyhow |

---

## kei-git-bitbucket

| Field | Value |
|-------|-------|
| **One-line Purpose** | Bitbucket Cloud GitBackend impl for kei-runtime-core (Wave 5) — REST v2.0 + HTTP Basic auth (username + app password) |
| **Key API Exports** | `BitbucketGitBackend`, `Repository`, `Credential` |
| **When to Use** | Bitbucket Cloud repository operations (create/mirror/clone/push); multi-cloud git backends; sibling to kei-git-{forgejo,gitea,gitlab} |
| **Depends On** | async-trait, serde, serde_json, thiserror, tokio, reqwest, base64, kei-runtime-core, wiremock |

---

## kei-git-forgejo

| Field | Value |
|-------|-------|
| **One-line Purpose** | GitBackend impl for public Forgejo (Gitea-compatible /api/v1) — Wave 5 atomar |
| **Key API Exports** | `ForgejoGitBackend`, `Repository` |
| **When to Use** | Codeberg-compatible Forgejo instances; public git backend operations; sibling to kei-git-{gitea,gitlab,bitbucket} for multi-platform flexibility |
| **Depends On** | async-trait, serde, serde_json, thiserror, tokio, reqwest, kei-runtime-core, wiremock, tempfile |

---

## kei-git-gitea

| Field | Value |
|-------|-------|
| **One-line Purpose** | GitBackend impl for Gitea (gitea.com / self-hosted) over /api/v1 — Wave 5 atomar |
| **Key API Exports** | `GiteaGitBackend`, `Repository`, `Organization` |
| **When to Use** | Gitea self-hosted instances; SaaS gitea.com; mirror/ensure_repo/clone/push operations; sibling to kei-git-{forgejo,gitlab,bitbucket} |
| **Depends On** | async-trait, serde, serde_json, thiserror, tokio, reqwest, kei-runtime-core, kei-shared, wiremock |

---

## kei-git-gitlab

| Field | Value |
|-------|-------|
| **One-line Purpose** | GitBackend impl for GitLab.com SaaS (and self-hosted via GITLAB_URL) — REST API v4 + PRIVATE-TOKEN auth + git CLI shell-out |
| **Key API Exports** | `GitlabGitBackend`, `Project`, `PathWithNamespace` |
| **When to Use** | GitLab.com SaaS projects; self-hosted GitLab instances; REST v4 API + git CLI hybrid; sibling to kei-git-{gitea,forgejo,bitbucket} |
| **Depends On** | async-trait, serde, serde_json, thiserror, tokio, reqwest, urlencoding, kei-runtime-core, wiremock |

---

## kei-graph-check

| Field | Value |
|-------|-------|
| **One-line Purpose** | Post-refactor graph-integrity gate — validates wikilinks, block refs, handoffs (v0.13.0) |
| **Key API Exports** | `GraphCheck`, `GraphIntegrity`, `RefValidation` |
| **When to Use** | CI gate for RULE 0.15 Phase C deep-sleep refactors; ensuring wikilink consistency post-merge; RULE 0.13 orchestrator branch verification |
| **Depends On** | clap, serde, serde_json, regex, walkdir, anyhow, tempfile |

---

## kei-graph-export

| Field | Value |
|-------|-------|
| **One-line Purpose** | Export KeiSei registry + ledger as D3 graph space fragment (visualization ready) |
| **Key API Exports** | `GraphExporter`, `D3Fragment`, `NodeData`, `LinkData` |
| **When to Use** | Visualizing agent taxonomies in browser; D3 graph rendering; kei-brain-view companion UI export |
| **Depends On** | rusqlite, serde, serde_json, clap, anyhow, toml |

---

## kei-graph-stream

| Field | Value |
|-------|-------|
| **One-line Purpose** | Tail agent-events.jsonl and stream to browser clients via WebSocket (real-time agent visualization) |
| **Key API Exports** | `GraphStreamServer`, `WebSocketHandler`, `EventStream` |
| **When to Use** | Live agent event visualization in kei-cortex UI; WebSocket pub-sub for multi-client agent monitoring; RULE 0.12 agent-git-model real-time DAG updates |
| **Depends On** | axum, tokio, serde, serde_json, clap, anyhow, tokio-tungstenite, reqwest, tempfile, futures |

---

## Summary

**Total crates catalogued:** 49 (6 non-kei + 43 kei-[a-g]*)

**Key architectural patterns:**
- **Layer-A convergence:** kei-entity-store as shared CRUD base for kei-{chat,content,crossdomain,discover,task}-store
- **Multi-provider pattern:** kei-{compute,auth,git}-* as trait impls (ComputeProvider, AuthProvider, GitBackend)
- **Graph substrate:** kei-{agent-runtime, fork, ledger, dna-index, brain-view, graph-*} form agent taxonomy backbone
- **Storage tiers:** rusqlite for local (SQLite bundled), sqlx for async SQL, kei-entity-store for schema-driven CRUD
- **Workflow automation:** kei-{decision, decompose, conflict-scan, graph-check} close research/audit/sleep → action pipelines

**Dependencies:** Predominantly Tokio async + Serde serialization. Minimal external deps (clap, chrono, regex, walkdir). **No ORM, no DI containers, no abstract factories.**

---

*Generated by code-implementer agent on 2026-05-02. See `/docs/encyclopedia/` for related catalogues.*
