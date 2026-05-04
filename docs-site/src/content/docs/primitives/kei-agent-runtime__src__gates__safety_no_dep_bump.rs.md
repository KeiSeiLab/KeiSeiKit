---
title: safety_no_dep_bump.rs
path: kei-agent-runtime/src/gates/safety_no_dep_bump.rs
dna_hash: sha256:00b6e72da68f1371
language: rust
size_loc: 19
generated: by-keidocs
---

# kei-agent-runtime/src/gates/safety_no_dep_bump.rs

`safety::no-dep-bump` gate — PreToolUse:Edit|Write denies edits to
Cargo.toml / Cargo.lock unless `ALLOW_DEP_BUMP=1` is in the env.

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
