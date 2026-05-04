---
title: cli_roundtrip.rs
path: kei-gateway/tests/cli_roundtrip.rs
dna_hash: sha256:2bb887d1f1436e06
language: rust
size_loc: 63
generated: by-keidocs
---

# kei-gateway/tests/cli_roundtrip.rs

End-to-end test for the CLI adapter wiring.

Verifies the adapter constructs a [`MessageEvent`] with the right shape
and that an `OutboundMessage` can be sent without error. Full stdin/stdout
piping is platform-specific and exercised in integration tests; here we
cover the in-process contract.

## Related

- parent: `kei-gateway/tests`
- imports: kei_gateway

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
