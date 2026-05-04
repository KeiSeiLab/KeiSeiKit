---
title: error.rs
path: kei-net-ipsec/src/error.rs
dna_hash: sha256:c0e5691e2a0af3bb
language: rust
size_loc: 52
generated: by-keidocs
---

# kei-net-ipsec/src/error.rs

Local error type for the IPsec / strongSwan adapter.

Mapped into [`kei_runtime_core::Error`] via `From<Error>` so the
[`crate::network::IpsecMode`] trait impls can use `?` against the
runtime-core `Result`.

## Public API

- `pub type Result` — Crate-local result alias.
- Crate-local error variants.
- `swanctl` invocation completed but returned a non-zero exit code.
- Local IO / spawn failure (e.g. `swanctl` binary missing on PATH).
- Parser could not extract a structured `PeerStatus` from the
- DNA construction failure.

## Related

- parent: `kei-net-ipsec/Cargo.toml`
- imports: thiserror

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
