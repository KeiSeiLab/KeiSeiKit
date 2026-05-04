---
title: read.rs
path: kei-cortex/src/tool/read.rs
dna_hash: sha256:27a35d400a7542ef
language: rust
size_loc: 151
generated: by-keidocs
---

# kei-cortex/src/tool/read.rs

`read` tool — file retrieval with line numbers.

Composition: validate input → reject relative / `..` / outside
`project_root` / blocked-basename → read file → render `cat -n`-style
output. No new I/O logic — standard `tokio::fs` plus `path_sandbox`.

Sandbox guarantees:
- rejects relative paths and any path containing `..`
- rejects paths that resolve OUTSIDE `project_root` (canonicalised)
- rejects sensitive basenames (`.env`, `id_rsa*`, `*.pem`, …)
- rejects `~/.zshrc`-class dotfiles even via project-root symlink
- rejects non-utf8 file contents (binary returns an error message)
- rejects files larger than `MAX_BYTES` (10 MiB)

## Public API

- Hard cap on file size returned to the model.
- Default line limit when caller does not specify one.
- Lexical-only path checks (cheap pre-filter before canonicalisation).
- Render lines `cat -n`-style, honouring offset/limit windowing.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: serde, serde_json, std, tempfile

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
