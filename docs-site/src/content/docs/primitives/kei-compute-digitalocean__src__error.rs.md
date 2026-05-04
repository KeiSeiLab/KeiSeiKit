---
title: error.rs
path: kei-compute-digitalocean/src/error.rs
dna_hash: sha256:49c264ae9acd8736
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-compute-digitalocean/src/error.rs

Local error type for the DigitalOcean backend.

Mapped into [`kei_runtime_core::Error`] via `From<Error>` so the trait
impls can use `?` against the runtime-core `Result`.

## Public API

- `pub type Result` — Crate-local result alias.
- Crate-local error variants.
- Transport / TLS / timeout failure from `reqwest`.
- Non-success HTTP status with the (best-effort) body text.
- DNA construction or parse failure.
- Local IO (env var read, etc.).
- JSON serialize / deserialize failure.
- Resource lookup miss (e.g. 404 on get_droplet).

## Related

- parent: `kei-compute-digitalocean/Cargo.toml`
- imports: kei_runtime_core, thiserror

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
