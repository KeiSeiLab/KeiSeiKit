---
title: input.rs
path: kei-refactor-engine/src/input.rs
dna_hash: sha256:41ed01aabaeaf2bb
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-refactor-engine/src/input.rs

Conflict input schema (mirror of kei-conflict-scan output).

Deserialized locally so this crate does not depend on kei-conflict-scan
as a library — the pipe is JSON, both sides speak the same contract.

## Related

- parent: `kei-refactor-engine/Cargo.toml`
- imports: anyhow, serde, std

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
