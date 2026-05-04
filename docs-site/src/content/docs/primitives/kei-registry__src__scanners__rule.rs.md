---
title: rule.rs
path: kei-registry/src/scanners/rule.rs
dna_hash: sha256:42e3fe4b269aabc7
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-registry/src/scanners/rule.rs

Rule scanner — walks `<rules-root>/*.md`.

Constructor Pattern: this cube knows only the flat `~/.claude/rules/`
directory layout. Body = raw markdown; name = filename stem; caps = `md`.

## Public API

- `pub struct RuleScanner` — `<rules-root>/<name>.md` adapter. The rules root is configurable via

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, std

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
