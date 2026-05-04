---
title: types.rs
path: kei-cortex/src/context/types.rs
dna_hash: sha256:bb23e03a9bcd2006
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-cortex/src/context/types.rs

Public types shared across the `context` submodules.

Constructor Pattern: types-only file, no behaviour. Behaviour lives in
`discover.rs`, `skill_loader.rs`, `inject.rs`. Anything that holds state
crossing module boundaries is declared here so call-sites stay flat.

## Public API

- Which class of context file this is. Drives both rendering order in
- `CLAUDE.md` — project / repo / parent context for Claude Code.
- `AGENTS.md` — cross-tool agent context (cursor, codex, generic).
- `SOUL.md` — KeiSei-specific persona overlay at project level.
- Any other `.md` discovered alongside (reserved for future use).
- One context file discovered during walk-up. `path` is absolute,
- A `/skill-name`-matched skill body, with its source path for

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
