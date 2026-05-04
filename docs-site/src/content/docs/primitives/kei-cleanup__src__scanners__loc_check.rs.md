---
title: loc_check.rs
path: kei-cleanup/src/scanners/loc_check.rs
dna_hash: sha256:8226f4c31dbfc252
language: rust
size_loc: 93
generated: by-keidocs
---

# kei-cleanup/src/scanners/loc_check.rs

Constructor-Pattern LOC scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 +
Appendix A. Walks workspace, counts file LOC and per-function body
LOC; flags violations against `[loc]` config (defaults 200 / 30).

Body-line measurement lives in [`super::fn_extract`] to keep this
module under the Constructor-Pattern 200-LOC limit (Rule ZERO).

## Public API

- `pub fn scan` — Public scanner entry — see Appendix A row "loc_check".

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: crate, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
