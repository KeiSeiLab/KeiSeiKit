---
title: ts_parse.rs
path: kei-db-contract/src/ts_parse.rs
dna_hash: sha256:4d25fc1c8225df44
language: rust
size_loc: 126
generated: by-keidocs
---

# kei-db-contract/src/ts_parse.rs

TS parser cube: shallow regex extraction of `type X = { ... }` and `interface X { ... }`.

## Public API

- One declared field on a TS type / interface.
- One declared `type` alias or `interface` block.
- `pub fn parse_ts_glob` — Walk a directory recursively, parse every `.ts` and `.tsx` file.
- `pub fn extract_ts_types` — Extract every type/interface block from a single TS source string.

## Related

- parent: `kei-db-contract/Cargo.toml`
- imports: anyhow, regex, serde, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
