---
title: phase-4-test-push
path: sleep-setup/phase-4-test-push.md
dna_hash: sha256:fd2f263186d73f55
language: markdown
size_loc: 89
generated: by-keidocs
---

# sleep-setup/phase-4-test-push.md

## Public API

- `Phase 4 — Test push (verify write access)` — Write a tiny marker file, call `kei-sleep-sync.sh`, let the user confirm
- `4a — Write a test marker` — ```bash
- `4b — Invoke the sync helper` — ```bash
- `4c — Show expected commit to user` — Read `HEAD`'s commit message from the local mirror:
- `4d — Confirm click` — Emit ONE `AskUserQuestion`:
- `4e — Diagnostic block (when user says "not showing up")` — Render constructively per RULE -1:
- `4f — Cleanup marker` — Regardless of branch:
- `Verify-criterion` — - Exactly ONE `AskUserQuestion` (plus loops on the "No" branch).

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
