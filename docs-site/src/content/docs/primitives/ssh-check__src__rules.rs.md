---
title: rules.rs
path: ssh-check/src/rules.rs
dna_hash: sha256:5b04ea8b27e746a4
language: rust
size_loc: 128
generated: by-keidocs
---

# ssh-check/src/rules.rs

Hardened SSH baseline — rule matrix. See block
`_blocks/security-ssh-hardening.md` for rationale per directive.

## Public API

- Value must equal (case-insensitive) one of the given strings.
- Value must equal the given string (case-insensitive).
- Value must be a numeric literal ≤ given bound.
- Value must contain ALL of the given tokens (comma-split, case-insensitive).
- Value must NOT contain ANY of the given tokens.
- Value must be present and non-empty; dynamic equality deferred to check.rs.

## Related

- parent: `ssh-check/Cargo.toml`

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
