---
title: openvpn_smoke.rs
path: kei-net-openvpn/tests/openvpn_smoke.rs
dna_hash: sha256:e08c2d066aa39ebb
language: rust
size_loc: 161
generated: by-keidocs
---

# kei-net-openvpn/tests/openvpn_smoke.rs

Smoke tests for `OpenvpnMode`. We use a recording `MockRunner`
instead of `SystemRunner`, so these tests are hermetic — no
`systemctl`, no UNIX socket, no live OpenVPN.

## Public API

- Per-call result. If empty, default to status=0.

## Related

- parent: `kei-net-openvpn/tests`
- imports: kei_net_openvpn, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
