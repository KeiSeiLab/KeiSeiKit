---
title: lib.rs
path: kei-leak-matrix/src/lib.rs
dna_hash: sha256:2cc30dd7ff8f03c5
language: rust
size_loc: 15
generated: by-keidocs
---

# kei-leak-matrix/src/lib.rs

kei-leak-matrix — single source of truth for content protection patterns.

See `security/leak-matrix.toml` for the SSoT data file. This crate
parses it once, compiles every regex upfront, and exposes scan +
substitute helpers used by hooks (no-github-push, sync-public.sh,
secrets-guard, genesis-leak-guard) and by ad-hoc CLI use.

## Related

- parent: `kei-leak-matrix/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
