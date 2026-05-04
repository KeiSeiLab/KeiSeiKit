---
title: headers.rs
path: kei-forge/src/headers.rs
dna_hash: sha256:8dac7a683e85c73e
language: rust
size_loc: 62
generated: by-keidocs
---

# kei-forge/src/headers.rs

Security headers applied to the GET / HTML response.

Defence-in-depth layer on top of the Host allow-list and JSON
content-type enforcement: these directives limit the blast radius of
any reflected-XSS / iframe-embedding attempt against the wizard UI.

- `Content-Security-Policy` — inline-script/style only from self, no
external origins, `form-action 'self'` blocks cross-origin form
posts even if the SOP layer is bypassed.
- `X-Content-Type-Options: nosniff` — browsers MUST NOT sniff MIME.
- `X-Frame-Options: DENY` — cannot be iframe-embedded (clickjacking).
- `Referrer-Policy: no-referrer` — don't leak the wizard URL.

## Public API

- `pub fn apply_security_headers` — Populate `headers` with the four security headers. Used by the GET /

## Related

- parent: `kei-forge/Cargo.toml`
- imports: axum

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
