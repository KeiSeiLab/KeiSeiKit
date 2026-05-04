---
title: phase-1-role
path: spawn-agent/phase-1-role.md
dna_hash: sha256:5db35cf0d46c53fc
language: markdown
size_loc: 84
generated: by-keidocs
---

# spawn-agent/phase-1-role.md

## Public API

- `Phase 1 — Role selection` — > Goal: pick the agent role (capability tier). This single choice resolves
- `1.a — Single AskUserQuestion` — Send ONE `AskUserQuestion` call. `multiSelect: false`. Do NOT fall through
- `1.b — Guidance (for the user, shown WITH the question)` — Before sending the question, print one short paragraph:
- `1.c — Verify criterion` — `ROLE ∈ {read-only, explorer, edit-local, edit-shared}`. If the user
- `1.d — Failure paths (NO DOWNGRADE)` — If the user wants a role outside the four fixed tiers (e.g. "I want a

## Related

- parent: `spawn-agent`

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
