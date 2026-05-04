---
title: claude.rs
path: kei-skill-importer/src/parsers/claude.rs
dna_hash: sha256:ef6285728b97d2f8
language: rust
size_loc: 135
generated: by-keidocs
---

# kei-skill-importer/src/parsers/claude.rs

Claude Code `SKILL.md` parser (KeiSeiKit native).

Format reference (verified against `skills/pet-init/SKILL.md` and
`skills/onboard/SKILL.md` in this repo):

- File: `skills/<name>/SKILL.md` (the index) plus optional
`phase-<n>-*.md` siblings.
- Frontmatter: YAML with `name`, `description`, optional
`argument-hint`, `category`.
- Body: H2 sections; the canonical wizard pattern uses an explicit
"Pipeline overview" table that lists `phase-*.md` references —
we DO NOT recurse into siblings here (parser is single-file);
instead we map H2 sections to phases, same as OpenClaw.
- Tools detection: `## References` and `## Rules` sections often
list primitives by name; the classifier picks them up.

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
