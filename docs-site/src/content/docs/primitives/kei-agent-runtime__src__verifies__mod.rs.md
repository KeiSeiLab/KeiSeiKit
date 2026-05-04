---
title: mod.rs
path: kei-agent-runtime/src/verifies/mod.rs
dna_hash: sha256:0244caac75e2255d
language: rust
size_loc: 18
generated: by-keidocs
---

# kei-agent-runtime/src/verifies/mod.rs

On-return verify capabilities.

After v0.18 convergence wave: 3 command-driven verifies
(`quality::cargo-check-green`, `quality::tests-green`,
`safety::no-dep-bump`) are `CommandVerify` const wrappers. The LOC
walker (`quality::constructor-pattern`), the two report-parser
verifies (`output::*`), and the two git-diff scope verifies stay in
their own modules — shape too divergent to fold into `CommandVerify`.

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
