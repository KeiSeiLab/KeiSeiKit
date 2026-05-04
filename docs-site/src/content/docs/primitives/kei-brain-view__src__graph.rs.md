---
title: graph.rs
path: kei-brain-view/src/graph.rs
dna_hash: sha256:aeddef0d58978359
language: rust
size_loc: 147
generated: by-keidocs
---

# kei-brain-view/src/graph.rs

In-memory taxonomy graph built from the kei-ledger SQLite file.

Constructor Pattern: one cube = Node + Graph + adjacency builder.
Read-only: we never write to the ledger, only hydrate a view.

## Public API

- One hydrated ledger row reduced to the fields brain-view renders.
- In-memory adjacency over the `agents` table.
- `pub fn node` — Look up a node by id; index is the slot in `self.nodes`.
- `pub fn build_graph` — Read all rows from the ledger's `agents` table and build the graph.
- `pub fn resolve_dna` — Resolve a DNA prefix to a unique node id. Returns `DnaNotFound` if no

## Related

- parent: `kei-brain-view/Cargo.toml`
- imports: crate, rusqlite, serde, std

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
