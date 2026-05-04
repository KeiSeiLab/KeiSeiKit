---
title: SKILL
path: dev-ship/SKILL.md
dna_hash: sha256:34eb1b771a84034c
language: markdown
size_loc: 291
generated: by-keidocs
---

# dev-ship/SKILL.md

## Public API

- `/dev-ship — Pre-Merge Quality Gate` — ---
- `When to use` — - Running a final quality gate before merging a feature branch into main.
- `Команды` — - `/dev-ship` → полная pre-merge проверка (4 агента)
- `Phase 0 — Branch Summary (lead)` — 1. `git log main..HEAD --oneline` → все коммиты в ветке
- `Phase 1 — Final Gate (4 агента параллельно)` — ### Agent: `sh-security-final`
- `Phase 2 — Ship Decision (lead)` — ```
- `Security Final` — [findings or CLEAN]
- `Test Coverage` — [coverage map + test results]
- `Dependencies` — New: N | Dead: N | Duplicates: N
- `Regression` — Breaking: N | Pattern delta: [improved/degraded] | SSOT: [status]
- `Frontend Final Gate (if frontend changes detected)` — Build: PASS/FAIL | Typecheck: PASS/FAIL | DB-contract: PASS/FAIL/N drift
- `Baseline Comparison` — | Metric | Before | After | Delta |
- `Verdict: SHIP / FIX / ABORT` — ```
- `Strict Mode (default)` — Merge блокируется если ПОСЛЕ хуже чем ДО по ЛЮБОМУ из:
- `Safety` — - НЕ мержить без verdict SHIP

## Related

- parent: `dev-ship`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
