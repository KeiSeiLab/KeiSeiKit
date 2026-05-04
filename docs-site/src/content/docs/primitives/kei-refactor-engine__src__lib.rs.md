---
title: lib.rs
path: kei-refactor-engine/src/lib.rs
dna_hash: sha256:d6b79386f2010cf5
language: rust
size_loc: 17
generated: by-keidocs
---

# kei-refactor-engine/src/lib.rs

kei-refactor-engine — library surface.

Consumes `kei-conflict-scan` JSON; produces a structured refactor plan
(markdown) and, optionally, an auto-resolve review markdown
(NOT a unified diff — see patch.rs header, v0.14.1 retraction).

Zero-conflict guarantee: any conflict whose `auto_resolvable = false`
is included in the plan under "Requires human decision" and EXCLUDED
from the auto-resolve markdown.

## Related

- parent: `kei-refactor-engine/Cargo.toml`

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
