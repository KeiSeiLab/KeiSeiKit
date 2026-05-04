---
title: tool_apply_INTEGRATION
path: kei-cortex/src/handlers/tool_apply_INTEGRATION.md
dna_hash: sha256:dc6f6f23ffd20f7c
language: markdown
size_loc: 121
generated: by-keidocs
---

# kei-cortex/src/handlers/tool_apply_INTEGRATION.md

## Public API

- `Integration handoff — `tool_apply.rs` → cortex-ui DiffPane` — Endpoint: `POST /api/v1/cortex/tool/apply`. Wired in `routes.rs` inside the
- `Trust posture (load-bearing)` — This endpoint applies file edits proposed by the agentic loop directly to
- `What is NOT yet enforced (future Wave)` — - **Per-conversation provenance**: signature verification tying an
- `Wire shape` — Request:
- `UI compatibility` — The current `applyToolEdit` in
- `Edit semantics summary` — - `replace_all=false` (default): exactly one occurrence required;
- `Write safety summary` — - New path → write succeeds and creates parent dirs as needed.
- `Atomicity` — Both edit and write stage to a sibling tempfile inside the destination

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
