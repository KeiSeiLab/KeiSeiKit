---
title: main.rs
path: kei-discover/src/main.rs
dna_hash: sha256:a9ed4b2e4bd209fd
language: rust
size_loc: 129
generated: by-keidocs
---

# kei-discover/src/main.rs

kei-discover CLI — register / list / search / install / stats.

Metadata-only: `install` flips the local `installed` flag but does
NOT fetch anything. Real federation (remote index, fetch, signature
verify) arrives in a future wave.

Exit-code contract: 2 for validation / duplicate / not-found, 1 for
storage / IO, 0 on success (matches kei-entity-store convention).

## Related

- parent: `kei-discover/Cargo.toml`
- imports: clap, kei_discover, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
