---
title: schemas.rs
path: kei-cortex/src/tool/schemas.rs
dna_hash: sha256:5aed7deef93c5068
language: rust
size_loc: 183
generated: by-keidocs
---

# kei-cortex/src/tool/schemas.rs

Anthropic tool-use schema definitions.

`tool_definitions()` returns the JSON array the daemon sends to
Anthropic in the `tools` field of `messages.create`. Each entry is a
`{name, description, input_schema}` object. The `input_schema` is a
JSON-Schema describing what the model must emit in `tool_use.input`.

Constructor Pattern: schema-only module, no executor logic. Each tool
is one small builder fn (≤30 LOC) so additions stay surgical.

## Public API

- `pub fn tool_definitions` — All 8 tool definitions. Order matches the registry default.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: serde_json

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
