---
title: cli.rs
path: kei-gdrive-import/src/cli.rs
dna_hash: sha256:b59b97b02fb9c913
language: rust
size_loc: 37
generated: by-keidocs
---

# kei-gdrive-import/src/cli.rs

clap derive structs for the two-verb CLI.

## Public API

- Classify a single folder. Emits one JSON object on stdout.
- Folder path. Local FS by default; rclone remote spec with --remote
- Treat path as an rclone remote spec.
- Walk one level under <root>; emit a JSON array of classifications.
- Root folder. With --remote, treated as an rclone remote spec
- Use `rclone lsjson` instead of local FS.

## Related

- parent: `kei-gdrive-import/Cargo.toml`
- imports: clap

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
