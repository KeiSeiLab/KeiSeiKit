---
title: tools_deny_tools.rs
path: kei-agent-runtime/src/gates/tools_deny_tools.rs
dna_hash: sha256:38dce77f0eb869e9
language: rust
size_loc: 27
generated: by-keidocs
---

# kei-agent-runtime/src/gates/tools_deny_tools.rs

`tools::deny-tools` — denies Edit/Write/MultiEdit/NotebookEdit entirely.

Renamed from `tools::read-only` in v0.17. The capability adds a list of
tools to the PreToolUse deny-list; the old name was a metaphor, the new
name describes the mechanism. Old name still resolves via registry alias.

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate

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
