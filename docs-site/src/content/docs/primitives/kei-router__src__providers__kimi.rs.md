---
title: kimi.rs
path: kei-router/src/providers/kimi.rs
dna_hash: sha256:29ec7b6c0d0e0f42
language: rust
size_loc: 233
generated: by-keidocs
---

# kei-router/src/providers/kimi.rs

Moonshot Kimi K2 provider (OpenAI-compatible). Reads `KIMI_API_KEY` (or `MOONSHOT_API_KEY`).
Cost 60/250 cents/MTok (VERIFIED 2026-04 fal.ai + openrouter.ai — Kimi K2 Thinking $0.60/$2.50 per MTok).

## Public API

- `pub const LEGACY_DEFAULT_MODEL` — Last-resort fallback if env + kei-model registry both miss (W55 Risk #1).
- Hardcoded fallback prices in cents/MTok (M-1: kept for offline/registry-miss case).
- `pub fn default_model` — 3-tier resolve: `KIMI_MODEL` env → registry role `kei-router-kimi` → legacy.
- Cache the kei-model registry once across all cost-method calls (M-1).
- M-1: derive cents/MTok from kei-model registry. Falls back when missing/placeholder.

## Related

- parent: `kei-router/Cargo.toml`
- imports: async_stream, async_trait, bytes, crate, futures, kei_model, serde, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
