---
title: wave44b_INTEGRATION
path: kei-cortex/src/handlers/wave44b_INTEGRATION.md
dna_hash: sha256:8a29928a512f6a64
language: markdown
size_loc: 91
generated: by-keidocs
---

# kei-cortex/src/handlers/wave44b_INTEGRATION.md

## Public API

- `Wave 44b — orchestrator integration notes` — Disjoint scope of this fork: `handlers/tool_apply.rs` + `handlers/term.rs`
- `Files in this fork` — - **NEW** `handlers/tool_apply_atomic.rs` (173 LOC) — symlink-safe
- `Conflict with wave44a (`tool/atomic_io.rs` extraction)` — Wave 44a extracts the existing `tool::write::atomic_write` into a
- `Ledger of behavioural changes (for QA)` — 1. `/tool/apply` now refuses overwrites that would canonicalize
- `Testing` — - `cargo test -p kei-cortex --lib` should run all tool_apply/term

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
