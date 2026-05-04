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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
