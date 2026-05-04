---
title: rules.rs
path: kei-conflict-scan/src/scanners/rules.rs
dna_hash: sha256:eecb0ac7c2a3cc59
language: rust
size_loc: 76
generated: by-keidocs
---

# kei-conflict-scan/src/scanners/rules.rs

Rule-file conflict detector.

Heuristic: look for contradictory directive pairs like
"never X" vs "prefer X" or "forbidden: X" vs "required: X" across
`rules/*.md`. Tokens compared after stripping filler words.

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
