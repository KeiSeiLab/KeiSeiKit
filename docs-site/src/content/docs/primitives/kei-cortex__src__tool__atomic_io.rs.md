---
title: atomic_io.rs
path: kei-cortex/src/tool/atomic_io.rs
dna_hash: sha256:2bfbaed75b4aceb0
language: rust
size_loc: 65
generated: by-keidocs
---

# kei-cortex/src/tool/atomic_io.rs

Atomic file write — tempfile-in-same-dir + rename.

Composition: shared by `write.rs` and `edit.rs` (and mirrored by
`handlers/tool_apply.rs` until that cube refactors to import this).
The same-directory rename is atomic on POSIX and Windows, so partial
writes never appear at the destination path.

Constructor Pattern: one fn, no state, ≤30 LOC active body. The
tempfile name encodes a nanosecond timestamp so concurrent writes to
the same destination collide deterministically (last-rename-wins) and
never overwrite each other's staging files.

## Public API

- Stage `bytes` to `<dir>/<basename>.<nanos>.tmp` then rename onto

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
