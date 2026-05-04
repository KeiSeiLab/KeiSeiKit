---
title: group.rs
path: kei-changelog/src/group.rs
dna_hash: sha256:4c914b537e680696
language: rust
size_loc: 39
generated: by-keidocs
---

# kei-changelog/src/group.rs

Group commits by kind, preserving insertion order within each bucket.

## Public API

- Commits grouped by `CommitKind`, sorted by `CommitKind::sort_key`.
- Build a `Grouped` from an ordered slice of commits.

## Related

- parent: `kei-changelog/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
