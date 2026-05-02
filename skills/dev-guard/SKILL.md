---
name: dev-guard
description: Continuous parallel quality gate during development — 3 backend agents (security/perf/structure) plus optional 4th frontend-validator agent (TS types ↔ DB schema drift, lint, type-check, visual diff) check every significant code change in real-time. Run after writing code, before committing.
---

# /dev-guard — Continuous Development Guard

## When to use

- After writing a new file or module to catch security, performance, or structural issues before committing.
- Before every commit, or in place of a manual commit (`/dev-guard commit`).
- After refactoring 3+ files or changing auth / payments / data layer code.

> wave-audit находит проблемы ПОСЛЕ. dev-guard ловит их ПОКА пишешь код.
> 3 backend агента + опциональный frontend-validator проверяют каждое значимое изменение.

## Команды

- `/dev-guard` → проверить текущие изменения (git diff)
- `/dev-guard [file]` → проверить конкретный файл
- `/dev-guard commit` → guard + auto-commit если clean

---

## Когда запускать

- После написания нового файла / модуля
- После изменения auth / payments / data layer
- Перед каждым commit (или вместо — `/dev-guard commit`)
- После рефакторинга 3+ файлов

---

## Phase 0 — Scope Detection (lead)

1. `git diff --name-only` → список изменённых файлов
2. Для каждого файла → классифицировать:
   - **HOT**: auth, crypto, payments, data layer, resolvers/handlers → все 3 агента
   - **WARM**: services, utils, config → 2 агента (security skip)
   - **COLD**: UI components, styles, docs → 1 агент (structure only)
3. Собрать diff только для HOT/WARM файлов

---

## Phase 1 — Parallel Guard (3 агента одним сообщением)

### Agent: `dg-security`

> Только для HOT файлов.

```
Проверь ТОЛЬКО изменённый код (diff ниже). Проект: [path], стек: [стек].

Diff:
[git diff output]

Checklist на КАЖДОЕ изменение:
- [ ] Новый endpoint/resolver? → есть auth middleware?
- [ ] Принимает input? → есть validation (Zod/joi/validator)?
- [ ] Работает с JWT? → verify(), НЕ decode()
- [ ] Работает с DB? → параметризованные запросы? нет string concat?
- [ ] Работает с файлами? → path traversal check?
- [ ] Новый secret/key? → НЕ в коде, в env?
- [ ] Error response? → не leakает stack trace / internal details?
- [ ] Rate limiting? → есть для auth endpoints?

Формат finding:
- CODE_QUOTE (из diff, 3-7 строк)
- PROBLEM + IMPACT
- FIX (конкретное изменение)
- VERIFY_CMD

Если всё чисто → `SECURITY: CLEAN (N checks passed)`.
```

### Agent: `dg-performance`

> Для HOT и WARM файлов.

```
Проверь ТОЛЬКО изменённый код (diff ниже). Проект: [path], стек: [стек].

Diff:
[git diff output]

Checklist:
- [ ] DB/API call внутри цикла или map? → N+1 alert
- [ ] Данные объединяются по индексу массива? → join by key instead
- [ ] JSON.parse на cached data? → Date/типы теряются
- [ ] new Service() на каждый request? → singleton/DI
- [ ] Нет limit на pagination/query? → max cap обязателен
- [ ] Sync I/O на async path? → await/async
- [ ] Массив растёт без bounds? → max size
- [ ] Тяжёлая операция без cache? → добавить
- [ ] Resolver fetches data it already has? → pass down, don't re-fetch

Если всё чисто → `PERFORMANCE: CLEAN (N checks passed)`.
```

### Agent: `dg-structure`

> Для ВСЕХ файлов (HOT/WARM/COLD).

```
Проверь ТОЛЬКО изменённый код (diff ниже). Проект: [path], стек: [стек].

Diff:
[git diff output]

Constructor Pattern Checks:
- [ ] Файл после изменения >200 LOC? → `wc -l [file]`
- [ ] Новая/изменённая функция >30 LOC? → split
- [ ] Новый тип/interface? → уже есть похожий? (grep)
- [ ] Import нового пакета? → уже есть аналог в deps?
- [ ] Дублирование кода? → extract shared util
- [ ] Стиль consistent с остальным проектом?

Для каждого файла ОБЯЗАТЕЛЕН `wc -l` через VERIFY_CMD.
Если всё чисто → `STRUCTURE: CLEAN (N checks passed, all files <200 LOC)`.
```

