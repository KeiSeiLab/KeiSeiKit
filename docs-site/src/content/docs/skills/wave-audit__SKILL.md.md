---
title: SKILL
path: wave-audit/SKILL.md
dna_hash: sha256:99172735528542ad
language: markdown
size_loc: 571
generated: by-keidocs
---

# wave-audit/SKILL.md

## Public API

- `/wave-audit — 3-Wave Parallel Audit` — ---
- `When to use` — - Comprehensive code review: security, runtime analysis, quality, and Constructor Pattern in one 3-wave parallel workflow.
- `Команды` — - `/wave-audit` → полный review (3 волны, все фокусы)
- `Anti-Hallucination Protocol` — > AI галлюцинирует. Это не баг, это свойство. Skill ОБЯЗАН это компенсировать.
- `Phase 0 — Baseline Snapshot` — **Выполняет lead (не агент).** Обязательно ДО запуска Wave 1.
- `Wave 1 — Discovery (4 параллельных агента)` — **Создать TeamCreate**, запустить 4 агента **ОДНИМ сообщением** (параллельно).
- `Wave 2 — Verification (4 параллельных агента)` — **Запустить 4 агента ОДНИМ сообщением**, передав КАЖДОМУ полный список findings из Wave 1.
- `Wave 3 — Cross-Optimization (1 синтезатор)` — **Один агент** получает ВСЕ данные из Wave 1 + Wave 2.
- `Формат итогового отчёта` — ```
- `Baseline` — | Check | Status | Details |
- `Wave 1 — Discovery` — ### Security (w1-security): N findings
- `Wave 2 — Verification` — ### Confirmed: N | Rejected: N | Downgraded: N
- `Wave 3 — Optimized Results` — ### Priority Matrix
- `Verdict: GO / GO with constraints / NO-GO` — Reason: [explanation]
- `Apply Plan` — 1. checkpoint: before wave-audit fixes
- `Apply Phase (только после approve)` — 1. `checkpoint:` commit перед началом
- `Safety Constraints` — Запрещено:
- `Стандартный формат Finding (обязателен для ВСЕХ агентов)` — > Единый формат предотвращает потерю данных между волнами и упрощает дедупликацию.
- `Deduplication Protocol` — > 4 параллельных агента Wave 1 НЕИЗБЕЖНО найдут одно и то же. Без дедупликации — отчёт раздут на 30-40%.
- `Lite Mode (`/wave-audit lite`)` — > 9 агентов = дорого. Для малых проектов (<30 файлов) или быстрых проверок:
- `Diff Mode (`/wave-audit diff`)` — > Изменил 5 файлов — зачем аудитить все 90?
- `Lead Automation Protocol` — > Lead = bottleneck. Между волнами lead вручную собирает, форматирует, дедуплицирует, передаёт. Минимизируем ручную работу.
- `Wave 2 Context Overflow Protection` — > Если Wave 1 выдала 30+ findings, Wave 2 агенты теряют фокус на последних.
- `Метрика эффективности` — После каждого запуска записать в отчёт:

## Related

- parent: `wave-audit`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
