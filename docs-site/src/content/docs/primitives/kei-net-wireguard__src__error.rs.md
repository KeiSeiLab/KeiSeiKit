---
title: error.rs
path: kei-net-wireguard/src/error.rs
dna_hash: sha256:48ddaa50f3ad7294
language: rust
size_loc: 46
generated: by-keidocs
---

# kei-net-wireguard/src/error.rs

Error types for `kei-net-wireguard`. Maps cleanly into
`kei_runtime_core::Error` so `WireguardMode` can fulfill `NetworkMode`.

## Public API

- `wg-quick`/`wg` exited non-zero. Carries the rendered command line +
- Underlying I/O failure (spawn / read / wait).
- `wg show ... dump` produced output we could not parse.
- DNA construction failed.

## Related

- parent: `kei-net-wireguard/Cargo.toml`
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
