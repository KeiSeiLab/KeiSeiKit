---
title: main.rs
path: kei-registry/src/main.rs
dna_hash: sha256:27665f45c1002d70
language: rust
size_loc: 91
generated: by-keidocs
---

# kei-registry/src/main.rs

kei-registry binary entry point.

Constructor Pattern: parse CLI args; intercept `RegisterStatusTruth`
locally (Phase 3 Layer 3 pipe — keeps `handlers.rs` untouched);
delegate everything else to `handlers::dispatch`.
Exit codes: 0 success, 1 IO error, 2 not-found, 3 schema mismatch.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: clap, kei_registry, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
