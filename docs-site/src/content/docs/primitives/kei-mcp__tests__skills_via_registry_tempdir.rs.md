---
title: skills_via_registry_tempdir.rs
path: kei-mcp/tests/skills_via_registry_tempdir.rs
dna_hash: sha256:86050bda1028bfa8
language: rust
size_loc: 133
generated: by-keidocs
---

# kei-mcp/tests/skills_via_registry_tempdir.rs

Regression test (Hermes P3.1.b) — `resources/list` and `resources/read`
flow through `kei_skills::SkillRegistry`, proven on a tempdir corpus
that does NOT depend on the real `skills/` tree.

Complements `skills_via_registry.rs` (which uses the repo corpus and
self-skips when absent). This test always runs because it builds the
corpus inline. Three SKILL.md files exercise the SSoT contract:
1. `alpha` with a real description
2. `beta` with a real description
3. `_archive/zeta` (must be filtered by `loader::is_archived`)

Asserts:
- Exactly 2 resources surface (alpha + beta), archived skipped by
`kei_skills::loader::is_archived`.
- `description` is taken verbatim from frontmatter.
- `resources/read` returns canonical SKILL.md (round-trip via
`kei_skills::format::serialize`).
- Unknown skill returns `INVALID_PARAMS` error envelope.

## Related

- parent: `kei-mcp/tests`
- imports: kei_mcp, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
