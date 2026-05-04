---
title: anthropic.rs
path: kei-router/src/providers/anthropic.rs
dna_hash: sha256:10440ca81fc28f22
language: rust
size_loc: 246
generated: by-keidocs
---

# kei-router/src/providers/anthropic.rs

Anthropic Messages API provider. Streams `text_delta` events.
Cost 100/500 cents/MTok (VERIFIED 2026-04 anthropic.com/pricing — Haiku 4.5: $1/$5 per MTok).

## Public API

- `pub const LEGACY_DEFAULT_MODEL` — Last-resort fallback if env + kei-model registry both miss (W55 Risk #1).
- Hardcoded fallback prices in cents/MTok (M-1: kept for offline/registry-miss case).
- `pub fn default_model` — 3-tier resolve: `ANTHROPIC_MODEL` env → registry role `kei-router-anthropic` → legacy.
- Cache the kei-model registry once across all cost-method calls (M-1).
- M-1: derive cents/MTok from kei-model registry. Falls back to hardcoded
- `pub fn from_env` — Construct from env (`ANTHROPIC_API_KEY`). Returns `None` if unset.
- `pub fn with_endpoint` — For tests: explicit endpoint override.

## Related

- parent: `kei-router/Cargo.toml`
- imports: async_stream, async_trait, bytes, crate, futures, kei_model, serde, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
