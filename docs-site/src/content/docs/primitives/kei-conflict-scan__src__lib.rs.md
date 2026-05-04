---
title: lib.rs
path: kei-conflict-scan/src/lib.rs
dna_hash: sha256:b266d5a74ce795f4
language: rust
size_loc: 16
generated: by-keidocs
---

# kei-conflict-scan/src/lib.rs

kei-conflict-scan — library surface.

Detects inconsistencies inside a `~/.claude`-style root:
- rule conflicts (contradictory directives in `rules/*.md`)
- hook overlap (two hooks on same matcher)
- block duplication (>70% text overlap in `_blocks/*.md`)
- orphan refs (wikilinks / handoffs to non-existent files)
- Constructor-Pattern violations (file >200 LOC / fn >30 LOC)

Produces a JSON array consumable by `kei-refactor-engine`.

## Related

- parent: `kei-conflict-scan/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
