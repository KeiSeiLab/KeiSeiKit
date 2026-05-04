---
title: main.rs
path: kei-model/src/main.rs
dna_hash: sha256:b1bb369fcd5af890
language: rust
size_loc: 194
generated: by-keidocs
---

# kei-model/src/main.rs

kei-model CLI entry. Dispatches to one handler per subcommand. Each
handler stays ≤30 LOC by delegating to library functions.

Exit codes:
0 — success
1 — file/IO error
2 — not-found / no-match / unknown id
3 — cycle in fallback chain

## Related

- parent: `kei-model/Cargo.toml`
- imports: clap, kei_model, std

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
