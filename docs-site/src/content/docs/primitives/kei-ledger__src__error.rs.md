---
title: error.rs
path: kei-ledger/src/error.rs
dna_hash: sha256:be5e309dad320dde
language: rust
size_loc: 76
generated: by-keidocs
---

# kei-ledger/src/error.rs

Error type for ledger operations that extend beyond raw SQL.

Constructor Pattern: one cube = one error type + its three trait impls.
Kept as a separate module so `ledger.rs` stays under the 200-LOC cap.

## Public API

- `pub const MAX_TREE_DEPTH` — Maximum depth walked by `ledger::tree()` before aborting with
- Errors from ledger ops that extend beyond raw SQL (tree walk + input
- BFS in `tree()` exceeded `MAX_TREE_DEPTH` iterations.
- Branch name longer than `MAX_BRANCH_LEN` chars (audit L1 cap).
- Attempted `fork` with a DNA that is already present in the ledger.
- v5 migration detected pre-existing duplicate DNAs in the agents

## Related

- parent: `kei-ledger/Cargo.toml`
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
