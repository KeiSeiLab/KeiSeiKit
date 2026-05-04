---
title: orphans.rs
path: kei-conflict-scan/src/scanners/orphans.rs
dna_hash: sha256:168c32d0ac4c77f1
language: rust
size_loc: 74
generated: by-keidocs
---

# kei-conflict-scan/src/scanners/orphans.rs

Orphan-reference detector.

Finds `[[wikilink]]` and `handoffs: - name` references whose targets
do not exist anywhere under the root. Case-insensitive basename match.

## Related

- parent: `kei-conflict-scan/Cargo.toml`
- imports: crate, regex, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
