---
title: lib.rs
path: kei-store/src/lib.rs
dna_hash: sha256:d0d019f7c2afd642
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-store/src/lib.rs

kei-store — memory-repo backend abstraction.

Trait `MemoryStore` + 5 implementations:
- `GitHubStore`, `ForgejoStore`, `GiteaStore` — git-over-SSH/HTTPS
- `FilesystemStore` — local `.git` only; never pushes
- `S3Store` — object-storage with manifest.json (MVP local stub)
- `S3CloudStore` — real S3 / R2 / MinIO via `aws-sdk-s3`
(behind `s3` feature; v0.21+)

Config loaded from `~/.claude/agents/_primitives/store-config.toml`
by default; overridable via `--config`.

RULE 0.8 — this crate reads secret references from env vars only
(`KEI_MEMORY_SSH_KEY`, `KEI_MEMORY_PAT`, `AWS_SECRET_ACCESS_KEY`, ...).

## Public API

- Cloud-backend sub-trait + shared tokio runtime + sync-over-async adapter.
- Test hygiene — shared ENV_LOCK for tests that mutate process env.

## Related

- parent: `kei-store/Cargo.toml`

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
