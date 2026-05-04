---
title: lib.rs
path: kei-forge/src/lib.rs
dna_hash: sha256:65cd967d001211ca
language: rust
size_loc: 21
generated: by-keidocs
---

# kei-forge/src/lib.rs

kei-forge — local web wizard for scaffolding new atoms per the locked
SUBSTRATE-SCHEMA.md contract.

Architecture (Constructor Pattern, one responsibility per file):
- [`server`]     — axum router + handlers
- [`middleware`] — DNS-rebinding + CSRF defences
- [`headers`]    — CSP / nosniff / frame-deny / referrer headers
- [`html`]       — static HTML form (JSON-over-fetch)
- [`form`]       — request deserialization + validation
- [`generate`]   — pure-Rust atom templating (no shell-out)

Public entry point is [`server::app`], which returns the fully-wired
`axum::Router` ready to be served by any bind target (production =
127.0.0.1:8747; tests = random ephemeral port).

## Related

- parent: `kei-forge/Cargo.toml`

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
