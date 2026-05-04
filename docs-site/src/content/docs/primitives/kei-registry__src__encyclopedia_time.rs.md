---
title: encyclopedia_time.rs
path: kei-registry/src/encyclopedia_time.rs
dna_hash: sha256:cdd6aba91e40fd69
language: rust
size_loc: 74
generated: by-keidocs
---

# kei-registry/src/encyclopedia_time.rs

Minimal ISO-8601 UTC timestamp without external crate dependencies.

Constructor Pattern: one pure function `utc_now()` with helpers.
No std-external time crate required — stdlib `SystemTime` only.

## Public API

- `pub fn utc_now` — Return current UTC time as ISO-8601 string (`YYYY-MM-DDTHH:MM:SSZ`).

## Related

- parent: `kei-registry/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
