---
title: openai.rs
path: kei-router/src/providers/openai.rs
dna_hash: sha256:553479087ecb1166
language: rust
size_loc: 240
generated: by-keidocs
---

# kei-router/src/providers/openai.rs

OpenAI Chat Completions streaming provider. Translates `delta.content` SSE → `StreamEvent::Token`.
Cost 15/60 cents/MTok [VERIFIED: https://openai.com/api/pricing/ on 2026-04-28] — gpt-4o-mini.

## Public API

- `pub const LEGACY_DEFAULT_MODEL` — Last-resort fallback if env + kei-model registry both miss (W55 Risk #1).
- Hardcoded fallback prices in cents/MTok (M-1: kept for offline/registry-miss case).
- `pub fn default_model` — 3-tier resolve: `OPENAI_MODEL` env → registry role `kei-router-openai` → legacy.
- Cache the kei-model registry once across all cost-method calls (M-1).
- M-1: derive cents/MTok from kei-model registry. Falls back when missing/placeholder.

## Related

- parent: `kei-router/Cargo.toml`
- imports: async_stream, async_trait, bytes, crate, futures, kei_model, serde, std, tokio

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
