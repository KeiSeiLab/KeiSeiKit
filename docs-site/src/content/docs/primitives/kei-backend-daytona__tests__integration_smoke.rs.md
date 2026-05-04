---
title: integration_smoke.rs
path: kei-backend-daytona/tests/integration_smoke.rs
dna_hash: sha256:2c792fbc3a97aa26
language: rust
size_loc: 89
generated: by-keidocs
---

# kei-backend-daytona/tests/integration_smoke.rs

End-to-end smoke test against the real Daytona service.

Skipped by default. To run:

```bash
export DAYTONA_API_KEY=...
export DAYTONA_BASE_URL=https://app.daytona.io/api
cargo test -p kei-backend-daytona --test integration_smoke -- --ignored --nocapture
```

The test acquires a sandbox keyed by a fixed task id, runs `echo hi`,
then **stops** (does NOT delete) so that the next run exercises the
resume-from-hibernated branch.

## Related

- parent: `kei-backend-daytona/tests`
- imports: kei_backend_daytona

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
