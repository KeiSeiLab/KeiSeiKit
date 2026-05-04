---
title: state_factories.rs
path: kei-cortex/src/state_factories.rs
dna_hash: sha256:01442356466cd766
language: rust
size_loc: 64
generated: by-keidocs
---

# kei-cortex/src/state_factories.rs

Construction helpers extracted from `state.rs` to keep that file under
the Constructor-Pattern 200-LOC ceiling after the Wave 44d resource-cap
work added an LRU registry for per-user locks.

Hosts:
- `default_invoker_factory` — builds the Anthropic memory-review
invoker fresh on every call so env-rotated API keys take effect.
- `default_review_system` — short, stable system slot for review.
- `open_token_tracker` — opens the token-event SQLite store and
gracefully degrades to `None` when the host has no telemetry dir.

## Public API

- Default Anthropic-backed invoker factory. Each call rebuilds the
- System-slot text for memory-review calls. Kept short and stable
- Try to open the token-event store at the configured path. Returns

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, kei_token_tracker, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
