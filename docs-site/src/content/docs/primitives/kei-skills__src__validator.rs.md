---
title: validator.rs
path: kei-skills/src/validator.rs
dna_hash: sha256:f88bb5696d80b0f1
language: rust
size_loc: 126
generated: by-keidocs
---

# kei-skills/src/validator.rs

SKILL.md validation — port of Hermes
`tools/skill_manager_tool.py::_validate_frontmatter` + size caps.

Returns a `Vec<ValidationIssue>` so a single skill can fail multiple
checks in one pass. Empty Vec = valid.

## Public API

- `pub const MAX_SKILL_CONTENT_CHARS` — Hermes parity: ~36k tokens at 2.75 chars/token.
- `pub const MAX_SKILL_FILE_BYTES` — Hermes parity: 1 MiB ceiling per supporting file (and SKILL.md itself).
- `pub const MAX_DESCRIPTION_LENGTH` — Hermes parity: ≤1024 chars on `description`.
- `pub const MAX_NAME_LENGTH` — Hermes parity: ≤64 chars on `name`.
- Slug regex — lowercase letters/digits, then `[a-z0-9._-]*`.
- One validation finding. Multiple may stack on one skill (e.g. body
- Discriminator on `ValidationIssue`. Stable across versions — callers
- `pub fn validate` — Validate raw SKILL.md content. Path is informational (used in

## Related

- parent: `kei-skills/Cargo.toml`
- imports: crate, regex, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
