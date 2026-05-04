---
title: memory_nudge.rs
path: kei-cortex/src/agent/memory_nudge.rs
dna_hash: sha256:eb164d82d4ee7d02
language: rust
size_loc: 149
generated: by-keidocs
---

# kei-cortex/src/agent/memory_nudge.rs

Periodic memory-review scheduler.

Constructor Pattern: this cube owns the *trigger* decision only.
The actual review work lives in `memory_review_task`. The split
exists so the scheduler is unit-testable without spinning up a
tokio runtime: `should_trigger()` is sync and pure.

Frozen-snapshot invariant (CRITICAL): the in-flight system prompt
of the parent agent is NEVER mutated by background reviews. Reviews
write only to the disk-backed memory store via `PersistTarget`.
The next session loads the new snapshot at startup; the running
session keeps its prefix-cache hits intact. This mirrors Hermes'
`_system_prompt_snapshot` discipline (memory_tool.py:122).

## Public API

- One conversation turn — minimal stub the scheduler needs to count.
- `pub struct AgentContext` — Snapshot of context the scheduler hands to the review task. Held
- `pub fn new` — Construct a fresh context. Use `with_invoker`/`with_persist` to
- `pub struct MemoryNudgeScheduler` — Scheduler state. `interval` is the number of *user* turns between
- `pub fn with_cooldown_secs` — Test/diagnostic constructor that lets us shrink the cooldown
- Register a new user turn and possibly fire a review. Returns
- `pub fn should_trigger_count` — Pure predicate — exposed for tests so they don't need a runtime.
- `pub fn reset` — Reset the counter — called e.g. when the session is cleared.
- `pub fn current_count` — Read-only counter accessor for diagnostics.
- `pub fn default_scheduler` — Build a scheduler with the Hermes default (every 10 user turns).

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std, tokio

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
