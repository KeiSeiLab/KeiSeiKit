---
title: openclaw.rs
path: kei-skill-importer/src/parsers/openclaw.rs
dna_hash: sha256:7a29d28bbd6b8c93
language: rust
size_loc: 128
generated: by-keidocs
---

# kei-skill-importer/src/parsers/openclaw.rs

OpenClaw format parser.

Format research (verified 2026-04-25 via WebFetch
`https://raw.githubusercontent.com/openclaw/openclaw/main/AGENTS.md`):

- File: `AGENTS.md` (root) OR `~/.openclaw/workspace/skills/<name>/SKILL.md`.
- Frontmatter: optional YAML (often absent for AGENTS.md). When
present, common keys: `name`, `description`, `tags`, `tools`.
- Body: H2-rooted sections (`## Start`, `## Map`, `## Architecture`,
`## Commands`, `## Gates`, `## Code`, …) — each H2 is a "phase"
in our canonical model. "Telegraph style" — terse bullet lists.
- Tools detection: `## Commands` section frequently lists `pnpm <verb>`
bash invocations as bullets; we treat those as candidate atom calls.

## Public API

- Split body into phases by H2 headings (`## Foo`). When body has zero

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: anyhow, crate, serde, serde_yaml_ng, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
