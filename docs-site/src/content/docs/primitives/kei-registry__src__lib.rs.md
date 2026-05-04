---
title: lib.rs
path: kei-registry/src/lib.rs
dna_hash: sha256:ed537fa3aabe9a8e
language: rust
size_loc: 52
generated: by-keidocs
---

# kei-registry/src/lib.rs

kei-registry — universal block identity layer.

Generalises the agent DNA pattern (kei-shared::dna + kei-ledger) to ANY
kit block: primitive crate, skill, rule, hook, atom. One SQLite store
at `~/.claude/registry.sqlite`, one `<block_type>::<caps>::<scope_sha8>::
<body_sha8>-<nonce8>` DNA wire format per block, idempotent re-register,
supersede chain on body change.

Constructor Pattern: each module is one cube with one responsibility.
Wire-format SSoT lives in `kei_shared::dna` — `dna_block::compose_for_block`
delegates to `kei_shared::compose_dna` so the format string exists in
exactly one place.

## Related

- parent: `kei-registry/Cargo.toml`

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
