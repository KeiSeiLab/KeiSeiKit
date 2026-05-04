---
title: phase-1-repo-pick
path: sleep-setup/phase-1-repo-pick.md
dna_hash: sha256:c81725a83ebe98c5
language: markdown
size_loc: 71
generated: by-keidocs
---

# sleep-setup/phase-1-repo-pick.md

## Public API

- `Phase 1 — Repo provider + visibility` — Ask the user to pick where the memory-repo lives. Purely click-based —
- `1a — Provider click` — Emit ONE `AskUserQuestion`:
- `1b — Visibility click` — Emit ONE `AskUserQuestion`:
- `1c — Public-visibility warning` — If `VISIBILITY == "Public (I accept the risk)"`, print the warning block
- `Verify-criterion` — - `PROVIDER ∈ {GitHub, GitLab, Bitbucket, Self-hosted}`.

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
