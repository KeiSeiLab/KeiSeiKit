---
title: verify_helpers.rs
path: kei-auth-google/src/verify_helpers.rs
dna_hash: sha256:c19bcf5e5def2da9
language: rust
size_loc: 114
generated: by-keidocs
---

# kei-auth-google/src/verify_helpers.rs

Pure helpers extracted from [`crate::provider`]. Each one is a
single-responsibility check used inside `verify()` — split out so
the provider file stays under the 200-LOC Constructor Pattern bound
and so the security-critical predicates are unit-testable in
isolation (no HTTP, no async).

## Public API

- Pull `(code, state, expected_state, code_verifier)` out of an
- Constant-time CSRF-state compare. Returns
- Reject userinfo where `email_verified` is absent / false.
- If `token.id_token` is `Some`, decode its claims and require

## Related

- parent: `kei-auth-google/Cargo.toml`
- imports: crate, kei_runtime_core, subtle

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
