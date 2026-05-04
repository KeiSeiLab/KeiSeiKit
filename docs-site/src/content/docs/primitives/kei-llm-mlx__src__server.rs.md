---
title: server.rs
path: kei-llm-mlx/src/server.rs
dna_hash: sha256:253ebb77a29197e0
language: rust
size_loc: 72
generated: by-keidocs
---

# kei-llm-mlx/src/server.rs

Local OpenAI-compat HTTP server — `mlx_lm.server`.

Constructor Pattern: this cube builds the spawn argv and returns a
`ServerHandle` describing the still-attached child PID + bound URL.
Bind is FORCED to localhost — `--host 0.0.0.0` (or any non-loopback
literal) is rejected with `Error::SecurityRefused`. Remote-binding
decisions belong to the operator with explicit configuration, not to
this primitive.

## Public API

- `pub fn build_spec` — Validate spec + build argv. Does NOT spawn — callers spawn through
- `pub fn build_argv` — Build argv for `mlx_lm.server`. Visible for tests.
- `pub fn openai_compat_url` — Compose the OpenAI-compat URL the consumer will hit.
- `pub fn is_localhost` — Localhost predicate. Treats `localhost`, `127.0.0.1`, `::1` as

## Related

- parent: `kei-llm-mlx/Cargo.toml`
- imports: crate, serde

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
