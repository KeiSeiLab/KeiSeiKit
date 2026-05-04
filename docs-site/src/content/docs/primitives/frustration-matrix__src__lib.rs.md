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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
