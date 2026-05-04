---
title: SKILL
path: visual-loop/SKILL.md
dna_hash: sha256:5fc9090ffb901e88
language: markdown
size_loc: 242
generated: by-keidocs
---

# visual-loop/SKILL.md

## Public API

- `/visual-loop — Visual / A11y / Responsive Regression Loop` — ---
- `When to use` — - First time on a project → scaffolds Playwright config + spec template
- `Phase 0 — Detect state` — 1. Walk up from `$ARG` (or CWD) to find project root (`package.json`, `pubspec.yaml`).
- `Phase 1 — Scaffold (one AskUserQuestion batch)` — Single click-only confirmation:
- `Phase 2 — First run (establish baseline OR diff)` — ```bash
- `Phase 3 — Triage diff (if any)` — If `npm run visual-check` returned non-zero:
- `Composition with other skills` — - **`/dev-guard`** frontend-validator agent: step 5 — if `package.json` has `visual-check` script, invoke `npm run visual-check`. Severity: WARN on diff.
- `Rules` — - **Click-only.** Free-text only for explicit baseline override scenarios.
- `Final report` — ```
- `Out of scope` — - Lighthouse perf — separate `/perf-audit` skill.
- `References` — - Playwright docs: https://playwright.dev

## Related

- parent: `visual-loop`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
