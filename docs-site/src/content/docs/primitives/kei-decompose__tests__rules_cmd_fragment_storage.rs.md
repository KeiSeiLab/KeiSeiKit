---
title: rules_cmd_fragment_storage.rs
path: kei-decompose/tests/rules_cmd_fragment_storage.rs
dna_hash: sha256:20e60665551a635d
language: rust
size_loc: 211
generated: by-keidocs
---

# kei-decompose/tests/rules_cmd_fragment_storage.rs

Integration test: decompose-rules writes real fragment files and registers
real paths in the SQLite registry. Validates Path 1 fix for Wave 14d.

Verify criterion: after `rules_cmd::run(...)`, every active `rule` row in
the registry has a `path` that exists on disk and whose content matches
the registered fragment body.

## Related

- parent: `kei-decompose/tests`
- imports: kei_decompose, std, tempfile

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
