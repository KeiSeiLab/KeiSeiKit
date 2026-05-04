---
title: token_tracking.rs
path: kei-cortex/tests/token_tracking.rs
dna_hash: sha256:f78e1986a1d1ed0a
language: rust
size_loc: 186
generated: by-keidocs
---

# kei-cortex/tests/token_tracking.rs

Phase 2 token-tracker wiring integration witnesses.

Drives `/v1/chat/completions` (sync) against a mock Anthropic that
returns `input_tokens=10, output_tokens=5`, with the AppState wired
to an in-memory [`kei_token_tracker::Store`]. Asserts that exactly
one [`TokenEvent`] is recorded after the call returns, with the
expected token counts.

Companion to `openai_loop_wiring.rs` — that file proves the loop is
the production path; this one proves the loop's per-turn telemetry
reaches the tracker store.

## Public API

- Build a mock Anthropic server that returns input_tokens=10,
- AppState wired to an explicit in-memory tracker handle so the test
- Stand-in for the memory-review invoker factory — never called by
- End-to-end: sync /v1/chat/completions records exactly one TokenEvent

## Related

- parent: `kei-cortex/tests`
- imports: axum, kei_cortex, kei_router, kei_token_tracker, std, tokio, tower

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
