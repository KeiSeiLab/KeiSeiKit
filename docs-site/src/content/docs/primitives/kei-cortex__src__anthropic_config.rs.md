---
title: anthropic_config.rs
path: kei-cortex/src/anthropic_config.rs
dna_hash: sha256:de4771e4b08c0295
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-cortex/src/anthropic_config.rs

Model-id and endpoint resolution for the Anthropic client.

Three-tier model fallback (env → kei-model registry → literal) and the
companion endpoint resolver live here so the HTTP client (`anthropic.rs`)
and the tool-use invoker (`anthropic_invoker.rs`) consume a single
resolver surface. Stateless: every call re-reads the underlying env
var so rotation works without restarting the process.

## Public API

- `pub const MODEL_FALLBACK` — Literal model-id fallback. Used when `ANTHROPIC_MODEL` is unset and the
- Selector role consulted when no env override is set. Matches the
- `pub const ENDPOINT` — Anthropic API endpoint (v1 messages). Compile-time default; the
- `pub const API_VERSION` — Anthropic API version header value.
- `pub fn default_model` — Resolve the Claude model id (W55 Stage 2, mirrors W55b kei-spawn).
- Inner registry resolve — returns `None` on any failure (missing TOML,
- `pub fn endpoint` — Resolve the live Anthropic endpoint. Reads `ANTHROPIC_ENDPOINT` at
- Serializes env-var mutation across `default_model_*` tests so the

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std

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
