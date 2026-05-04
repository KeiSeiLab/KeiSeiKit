---
title: chat_memory_nudge.rs
path: kei-cortex/src/handlers/chat_memory_nudge.rs
dna_hash: sha256:1ec41e3ed7946cb5
language: rust
size_loc: 88
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_memory_nudge.rs

Wiring between the chat handler and the memory-nudge scheduler.

Constructor Pattern: this cube owns ONE responsibility — assembling
an `AgentContext` from a completed chat turn and firing
`MemoryNudgeScheduler::maybe_trigger`. Kept separate from
`chat_stream.rs` so neither file exceeds the 200-LOC ceiling.

Frozen-snapshot invariant: the conversation `Vec<Turn>` we pass to
the scheduler is freshly constructed on each call from the
user-message + assistant-text pair. We do NOT keep a running
conversation here — the stream handler is stateless across requests
and the scheduler treats each call independently. The Hermes
"snapshot" abstraction is satisfied by the read-only `RwLock`
handed to the review task at trigger time.

## Public API

- `pub const DEFAULT_PET_NAME` — Default pet-name used for the persist target when the request
- `pub fn build_context` — Build an `AgentContext` for a completed (user, assistant) turn
- `pub fn spawn_nudge` — Spawn `maybe_trigger` as a detached task. The chat handler does

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, kei_pet, std, tokio

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
