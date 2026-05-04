---
title: google_security_regression.rs
path: kei-auth-google/tests/google_security_regression.rs
dna_hash: sha256:f662ec72c778beab
language: rust
size_loc: 164
generated: by-keidocs
---

# kei-auth-google/tests/google_security_regression.rs

CVE-2023-7028 class regression tests for `GoogleAuthProvider`.

Booking.com / Slack / GitLab were all hit by the same pattern: an
OIDC relying-party trusted `userinfo.email` without checking
`email_verified`, allowing a Workspace admin to mint accounts with
arbitrary unverified email aliases and sign in as any user.

These tests ensure `verify()`:
1. refuses `email_verified == false`
2. refuses absent `email_verified`
3. uses `sub` (not `email`) as `user_id`
4. cross-checks `id_token.sub == userinfo.sub` when an `id_token`
is returned, and rejects mismatch
5. accepts the happy path when both are equal

## Related

- parent: `kei-auth-google/tests`
- imports: base64, kei_auth_google, kei_runtime_core, serde_json, wiremock

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
