---
title: injection_check.rs
path: kei-pet/src/injection_check.rs
dna_hash: sha256:5ee735111cb24cd3
language: rust
size_loc: 91
generated: by-keidocs
---

# kei-pet/src/injection_check.rs

Injection check (sibling of `kei-memory::injection_guard`).

Constructor Pattern: orchestration only. Textual rules live in
`injection_check_textual.rs`; binary/blob heuristics live in
`injection_check_binary.rs`. Wire-point #2 of the three injection
guards described in `kei-memory/src/injection_guard.rs`. Mirrors the
Block-tier subset but stays inside `kei-pet`'s existing dep set
(no `regex` crate).

Bypass: `KEI_MEMORY_SKIP_GUARD=1` (shared env with kei-memory so
one-off recovery toggles both paths consistently).

## Public API

- One reason scan rejected a candidate string.
- `pub fn scan` — Scan `content`. Returns `Err` on the first Block-tier hit.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate

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
