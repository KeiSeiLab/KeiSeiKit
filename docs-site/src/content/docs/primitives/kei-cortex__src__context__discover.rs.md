---
title: discover.rs
path: kei-cortex/src/context/discover.rs
dna_hash: sha256:4a1539c11c7f5970
language: rust
size_loc: 124
generated: by-keidocs
---

# kei-cortex/src/context/discover.rs

Walk up from `cwd` to the user's `$HOME` (or `/`) and collect every
`CLAUDE.md`, `AGENTS.md`, `SOUL.md` encountered at each directory level.

Returned order is **nearest-first**: index 0 is the file in `cwd`
itself, index 1 is in `cwd.parent()`, and so on. The outer caller
(`inject::build_system_prompt`) relies on this ordering to make sure
the most-specific context wins when truncation is required.

Hard stops:
- At `$HOME` (after processing it).
- At `/` (after processing it).
- On the first directory whose name is `node_modules`, `.git`, or
`_archive` (we still process the level above; we just don't
descend into those).

Safety:
- Symlinks are NOT followed (`WalkDir::follow_links(false)`).
- Each file is capped at 1 MiB; oversize content is truncated with a
trailing `\n[truncated]` marker.

## Public API

- Hard read cap per file. Anything larger is truncated.
- Directory names we never descend into when listing.
- `pub fn discover` — Walk up from `cwd`, collecting context files at every level.
- Resolve `$HOME` once. Treated as a stop boundary; we still process the
- True when `dir` is not one of the explicitly-skipped names. We still
- True when we should stop after processing `dir` (reached `$HOME` or `/`).
- Read all known context files at a single directory level.
- Filenames + their classification. Order here determines the within-level
- Read `path` if it exists and is a regular file (not a symlink), capped
- Cut `s` at `MAX_FILE_BYTES` (UTF-8-safe) and append a `[truncated]`

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
