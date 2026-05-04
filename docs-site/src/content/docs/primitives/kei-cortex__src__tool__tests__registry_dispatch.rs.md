---
title: registry_dispatch.rs
path: kei-cortex/src/tool/tests/registry_dispatch.rs
dna_hash: sha256:37ff4c3bff0bebd9
language: rust
size_loc: 113
generated: by-keidocs
---

# kei-cortex/src/tool/tests/registry_dispatch.rs

Validates the registry's dispatch table:
- default registry has all 8 named tools
- dispatch on unknown name yields `is_error = true` (no panic)
- dispatch on registered tool returns a successful `ToolResult`
- registry survives concurrent dispatches (Send + Sync via Arc)

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

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
