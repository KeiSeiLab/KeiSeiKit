---
title: state.rs
path: kei-cortex/src/state.rs
dna_hash: sha256:873571422e45d7a8
language: rust
size_loc: 184
generated: by-keidocs
---

# kei-cortex/src/state.rs

Shared state passed to every handler via `axum::extract::State`.

Holds the loaded configuration, the bearer token, the LLM provider router
(Wave 32 v0.40 multi-provider abstraction), and a per-user lock registry
used by expensive side-effecting handlers (portrait install) to serialize
work against the same `user_id`.

Hermes P2.2.b: the memory-review scheduler + an invoker factory live
here so the chat handler can fire `maybe_trigger` after each turn
without re-discovering wiring. The factory closure rebuilds the
Anthropic invoker per call so the API key is read fresh (env-rotation
friendly, mirrors `anthropic::open_stream`).

## Public API

- Hard cap on how many distinct `user_id` mutexes we keep alive. Anything
- `pub type InvokerFactory` — Type alias for the per-call invoker factory. Each `()` invocation
- Read-only handler state (cheaply cloneable via `Arc`).
- Bounded LRU registry of per-user mutexes. Capped at
- Token-event store. `None` when the configured path could not be
- `pub fn new` — Construct new state from a validated config and bearer token.
- `pub fn with_router` — Test-friendly constructor that takes an explicit router (e.g.
- `pub fn with_router_and_factory` — Full-control constructor: caller passes router AND invoker
- `pub fn with_router_factory_and_tracker` — Test-only escape hatch: caller supplies the token-tracker handle
- `pub fn config` — Borrow the configuration.
- `pub fn token` — Borrow the bearer token.
- `pub fn router` — Borrow the LLM provider router (cheap clone via `Arc`).
- `pub fn scheduler` — Borrow the memory-review scheduler. Cheap clone via `Arc`.
- `pub fn build_memory_invoker` — Build a fresh memory-review invoker via the configured factory.
- `pub fn token_tracker` — Borrow the token-event tracker. `None` when the configured DB
- Return the per-user mutex, creating it on first access. The returned
- Test-only accessor: current size of the per-user-lock registry.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, kei_router, kei_token_tracker, lru, std, tokio

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
