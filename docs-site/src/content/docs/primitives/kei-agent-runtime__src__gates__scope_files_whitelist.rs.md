---
title: scope_files_whitelist.rs
path: kei-agent-runtime/src/gates/scope_files_whitelist.rs
dna_hash: sha256:b7a437c1333b71d0
language: rust
size_loc: 16
generated: by-keidocs
---

# kei-agent-runtime/src/gates/scope_files_whitelist.rs

`scope::files-whitelist` — PreToolUse:Edit|Write denies paths outside
`task.scope.files-whitelist` globs. Empty list = not applicable (allow).

As of v0.18 convergence wave: thin const wrapper over `PatternGate`.

## Related

- parent: `kei-agent-runtime/Cargo.toml`

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
