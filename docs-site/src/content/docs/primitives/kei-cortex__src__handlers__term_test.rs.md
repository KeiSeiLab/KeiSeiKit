---
title: term_test.rs
path: kei-cortex/src/handlers/term_test.rs
dna_hash: sha256:e47a4dcfa6e77530
language: rust
size_loc: 142
generated: by-keidocs
---

# kei-cortex/src/handlers/term_test.rs

Unit tests for `term.rs`. We test the cwd resolver, the Origin
validator, and the PTY spawn seam. Driving an actual WS upgrade
requires an axum test server; that lives in the integration suite —
these tests focus on the synchronous halves.

## Public API

- Build a tempdir with one nested directory.
- Spawn-only smoke: the PTY allocates, `$SHELL` (or /bin/sh) launches,

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, std, tempfile, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
