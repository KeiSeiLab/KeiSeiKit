---
title: persistence_paths.rs
path: kei-frustration-loop/tests/persistence_paths.rs
dna_hash: sha256:6e4a295114267b89
language: rust
size_loc: 67
generated: by-keidocs
---

# kei-frustration-loop/tests/persistence_paths.rs

Path-layout test for the per-user persistence cube. Asserts that every
file resolver returns the exact filename pattern the spec promised:
`<home>/.claude/frustration/<user>.firmware.gz`
`<home>/.claude/frustration/<user>.last-scan.ts`
`<home>/.claude/frustration/<user>.feedback.jsonl`
`<home>/.claude/frustration/queue.jsonl`

## Related

- parent: `kei-frustration-loop/tests`
- imports: kei_frustration_loop, std, tempfile

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
