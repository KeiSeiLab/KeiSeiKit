---
title: lib.rs
path: frustration-matrix/src/lib.rs
dna_hash: sha256:aebb08b0e6cc224c
language: rust
size_loc: 26
generated: by-keidocs
---

# frustration-matrix/src/lib.rs

Library surface for `frustration-matrix`.

Exposes the byte-level n-gram firmware (training + likelihood scoring),
the regex-category SSoT, and the JSONL chatlog parser for reuse by
sibling crates:
* `dna-store::axis_semantic` — consumes `firmware`
* `kei-frustration-loop` — consumes `firmware`, `firmware_corpus`,
`categories`, `jsonl` to drive the per-user online learning loop

Kept deliberately narrow: only modules the binary AND external sibling
crates need to consume are public. Internal helpers (scan, report,
classifier, eval) stay private to the binary.

The binary (`main.rs`) continues to compile independently with its own
`mod firmware;` declarations — library and binary share source files
via Cargo's dual-target rule, not via re-use from one to the other.

Constructor Pattern: this cube is pure re-export. Any behaviour change
happens inside the individual `firmware*.rs` / `jsonl.rs` /
`categories.rs` cubes, not here.

## Related

- parent: `frustration-matrix/Cargo.toml`

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
