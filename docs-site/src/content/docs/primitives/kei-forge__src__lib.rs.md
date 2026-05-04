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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
