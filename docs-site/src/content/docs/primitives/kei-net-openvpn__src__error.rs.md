---
title: error.rs
path: kei-net-openvpn/src/error.rs
dna_hash: sha256:d8610526c2f4bca3
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-net-openvpn/src/error.rs

Crate-local error. Bridges into `kei_runtime_core::Error` so
`NetworkMode` impl methods can `?` through this and surface a
substrate-level error to callers without leaking systemctl /
management-socket transport details.

## Public API

- Bridge into substrate-level error. OpenVPN-specific failures are

## Related

- parent: `kei-net-openvpn/Cargo.toml`
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
