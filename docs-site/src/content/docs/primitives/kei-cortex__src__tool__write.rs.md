---
title: write.rs
path: kei-cortex/src/tool/write.rs
dna_hash: sha256:956443431fd0046b
language: rust
size_loc: 139
generated: by-keidocs
---

# kei-cortex/src/tool/write.rs

`write` tool — atomic file write.

Composition: validate path → enforce `project_root` chroot + basename
deny → ensure parent dir → atomic_write to staging tempfile + rename.
Same-directory rename is atomic on POSIX and Windows so partial writes
never appear.

Sandbox: lexical pre-check (`validate_path_lexical`) +
`path_sandbox::check_all` (chroot + basename + home-rc). The legacy
`deny_system_dirs` substring check stays as a Layer-3 belt-and-
suspenders for the system-dir corner cases.

## Public API

- Reject writes to root-level system directories. Belt-and-suspenders

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: serde, serde_json, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
