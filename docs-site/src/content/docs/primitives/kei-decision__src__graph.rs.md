---
title: graph.rs
path: kei-decision/src/graph.rs
dna_hash: sha256:a217477195b96f96
language: rust
size_loc: 102
generated: by-keidocs
---

# kei-decision/src/graph.rs

Cumulative research-graph merger.

Walks a research root, picks up each per-topic `graph.json`, merges into
a single cumulative graph file. Schema is intentionally permissive — we
treat each graph.json as `{ "nodes": [...], "edges": [...] }` and append
to a master accumulator, dedup'ing by `id` for nodes and (`from`, `to`)
for edges.

## Public API

- `pub fn merge_graphs` — Walk `research_dir` for `graph.json` siblings of `MASTER-REPORT.md`,

## Related

- parent: `kei-decision/Cargo.toml`
- imports: anyhow, serde, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
