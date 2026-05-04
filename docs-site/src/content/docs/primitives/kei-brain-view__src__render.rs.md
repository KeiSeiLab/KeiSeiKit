---
title: render.rs
path: kei-brain-view/src/render.rs
dna_hash: sha256:9c8e0638b6b04ed7
language: rust
size_loc: 138
generated: by-keidocs
---

# kei-brain-view/src/render.rs

ASCII tree rendering with optional ANSI color.

Constructor Pattern: one cube = render entrypoints + color helpers.
Colors obey `NO_COLOR` convention (https://no-color.org) — present
env var of any value disables ANSI escape codes at runtime.

## Public API

- `pub fn render_ascii` — Render the full graph as an indented text tree (roots first, BFS
- `pub fn render_ascii_with_color` — Explicit-color variant — used by tests to assert color-free output.
- `pub fn render_lineage` — Render the lineage (ancestors + focus + descendants) for a single DNA.

## Related

- parent: `kei-brain-view/Cargo.toml`
- imports: crate, std

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
