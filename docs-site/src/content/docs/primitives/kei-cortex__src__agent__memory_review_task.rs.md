---
title: memory_review_task.rs
path: kei-cortex/src/agent/memory_review_task.rs
dna_hash: sha256:b8d79fb08b2719b6
language: rust
size_loc: 183
generated: by-keidocs
---

# kei-cortex/src/agent/memory_review_task.rs

Background memory-review task.

Constructor Pattern: this cube spawns a detached tokio task that
runs an ephemeral review agent. It does NOT own the agent surface
itself — it ports the Hermes `_spawn_background_review` shape
(run_agent.py:3267-3398) to a Rust async equivalent.

Wiring contract (intentionally narrow):
1. Caller hands us `ReviewHandles` (snapshot + memory-store Arc +
invoker callable + optional persist target).
2. We snapshot the conversation, append the review prompt, and
hand the bundle to `invoker`.
3. The invoker is responsible for the LLM round-trip. Reply text
lands in `ReviewOutcome.raw_reply`. When a `PersistRequest`
target is attached, a successful (non-short-circuit) reply is
written to the disk-backed memory store via `memory_persist`.

Tradeoffs:
* `Arc<dyn Invoker>` instead of generics keeps this file <200 LOC
and lets the scheduler treat all invokers uniformly. Cost: one
virtual call per review. Reviews fire ~every 10 turns — cost
is negligible vs the LLM round-trip dwarfing it.
* Detached `tokio::spawn` means the task lives independently of
the parent request. If the runtime shuts down mid-review, the
write may be lost. That's acceptable — the next session's
review will catch the same content. We deliberately do NOT
join() the handle: blocking the parent on a memory write
defeats the point of a background nudge.

## Public API

- Result of a single review pass.
- `pub trait Invoker` — Trait for the actual LLM round-trip. Production wires
- Hand the snapshot + appended review prompt to the model and
- `pub struct ReviewHandles` — Bundle handed to the spawned task. Cheap to construct — only
- Optional persistence target. When `Some`, a successful reply
- Subset of `PersistRequest` that's known at handle-build time —
- `pub fn from_context` — Build handles from a context. The invoker + persist target are
- `pub fn spawn_review` — Spawn a detached review task. Returns immediately. Invoker absent
- Run one review pass synchronously (the body of the spawned task).
- When a persist target is configured, fire a `spawn_blocking` write

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: kei_pet, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
