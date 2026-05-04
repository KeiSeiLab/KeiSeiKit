---
title: cursor.rs
path: kei-skill-importer/src/parsers/cursor.rs
dna_hash: sha256:3735ef39ca023c19
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-skill-importer/src/parsers/cursor.rs

Cursor `.mdc` parser.

Format research (verified 2026-04-25 via Cursor docs page extraction):

- Files: `<repo>/.cursor/rules/<name>.mdc` (modern) OR
`<repo>/.cursorrules` (legacy, plain text). We support `.mdc` here.
- Frontmatter: REQUIRED YAML. Common keys:
description: "Short description shown in rule selector"
globs: ["**/*.tsx", "src/**/*.ts"]   # file glob scope
alwaysApply: false                    # auto-include in every prompt
- Body: free-form markdown rule content.
- No phases convention; rules are flat. We synthesize one phase.

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: anyhow, crate, serde, serde_yaml_ng, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