### Agent: `frontend-validator` (опциональный 4-й)

> Запускается если в diff есть `*.tsx | *.ts | *.svelte | *.vue | *.dart` ИЛИ затронут DB-layer
> (`migrations/*.sql`, `src/db/**`, `src/types/**`, `prisma/schema.prisma`, `drizzle.config.*`).

```
Спавнить с subagent_type: "frontend-validator". Prompt:

Frontend continuous-validation pass. Project root: [path], stack: [Next.js / Vite / Flutter / ...].
Changed files: [список из git diff --name-only].

Run in order, emit per-section status:

1. Stack detect (package.json / pubspec.yaml / next.config.*).
2. Type-check (npx tsc --noEmit OR dart analyze). List file:line errors.
3. Lint (npx eslint . --ext .ts,.tsx OR dart analyze). List warnings.
4. DB-contract drift — invoke `kei-db-contract <root> --output json` if binary in PATH AND project
   has migrations/ OR src/db/. Parse drift_count + per-table fields.
5. Visual regression — only if playwright.config.* exists. Skip otherwise.
6. Verdict block: each check status (PASS / WARN / FAIL) + brief evidence.

Output format:
- TYPE_CHECK: <status> [<count> errors]
- LINT: <status> [<count> warnings]
- DB_CONTRACT: <status> [<drift_count> tables drifted]
- VISUAL_DIFF: <status>
- VERDICT: COMMIT_OK / FIX_FIRST / REVIEW_NEEDED

Read-only on the codebase; do NOT autofix. Hand off TS errors to code-implementer-typescript.
```

Severity rules:
- **TYPE_CHECK FAIL** → block commit (hard).
- **DB_CONTRACT drift_count > 0** → block commit, suggest schema-or-types update.
- **LINT warnings** → advisory.
- **VISUAL_DIFF mismatch** → review needed (user approves new baseline OR fixes regression).

---

## Phase 2 — Quick Synthesis (lead, НЕ агент)

Lead объединяет результаты за 30 секунд:

```
=== DEV GUARD REPORT ===
Files checked: N (HOT: N, WARM: N, COLD: N)

Security:    CLEAN / N findings
Performance: CLEAN / N findings
Structure:   CLEAN / N findings

[findings if any — grouped by file]

Verdict: COMMIT / FIX FIRST / REVIEW NEEDED
```

### Verdict logic:
- **COMMIT**: все CLEAN или только LOW findings → можно коммитить
- **FIX FIRST**: есть HIGH/CRITICAL → показать findings, ждать фикс
- **REVIEW NEEDED**: спорные findings → показать пользователю для решения

### При `/dev-guard commit`:
- Если COMMIT → автоматически `git add [changed files] && git commit`
- Если FIX FIRST → показать findings, НЕ коммитить
- Commit message генерируется из diff (semantic: feat/fix/refactor)

---

## Adaptive Depth

> Не гонять 3 агента на каждый CSS-файл.

| Изменение | Агенты | Время |
|-----------|--------|-------|
| auth/crypto/payments | security + performance + structure | ~2 min |
| services/utils/config | performance + structure | ~1 min |
| UI/styles/docs | structure only | ~30 sec |
| 1 файл, <20 строк | lead проверяет сам (без агентов) | ~10 sec |
| frontend (.tsx/.ts/.dart) | + frontend-validator | +30-60 sec |
| DB layer (migrations/, src/db/, schema.prisma) | + frontend-validator (DB contract) | +20 sec |

---

## Integration с git workflow

### Pre-commit (рекомендуется):
```bash
# В .claude/hooks/ или git hooks
# Запускать /dev-guard перед каждым commit
```

### Post-change (автоматически):
При работе в сессии Claude, после каждого блока изменений (3+ файлов) → lead предлагает `/dev-guard`.

---

## Safety

- НЕ блокировать commit на LOW findings
- НЕ автокоммитить если есть HIGH+
- НЕ запускать 3 агента на тривиальные изменения
- Findings без CODE_QUOTE → [UNVERIFIED], понижение severity
