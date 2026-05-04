---
title: format.rs
path: kei-skills/src/format.rs
dna_hash: sha256:6260a3d42841b58f
language: rust
size_loc: 109
generated: by-keidocs
---

# kei-skills/src/format.rs

SKILL.md parser/serializer.

On-disk shape:
```text
---
name: <slug>
description: <≤1024 chars>
category: <optional>
stability: <optional — experimental | validated>
---

<markdown body>
```

Round-trip rule: `serialize(parse(s)) == s` byte-for-byte for any
Hermes / agentskills.io conformant SKILL.md. Tested in
`tests/format_roundtrip.rs`.

## Public API

- Frontmatter required by Hermes / agentskills.io. Optional fields kept
- Slug-form skill name. Default empty so a missing `name` key in
- Human-readable summary, ≤1024 chars (validator-enforced). Default
- Catch-all for vendor extensions (`metadata`, `argument-hint`, etc.).
- Parsed skill: typed frontmatter + raw markdown body + originating path.
- Locate the closing `---` after the opening one. Returns the offset of
- `pub fn parse` — Parse a SKILL.md string. Errors wrap `anyhow` messages with the
- `pub fn serialize` — Serialize back to canonical SKILL.md form. Layout:

## Related

- parent: `kei-skills/Cargo.toml`
- imports: anyhow, serde, std

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
