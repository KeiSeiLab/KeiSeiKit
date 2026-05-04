---
title: ipsec_smoke.rs
path: kei-net-ipsec/tests/ipsec_smoke.rs
dna_hash: sha256:ce6e8ab26e23e07c
language: rust
size_loc: 120
generated: by-keidocs
---

# kei-net-ipsec/tests/ipsec_smoke.rs

Smoke tests for [`kei_net_ipsec::IpsecMode`].

All shell-out is routed through [`kei_net_ipsec::MockRunner`]; nothing
invokes a real `swanctl` binary, so these tests run without root and
without strongSwan installed.

## Related

- parent: `kei-net-ipsec/tests`
- imports: kei_net_ipsec, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
