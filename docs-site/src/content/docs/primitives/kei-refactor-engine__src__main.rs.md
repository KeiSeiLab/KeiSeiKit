---
title: main.rs
path: kei-refactor-engine/src/main.rs
dna_hash: sha256:7ad856c9971148e0
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-refactor-engine/src/main.rs

kei-refactor-engine — binary entry.

Usage:
kei-refactor-engine --input conflicts.json --plan-only > plan.md
kei-refactor-engine --input conflicts.json --apply-to-branch deep-sleep/2026-04-22 \
--plan-out plan.md --patch-out plan-autoresolve.md

NOTE (v0.14.1): `--patch-out` writes a MARKDOWN review file, NOT a
unified diff. The old claim "git apply-ready patch" was retracted —
see `patch.rs` header. The flag name is kept for backwards-compat.

## Public API

- Input JSON file (output of kei-conflict-scan). Use `-` for stdin.
- Plan-only mode (default). Prints markdown to stdout if no --plan-out.
- Apply mode — also write an auto-resolve review file; takes the branch name.
- Optional explicit path for the markdown plan.
- Optional explicit path for the auto-resolve review markdown

## Related

- parent: `kei-refactor-engine/Cargo.toml`
- imports: anyhow, clap, kei_refactor_engine, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
