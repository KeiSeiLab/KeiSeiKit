---
title: tool_apply_atomic.rs
path: kei-cortex/src/handlers/tool_apply_atomic.rs
dna_hash: sha256:b8aa73be9c76c5db
language: rust
size_loc: 173
generated: by-keidocs
---

# kei-cortex/src/handlers/tool_apply_atomic.rs

Symlink-safe atomic write — closes the TOCTOU + symlink-escape window
that the simpler `tool::write::atomic_write` leaves open for the trusted
`/tool/apply` endpoint (F-CRIT-4).

Algorithm:
1. `openat(parent_fd, "<basename>.<ts>.tmp", O_CREAT|O_EXCL|O_WRONLY|
O_NOFOLLOW)` — creates a fresh temp inode in the parent dir, refusing
to follow a symlink even at the leaf.
2. `write_all` + `fsync`.
3. `renameat(parent_fd, tmp, parent_fd, basename)` — atomic same-dir
rename; never follows a symlink at the destination because
`renameat` operates on the directory entry, not on a target.
4. After rename, re-canonicalise the destination AND verify it still
lives under `project_root_canon`. Any mismatch (a parent dir was
symlink-swapped between resolve and write) → unlink the just-created
file and return `Forbidden`.

Residual race window: an attacker who can write to the parent
directory's parent (and thus swap a parent dir for a symlink, then swap
it back between rename and canonicalize) can in principle still escape.
This is acknowledged in `tool_apply_INTEGRATION.md` — the endpoint is
TRUSTED by bearer-auth and the on-disk write is sequenced behind the
parent-fd, dramatically narrowing the window vs the prior `tokio::fs::
write` + path-only check. A future Wave can close it with `openat2`
`RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS` (Linux ≥5.6).

Local to `tool_apply.rs` for now per Wave 44b INTEGRATION note —
orchestrator may relocate to `tool/atomic_io.rs::atomic_write_nofollow`
after the wave44a `atomic_write` extraction lands.

## Public API

- Atomic, symlink-refusing write of `content` to `dest`. Caller has already
- All blocking syscalls run inside a single `spawn_blocking` so the runtime
- Open the parent directory; the returned `OwnedFd` keeps it open for the
- Create the staging tempfile with `O_NOFOLLOW | O_EXCL` — refuses any
- Write payload bytes and `fsync(2)` the file before rename.
- `renameat` from staging name to final basename, both relative to
- After rename, canonicalise the destination and confirm it still lives
- Unlink `name` under `dirfd`, swallowing errors — used in cleanup paths
- Resolve `dest`'s parent directory, defaulting to `.` for bare names.
- Convert `dest`'s file name to a `CString` (refusing names with NUL).
- Build a unique temp basename `<dest>.<nanos>.tmp`.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, nix, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
