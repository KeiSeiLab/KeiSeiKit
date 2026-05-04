---
title: lib.rs
path: kei-ping/src/lib.rs
dna_hash: sha256:ccb5594d55c65f5c
language: rust
size_loc: 15
generated: by-keidocs
---

# kei-ping/src/lib.rs

`kei-ping` — cross-window agent heartbeat. Auto-selects backend.

Constructor Pattern: 1 trait + 2 impl-cubes (sqlite / redis) + 1 dispatcher.
Each cube ≤200 LOC, 1 responsibility.

## Related

- parent: `kei-ping/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
