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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
