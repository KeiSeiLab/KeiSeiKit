---
title: phase-2-repo-url
path: sleep-setup/phase-2-repo-url.md
dna_hash: sha256:f078bce303d0a7e2
language: markdown
size_loc: 62
generated: by-keidocs
---

# sleep-setup/phase-2-repo-url.md

## Public API

- `Phase 2 — Collect SSH repo URL` — The one and only free-text field in the wizard. Everything else is a
- `2a — Free-text prompt` — Emit ONE `AskUserQuestion` with a `freeText` field:
- `2b — Validate` — Regex: `^git@[A-Za-z0-9._-]+:[A-Za-z0-9._/-]+\.git$`
- `2c — Cross-check against Phase 1` — Extract the host from the URL:
- `Verify-criterion` — - `REPO_URL` matches the validation regex.

## Related

- parent: `sleep-setup`

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
