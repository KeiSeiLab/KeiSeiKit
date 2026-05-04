---
title: main.rs
path: keisei/src/main.rs
dna_hash: sha256:cd04d65dd9f68e9d
language: rust
size_loc: 105
generated: by-keidocs
---

# keisei/src/main.rs

keisei — exobrain attach/status CLI (v0.22 multi-brain + Auto scope).

Constructor Pattern: main.rs = clap parse + dispatch only. All
subcommand logic lives in sibling modules
(`attach.rs`, `status.rs`, `mount.rs`, `detach.rs`, `list.rs`).

## Public API

- Host-wide config (`~/.claude/...`, `~/.cursor/...`).
- Project-local config (`./.claude/...`, `./.cursor/...`).
- Let the adapter pick based on CWD (v0.22 default). `./.claude/`
- Attach a brain to the single currently detected AI client.
- Path to the brain directory (must contain manifest.toml).
- Which client config to write — host-wide (`user`), project-local
- Attach a brain to EVERY detected AI client in one shot.
- Path to the brain directory (must contain manifest.toml).
- Remove the brain from every client recorded in the marker.
- Show the currently attached brain + health checks.
- List every registered adapter + whether it's detected here.

## Related

- parent: `keisei/Cargo.toml`
- imports: clap, crate, std

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
