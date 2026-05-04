---
title: lib.rs
path: kei-git-bitbucket/src/lib.rs
dna_hash: sha256:a5e3f938b1532e5b
language: rust
size_loc: 21
generated: by-keidocs
---

# kei-git-bitbucket/src/lib.rs

kei-git-bitbucket — Bitbucket Cloud impl of [`kei_runtime_core::GitBackend`].

Layout:
- [`error`]: local `Error`/`Result` mapping into the runtime-core error.
- [`client`]: thin async REST 2.0 wrapper (mockable base URL).
- [`backend`]: [`BitbucketBackend`] — DNA-bearing trait impl.

Auth: HTTP Basic with `BITBUCKET_USERNAME` + `BITBUCKET_APP_PASSWORD`.
Base URL defaults to `https://api.bitbucket.org/2.0` and is overridable
for `wiremock` tests via `BITBUCKET_URL`.

## Related

- parent: `kei-git-bitbucket/Cargo.toml`

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
