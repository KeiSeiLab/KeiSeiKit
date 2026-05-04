---
title: paths.rs
path: keisei/src/paths.rs
dna_hash: sha256:001bc513ca5faa31
language: rust
size_loc: 34
generated: by-keidocs
---

# keisei/src/paths.rs

Host path resolution — SSoT for `$KEISEI_HOME` / `$HOME` fallback.

Constructor Pattern: single responsibility — resolve the user's home
directory for every adapter + the keisei state dir. `$KEISEI_HOME`
overrides `$HOME` so integration tests can isolate state per tmpdir.
Adapters compose on top of this SSoT; no duplication of the env-var
chain across the codebase.

## Public API

- `pub fn resolve_home` — Resolve the user's home directory.
- `pub fn keisei_state_dir` — Keisei's own state directory (marker file + future per-tool state).

## Related

- parent: `keisei/Cargo.toml`
- imports: std

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
