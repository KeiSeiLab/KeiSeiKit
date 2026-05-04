---
title: discover_walks_up.rs
path: kei-cortex/src/context/tests/discover_walks_up.rs
dna_hash: sha256:7e98e562293b2abf
language: rust
size_loc: 69
generated: by-keidocs
---

# kei-cortex/src/context/tests/discover_walks_up.rs

Discover walks from `cwd` upward, returning every CLAUDE.md / AGENTS.md
it finds in nearest-first order.

Setup: tempdir -> a/b/c/d/e (5 levels). Plant CLAUDE.md at e (innermost),
at c (mid), at a (outermost). Run discover from e. Expect three hits in
the order [e, c, a].

Symlink-safety: also plant a symlink CLAUDE.md and assert it's skipped.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
