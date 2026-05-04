---
title: quality_constructor_pattern.rs
path: kei-agent-runtime/src/verifies/quality_constructor_pattern.rs
dna_hash: sha256:8906c39a449df5b8
language: rust
size_loc: 102
generated: by-keidocs
---

# kei-agent-runtime/src/verifies/quality_constructor_pattern.rs

`quality::constructor-pattern` — walks the run dir, asserts every `.rs`
file ≤ 200 LOC and every top-level `fn` ≤ 30 LOC.

## Public API

- Extract `(fn_name, line_count)` for top-level `fn` definitions by tracking

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
