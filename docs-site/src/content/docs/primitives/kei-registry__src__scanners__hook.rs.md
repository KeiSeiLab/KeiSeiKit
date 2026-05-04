---
title: hook.rs
path: kei-registry/src/scanners/hook.rs
dna_hash: sha256:5dae97538999d41b
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-registry/src/scanners/hook.rs

Hook scanner — walks `<hooks-root>/*.sh`.

Constructor Pattern: this cube knows only the flat `~/.claude/hooks/`
directory layout. Body = raw shell script bytes; name = filename stem;
caps = `shell`.

## Public API

- `pub struct HookScanner` — `<hooks-root>/<name>.sh` adapter. Configurable root because hooks live

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
