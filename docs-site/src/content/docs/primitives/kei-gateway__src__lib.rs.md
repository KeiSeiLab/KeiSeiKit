---
title: lib.rs
path: kei-gateway/src/lib.rs
dna_hash: sha256:049024bcf5138e5f
language: rust
size_loc: 30
generated: by-keidocs
---

# kei-gateway/src/lib.rs

P4.1 — Unified messaging gateway.

Cross-platform message ingress (Telegram / Discord / Slack / CLI / WhatsApp /
Signal / Generic) → normalised [`MessageEvent`] → session-keyed agent run →
response delivery via [`DeliveryRouter`].

MVP scope: only the CLI adapter is fully implemented. Telegram / Discord /
Slack adapters are feature-gated stubs (Hermes-equivalent surface, todo!()
bodies). Full impls land in P4.1.b.

## Related

- parent: `kei-gateway/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
