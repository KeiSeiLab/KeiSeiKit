---
title: dead_code.rs
path: kei-cleanup/src/scanners/dead_code.rs
dna_hash: sha256:f3759f5a6ca1116e
language: rust
size_loc: 185
generated: by-keidocs
---

# kei-cleanup/src/scanners/dead_code.rs

Dead-code / unused-dep scanner via external CLI — see
CLEANUP-RUNTIME-SPEC.md §3.7 + §8.3 (graceful fallback).

Strategy:
1. Try `cargo machete --with-metadata` (stable). Parse stdout.
2. Else try `cargo +nightly udeps --workspace`. Parse output.
3. Else return `ToolNotFound` — runtime records as `scanners_skipped`.

v0.1: machete-only parsing implemented; udeps invocation wired but
its output parser is conservative (best-effort line scan). Both
branches return the same `Finding` shape to keep callers simple.

## Public API

- `pub fn scan` — Public scanner entry — see Appendix A row "dead_code".
- Parse cargo-machete stdout. Lines look like:

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: crate, std

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
