---
title: topo.rs
path: kei-pipe/src/topo.rs
dna_hash: sha256:b74210560c3c1212
language: rust
size_loc: 98
generated: by-keidocs
---

# kei-pipe/src/topo.rs

Kahn-style topological sort for the parsed DAG.

Split out from `dag.rs` to stay under the Constructor Pattern 200-LOC
limit. Stable — ties are broken by declaration order so reports are
deterministic across runs.

## Public API

- `pub fn topo_sort` — Topologically sort the DAG. Returns `&Step` references in execution

## Related

- parent: `kei-pipe/Cargo.toml`
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
