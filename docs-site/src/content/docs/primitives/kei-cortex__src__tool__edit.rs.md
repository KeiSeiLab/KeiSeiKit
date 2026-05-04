---
title: edit.rs
path: kei-cortex/src/tool/edit.rs
dna_hash: sha256:1292eaec76df0072
language: rust
size_loc: 162
generated: by-keidocs
---

# kei-cortex/src/tool/edit.rs

`edit` tool — string replacement with uniqueness check + atomic write.

Composition: read existing file → verify `old_string` occurrence count
→ replace → atomic_write back via shared `atomic_io::atomic_write`. No
new I/O logic — composes `read` and `write` cube primitives.

Semantics: when `replace_all = false` (default), `old_string` MUST
match exactly once. When `replace_all = true`, every occurrence is
replaced and the count is reported in the success message.

Sandbox: same lexical + chroot + basename rules as `write.rs`.

## Public API

- Count non-overlapping occurrences of `needle` in `hay`.
- Apply the replacement, enforcing the unique-match rule when needed.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: serde, serde_json, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
