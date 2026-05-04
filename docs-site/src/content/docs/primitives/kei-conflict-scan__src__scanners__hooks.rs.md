---
title: hooks.rs
path: kei-conflict-scan/src/scanners/hooks.rs
dna_hash: sha256:508d9a70996e81e3
language: rust
size_loc: 67
generated: by-keidocs
---

# kei-conflict-scan/src/scanners/hooks.rs

Hook-overlap detector.

Heuristic: two hook scripts in `hooks/` whose first line-match of
`tool_name|matcher|event|PreToolUse|PostToolUse|UserPromptSubmit`
targets the same value. Flags the pair as possibly-redundant.

## Related

- parent: `kei-conflict-scan/Cargo.toml`
- imports: crate, regex, std

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
