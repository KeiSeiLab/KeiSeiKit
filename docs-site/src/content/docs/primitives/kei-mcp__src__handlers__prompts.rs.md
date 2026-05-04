---
title: prompts.rs
path: kei-mcp/src/handlers/prompts.rs
dna_hash: sha256:e5f0042221d73af2
language: rust
size_loc: 27
generated: by-keidocs
---

# kei-mcp/src/handlers/prompts.rs

`prompts/list` and `prompts/get` — placeholder. Returns an empty list.

Future: walk a prompts directory (TBD) and emit MCP prompt descriptors.
For now we honour the spec by returning a well-formed empty `prompts`
array on list, and a `not found` error on get.

## Related

- parent: `kei-mcp/Cargo.toml`
- imports: crate, serde_json

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
