---
title: resources.rs
path: kei-mcp/src/handlers/resources.rs
dna_hash: sha256:ba3a089ef777d9b6
language: rust
size_loc: 73
generated: by-keidocs
---

# kei-mcp/src/handlers/resources.rs

`resources/list` and `resources/read` — skills as MCP resources via
kei-skills `SkillRegistry` (canonical SSoT).

Each loaded `Skill` becomes one resource:
uri         = `skill://<name>`
name        = `<name>`
mimeType    = `text/markdown`
description = frontmatter.description (truncated to 1024 chars per
agentskills.io spec) or fallback to `Skill: <name>`.

`resources/read` returns the canonical serialized SKILL.md text under
the standard MCP `contents` array.

HERMES-MIGRATION-PLAN P3.1.b — replaces raw walkdir/fs::read_to_string
with `ctx.skills_registry.list()` / `.get(name)` + `kei_skills::format::serialize`.

## Related

- parent: `kei-mcp/Cargo.toml`
- imports: crate, kei_skills, serde_json

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
