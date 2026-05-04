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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
