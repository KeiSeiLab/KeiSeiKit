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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
