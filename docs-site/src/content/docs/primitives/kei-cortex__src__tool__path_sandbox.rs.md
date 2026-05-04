---
title: path_sandbox.rs
path: kei-cortex/src/tool/path_sandbox.rs
dna_hash: sha256:7e10187e820b3ed4
language: rust
size_loc: 197
generated: by-keidocs
---

# kei-cortex/src/tool/path_sandbox.rs

Path-sandbox primitives — chroot enforcement + basename deny-lists.

Composition: pure functions, no I/O beyond `canonicalize`. Used by
`read.rs`, `write.rs`, `edit.rs` to (a) keep all access inside the
configured `project_root`, and (b) reject sensitive basenames even
when they happen to live inside the project tree (a symlinked
`.env`, a co-located `id_rsa`, etc.).

Defense-in-depth layers:
1. `enforce_project_root(path, root)` — canonicalised prefix check
2. `is_blocked_basename(path)` — file-type matches (env / keys / pem)
3. `is_blocked_home_rc(path)` — shell rc / dotfile ownership at $HOME

Constructor Pattern: zero state, ≤200 LOC, lives next to the tools
that import it (sibling cube, no dyn dispatch).

## Public API

- `pub fn enforce_project_root` — Canonicalised prefix-check: requested path must resolve INSIDE
- `canonicalize()` fails when the file does not exist yet (write tool).
- `pub fn is_blocked_basename` — Reject sensitive basenames regardless of containing dir. Catches
- `pub fn is_blocked_home_rc` — Block writes / reads of dotfile shell-rc and ssh / aws credential
- Relative paths under `$HOME` blocked even by basename match. Does
- `pub fn check_all` — Compose the three checks; returns the first denial it finds.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
