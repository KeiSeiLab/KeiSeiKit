---
title: backend.rs
path: kei-git-forgejo/src/backend.rs
dna_hash: sha256:88a579f3d0bde2cb
language: rust
size_loc: 185
generated: by-keidocs
---

# kei-git-forgejo/src/backend.rs

[`ForgejoBackend`] — `GitBackend` impl for public Forgejo / Codeberg.

Network IO splits two ways:
- Repo lifecycle (exists / create / branch SHA) → REST via `ForgejoClient`.
- Working-copy ops (clone / push / mirror) → shell-out to `git`.

`provider_name = "forgejo"`. `supports_auto_create = true` because the
`/api/v1/user/repos` POST will create on demand inside `ensure_repo`.

## Public API

- `pub fn new` — Build a backend from a pre-configured client. The DNA is a fresh
- `pub fn client` — Borrow the underlying client (for callers that need direct REST
- Embed PAT into a clone/push URL. `https://host/o/r.git` →
- Extract `(owner, repo)` from a Forgejo-style HTTPS URL. Accepts

## Related

- parent: `kei-git-forgejo/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, std, tokio

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
