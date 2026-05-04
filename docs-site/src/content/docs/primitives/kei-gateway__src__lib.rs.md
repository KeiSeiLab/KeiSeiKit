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
