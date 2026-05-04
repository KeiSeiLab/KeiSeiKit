---
title: lib.rs
path: kei-leak-matrix/src/lib.rs
dna_hash: sha256:2cc30dd7ff8f03c5
language: rust
size_loc: 15
generated: by-keidocs
---

# kei-leak-matrix/src/lib.rs

kei-leak-matrix — single source of truth for content protection patterns.

See `security/leak-matrix.toml` for the SSoT data file. This crate
parses it once, compiles every regex upfront, and exposes scan +
substitute helpers used by hooks (no-github-push, sync-public.sh,
secrets-guard, genesis-leak-guard) and by ad-hoc CLI use.

## Related

- parent: `kei-leak-matrix/Cargo.toml`

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
