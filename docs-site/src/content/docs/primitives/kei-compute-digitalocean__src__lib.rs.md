---
title: lib.rs
path: kei-compute-digitalocean/src/lib.rs
dna_hash: sha256:81244532ff12fd1b
language: rust
size_loc: 20
generated: by-keidocs
---

# kei-compute-digitalocean/src/lib.rs

kei-compute-digitalocean — DigitalOcean impl of [`kei_runtime_core::ComputeProvider`].

Layout:
- [`error`]: local `Error`/`Result` mapping into the runtime-core error.
- [`client`]: thin async REST v2 wrapper (mockable base URL).
- [`backend`]: [`DigitalOceanBackend`] — DNA-bearing trait impl.

Auth: bearer `DIGITALOCEAN_TOKEN`. Base URL defaults to
`https://api.digitalocean.com/v2` and is overridable for tests.

## Related

- parent: `kei-compute-digitalocean/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
