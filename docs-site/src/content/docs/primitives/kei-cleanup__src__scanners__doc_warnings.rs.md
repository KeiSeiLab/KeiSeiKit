---
title: doc_warnings.rs
path: kei-cleanup/src/scanners/doc_warnings.rs
dna_hash: sha256:38dbd35dc07a3c97
language: rust
size_loc: 142
generated: by-keidocs
---

# kei-cleanup/src/scanners/doc_warnings.rs

Doc-warnings scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 + Appendix A.

Shells out to `cargo doc --no-deps --workspace -- -D warnings`, then
parses stderr for `warning:` lines; each warning that carries a
`--> <file>:<line>:<col>` follow-up becomes one Finding.

Pipe-drain pattern from kei-arch-map cargo_check.rs (Phase 1.6) so a
large workspace doesn't deadlock the child on a 64 KiB pipe buffer.
Wall-clock cap 120 s via `wait-timeout`.

## Public API

- `pub fn scan` — Public scanner entry — see Appendix A row "doc_warnings".
- Parse cargo stderr for `warning:` and following `--> path:line:col`.

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: crate, std, wait_timeout

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
