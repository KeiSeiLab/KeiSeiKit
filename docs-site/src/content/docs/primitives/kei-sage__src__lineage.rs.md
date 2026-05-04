---
title: lineage.rs
path: kei-sage/src/lineage.rs
dna_hash: sha256:8942eb98fe045581
language: rust
size_loc: 195
generated: by-keidocs
---

# kei-sage/src/lineage.rs

Lineage traversal for primitive TOMLs.

Parses `[lineage]` section of capability.toml + manifest TOMLs,
extracting `parents` wikilinks, `created-by`, `fork-from`. Builds
an in-memory directed graph and walks ancestors + descendants.

## Public API

- Lineage metadata for a single primitive.
- `pub fn parse_lineage` — Parse a single TOML into a `LineageNode`, or `None` if unidentifiable.
- `pub fn discover_lineage` — Walk capabilities + manifests roots and parse every lineage node.
- Traversal result: ancestors (via parents + fork-from) and descendants.
- `pub fn trace_lineage` — BFS ancestors (follow parents + fork_from) + descendants (inverse edges).
- `pub fn nodes_by_author` — Filter + sort nodes by a creator id, return most-recent first (by created_at).

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, kei_atom_discovery, serde, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
