---
title: main.rs
path: kei-runtime/src/main.rs
dna_hash: sha256:8b4035288e57631b
language: rust
size_loc: 165
generated: by-keidocs
---

# kei-runtime/src/main.rs

kei-runtime — CLI dispatcher.

Subcommands: list-atoms | invoke | schema-lint | pipe (stub).
Default --root: `~/.claude/agents/_primitives/_rust`.

## Public API

- Exit code returned when `invoke` lands on a not-yet-wired atom.
- List atoms discovered under --root.
- Invoke one atom (MVP stub — see docs).
- Lint every `atoms/*.md` under --root for schema correctness.
- Execute a pipeline (not yet implemented).
- Map typed invoke errors to exit codes per locked §Runtime schema.

## Related

- parent: `kei-runtime/Cargo.toml`
- imports: clap, kei_runtime, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
