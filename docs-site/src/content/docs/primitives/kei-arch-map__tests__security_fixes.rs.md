---
title: security_fixes.rs
path: kei-arch-map/tests/security_fixes.rs
dna_hash: sha256:978b084f60783916
language: rust
size_loc: 102
generated: by-keidocs
---

# kei-arch-map/tests/security_fixes.rs

Regression tests for Wave 4 security fixes (Task #50 A–F).

Mirror of `tests/evidence.rs` style — uses the same `support` module
to keep call-sites short. Each #[test] points at the specific fix it
protects so a future regression is traceable to its origin task.

## Public API

- Fix A: hostnames must be DNS-resolved and EVERY resolved address checked
- Fix C: `cargo metadata` (no build.rs run) replaces `cargo check`. Empty
- Fix D: `..` traversal must not let a claim reach above repo root.
- Fix E: actual JSON value must NOT appear verbatim in the FAIL reason.

## Related

- parent: `kei-arch-map/tests`
- imports: std, support, tempfile

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
