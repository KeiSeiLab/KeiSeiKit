---
title: SKILL
path: dev-guard/SKILL.md
dna_hash: sha256:7d21975776b02b94
language: markdown
size_loc: 223
generated: by-keidocs
---

# dev-guard/SKILL.md

## Public API

- `/dev-guard — Continuous Development Guard` — ---
- `When to use` — - After writing a new file or module to catch security, performance, or structural issues before committing.
- `Команды` — - `/dev-guard` → проверить текущие изменения (git diff)
- `Когда запускать` — - После написания нового файла / модуля
- `Phase 0 — Scope Detection (lead)` — 1. `git diff --name-only` → список изменённых файлов
- `Phase 1 — Parallel Guard (3 агента одним сообщением)` — ### Agent: `dg-security`
- `Phase 2 — Quick Synthesis (lead, НЕ агент)` — Lead объединяет результаты за 30 секунд:
- `Adaptive Depth` — > Не гонять 3 агента на каждый CSS-файл.
- `Integration с git workflow` — ### Pre-commit (рекомендуется):
- `В .claude/hooks/ или git hooks` — 
- `Запускать /dev-guard перед каждым commit` — ```
- `Safety` — - НЕ блокировать commit на LOW findings

## Related

- parent: `dev-guard`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
