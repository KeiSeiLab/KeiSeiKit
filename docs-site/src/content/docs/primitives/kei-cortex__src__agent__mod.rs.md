---
title: mod.rs
path: kei-cortex/src/agent/mod.rs
dna_hash: sha256:e6bbaff9c5767f1c
language: rust
size_loc: 18
generated: by-keidocs
---

# kei-cortex/src/agent/mod.rs

Agent-side helpers — periodic memory review, nudge scheduling, and
the prompt template used by the background reviewer.

Constructor Pattern: each cube is a single-responsibility module.
Public surface is intentionally narrow — only the scheduler trigger
and the review-task entry point are reachable from outside.

Frozen-snapshot invariant: nothing in this module mutates the
parent agent's in-flight system prompt. Background reviews write
exclusively to disk-backed memory stores. The next session picks
up the new snapshot via the normal load path; the running session
is left undisturbed (preserves Anthropic prefix-cache hits).

## Related

- parent: `kei-cortex/Cargo.toml`

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
