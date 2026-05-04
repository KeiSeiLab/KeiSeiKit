---
title: wireguard_smoke.rs
path: kei-net-wireguard/tests/wireguard_smoke.rs
dna_hash: sha256:6795a6fcd6ada33a
language: rust
size_loc: 130
generated: by-keidocs
---

# kei-net-wireguard/tests/wireguard_smoke.rs

Smoke tests for `kei-net-wireguard`. Drive the `WireguardMode` impl
through a recording [`MockRunner`] — never invokes a real `wg-quick`.

## Public API

- Records every `(cmd, args)` invocation and replays a programmable

## Related

- parent: `kei-net-wireguard/tests`
- imports: anyhow, kei_net_wireguard, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
