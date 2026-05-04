---
title: lib.rs
path: kei-skills/src/lib.rs
dna_hash: sha256:5b612c1e7a995589
language: rust
size_loc: 24
generated: by-keidocs
---

# kei-skills/src/lib.rs

kei-skills — Hermes / agentskills.io SKILL.md format primitives.

Constructor Pattern: one cube per concern.
- `format`     — parse/serialize SKILL.md (YAML frontmatter + markdown body)
- `validator`  — port of Hermes `_validate_frontmatter` + size caps
- `patcher`    — fuzzy find-replace via `similar` crate (atomic write)
- `loader`     — walk a directory and load every valid SKILL.md
- `registry`   — name-keyed in-memory store with optional hot-reload

Bidirectional Hermes interop: same on-disk format, same `extra_taps`
distribution. Reading a Hermes skill round-trips byte-equal through
`format::parse → format::serialize`.

## Related

- parent: `kei-skills/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
