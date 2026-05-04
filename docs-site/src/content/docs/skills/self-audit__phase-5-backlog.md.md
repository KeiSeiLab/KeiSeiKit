---
title: phase-5-backlog
path: self-audit/phase-5-backlog.md
dna_hash: sha256:a2597113291ab701
language: markdown
size_loc: 55
generated: by-keidocs
---

# self-audit/phase-5-backlog.md

## Public API

- `Phase 5 — Backlog` — Update `~/.claude/memory/audit-backlog.md`: increment the session counter,
- `5a — Increment session counter` — Read the `<!-- session_count: N -->` header. Rewrite the same line with
- `5b — Append per-finding notes` — For each finding, append one line based on its `ROUTES` entry:
- `5c — Clear processed items click` — Emit ONE `AskUserQuestion`:
- `5d — Emit final report` — Print the final report (format from `SKILL.md`).
- `Verify-criterion` — - `<!-- session_count: N -->` header incremented by exactly 1.

## Related

- parent: `self-audit`

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
