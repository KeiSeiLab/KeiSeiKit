---
title: tools_list.rs
path: kei-mcp/tests/tools_list.rs
dna_hash: sha256:fadde79ee355d1b2
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-mcp/tests/tools_list.rs

Integration test — `tools/list` walks a mock atom registry and emits
MCP-shape tool descriptors.

Builds a minimal `<root>/<crate>/atoms/<verb>.md` layout in tempdir and
verifies that every atom round-trips into a `{name, description,
inputSchema}` triple.

## Related

- parent: `kei-mcp/tests`
- imports: kei_mcp, serde_json, std

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
