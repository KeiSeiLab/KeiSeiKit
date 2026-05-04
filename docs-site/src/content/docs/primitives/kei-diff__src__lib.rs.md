---
title: lib.rs
path: kei-diff/src/lib.rs
dna_hash: sha256:fb838001f73f1507
language: rust
size_loc: 22
generated: by-keidocs
---

# kei-diff/src/lib.rs

kei-diff — structural JSON diff (RFC 6902 subset: add/remove/replace).

## Design
* Emits ONLY `add`, `remove`, `replace`. No `copy`/`move`/`test`.
* Arrays diffed by index (not LCS) — matches drift-detection semantics.
* Paths are RFC 6901 JSON Pointers (`~` → `~0`, `/` → `~1`).
* Correctness invariant: `apply(old, diff(old, new)) == new`.

Consumed by `kei-replay` (drift detection between DNA-scoped agent runs)
and `kei-cache` (invalidation signals). Pure compute, zero sibling deps.

## Related

- parent: `kei-diff/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
