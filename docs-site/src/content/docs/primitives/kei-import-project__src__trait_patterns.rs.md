---
title: trait_patterns.rs
path: kei-import-project/src/trait_patterns.rs
dna_hash: sha256:4a4a3d98d878e1f4
language: rust
size_loc: 199
generated: by-keidocs
---

# kei-import-project/src/trait_patterns.rs

trait_patterns — static dictionary of kei-runtime-core trait signatures.

Each `TraitPattern` describes one runtime trait by its required method
names, optional forbidden-dep cues, and indicator keywords that raise
confidence even when method names are abbreviated or wrapped.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- Every trait defined in kei-runtime-core.
- `ComputeProvider` — VM lifecycle (create/destroy/status/resize).
- `AuthProvider` — identity challenge/verify/revoke.
- `NotifyChannel` — push notifications (send).
- `GitBackend` — repo operations (clone/push/mirror/ensure_repo).
- `LlmBackend` — text completion (complete/context_window).
- `ServiceManager` — OS service lifecycle (install/start/stop/status).
- `MemoryBackend` — key-value memory store (store/query/compact).
- `Scheduler` — cron / one-shot task registration (register/cancel/list).
- `NetworkMode` — VPN / tunnel management (configure/teardown/peers).
- `Backup` — snapshot push/restore/prune.
- `CostGuard` — spend tracking and hard-kill budget management.
- `Observability` — structured log + metric emission (log/metric/flush).
- `pub struct TraitPattern` — Static description of one runtime trait's detection fingerprint.
- Method names that MUST appear in the source for a confident match.
- Crate names in `use` paths that disqualify this pattern (set to `[]`
- Free-text keywords in source that raise keyword-component confidence.
- `pub fn all_patterns` — All 12 trait patterns (11 actual traits + kei-runtime-core has 11 traits;

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
