---
title: execute_cmd.rs
path: kei-import-project/src/execute_cmd.rs
dna_hash: sha256:2034fb70fcc132f3
language: rust
size_loc: 78
generated: by-keidocs
---

# kei-import-project/src/execute_cmd.rs

execute_cmd — CLI orchestrator for Phase 5: migration plan execution.

Composes plan_parser → executor → phase_prompt → output.
Constructor Pattern: one responsibility, ≤100 LOC, ≤30 LOC per fn.

## Public API

- `pub fn run_execute` — Run the `execute` subcommand end-to-end.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, crate, std

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
