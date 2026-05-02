---
name: wave-audit
description: 3-wave parallel audit — Wave 1 discovery (4 agents), Wave 2 verification (4 agents), Wave 3 cross-optimization (1 synthesizer). Each wave runs agents in parallel. Combines security review, runtime analysis, quality check, and constructor pattern into one unified workflow with evidence grading and Go/No-Go verdict.
---

# /wave-audit — 3-Wave Parallel Audit

## When to use

- Comprehensive code review: security, runtime analysis, quality, and Constructor Pattern in one 3-wave parallel workflow.
- Before merging a significant branch or shipping a milestone (95%+ finding coverage via cross-verification).
- After `/dev-guard` commit approvals when you need a deeper, cross-dependency audit sweep.

> Один аудитор находит 60%. Два параллельных — 80%. Три волны с перекрёстной проверкой — 95%+.
> Доказано экспериментом 2026-03-24: ни один подход в изоляции не покрыл все findings.

## Команды

- `/wave-audit` → полный review (3 волны, все фокусы)
- `/wave-audit review [focus]` → выборочный review
- `/wave-audit lite` → быстрый режим (2 агента Wave 1, без Wave 2, упрощённый Wave 3)
- `/wave-audit diff` → инкрементальный (только изменённые файлы + их зависимости)
- `/wave-audit apply` → применить фиксы в worktree (после approve)

`focus`: `security | runtime | quality | structure | all` (default: `all`)

---

## Anti-Hallucination Protocol

> AI галлюцинирует. Это не баг, это свойство. Skill ОБЯЗАН это компенсировать.

### Правило CODE_QUOTE
Каждый finding ОБЯЗАН содержать **дословную цитату кода** (3-7 строк), НЕ описание.

```
BAD:  "Функция не верифицирует JWT подпись"
GOOD: "apple.ts:72-74: `const decoded = jwt.decode(idToken, { complete: true });`
       — jwt.decode() НЕ верифицирует подпись, нужен jwt.verify()"
```

Если агент НЕ может процитировать код — finding отклоняется как unverified.

### Правило VERIFY_CMD
Каждый finding ОБЯЗАН содержать команду верификации — как подтвердить без чтения кода:

```
verify: grep -n "jwt.decode" apps/api/src/external/oauth/apple.ts
verify: wc -l apps/api/src/services/auth.service.ts  # expect >200
verify: grep -rn "new AuthService" apps/api/src/  # count instantiations
```

### Правило COVERAGE_LOG
Каждый агент Wave 1 ОБЯЗАН в конце отчёта указать:
```
FILES_READ: [список файлов которые реально открыл через Read tool]
FILES_TOTAL: [общее число source files]
COVERAGE: N% (read/total)
NOT_CHECKED: [список файлов/модулей которые НЕ проверял]
```

### Правило SEVERITY_CALIBRATION
Перед назначением severity, агент ОБЯЗАН ответить на вопрос:
- CRITICAL: "Можно ли эксплуатировать прямо сейчас без аутентификации?" Если нет → max HIGH.
- HIGH: "Приведёт ли это к потере данных или компрометации в production?" Если нет → max MEDIUM.
- MEDIUM: "Заметит ли это пользователь?" Если нет → LOW.

---

## Phase 0 — Baseline Snapshot

**Выполняет lead (не агент).** Обязательно ДО запуска Wave 1.

1. Проверить что это git-репозиторий
2. Определить реальный стек (НЕ из memory — читать package.json / pubspec.yaml / go.mod / Cargo.toml)
3. Снять baseline:
   - `git log --oneline -5` — последние коммиты
   - `git status --short` — текущее состояние
   - `git diff --stat` — незакоммиченные изменения
4. Определить и запустить CI-проверки стека:
   - **JS/TS:** `npm run lint`, `npm run build`, `npm test` (или turbo/nx аналоги)
   - **Flutter:** `flutter analyze`, `flutter test`
   - **Go:** `go vet ./...`, `go build ./...`, `go test ./...`
   - **Python:** `ruff check .`, `pytest`
   - **Rust:** `cargo clippy`, `cargo build`, `cargo test`
5. Зафиксировать baseline-метрики:
   ```
   BASELINE:
   - lint: PASS/FAIL (N warnings, M errors)
   - build: PASS/FAIL
   - tests: PASS/FAIL (N passed, M failed, K skipped)
   - stack: [detected stack]
   - files: [total source files count]
   ```

---

## Wave 1 — Discovery (4 параллельных агента)

**Создать TeamCreate**, запустить 4 агента **ОДНИМ сообщением** (параллельно).
Каждый агент работает независимо, НЕ знает о findings других.

### Agent: `w1-security`

```
Проведи security review проекта [path].
Стек: [из baseline]. НЕ применяй фиксы.

Phase 1 — Risk Classification:
- HIGH: auth, crypto, network, memory, deserialization, FFI
- MEDIUM: input validation, error handling, config, logging, API
- LOW: docs, tests, formatting

Phase 2 — Vulnerability Checklist:
- [ ] Input validation — все входные данные валидируются?
- [ ] Auth/authz bypass — можно обойти?
- [ ] Race condition — shared state без lock? TOCTOU?
- [ ] Injection — SQL, command, path traversal, SSTI?
- [ ] Secrets — hardcoded keys, tokens в логах?
- [ ] Deserialization — untrusted input в unmarshal/decode?
- [ ] Resource exhaustion — unbounded allocations, missing timeouts?
- [ ] Crypto — правильные алгоритмы? верификация подписей?

Phase 3 — Supply Chain:
Для КАЖДОЙ зависимости: maintainers, activity, CVE, transitive deps, dead deps.

Каждый finding ОБЯЗАН содержать:
1. severity (CRITICAL/HIGH/MEDIUM/LOW) — пройди SEVERITY_CALIBRATION
2. файл:строка
3. CODE_QUOTE (3-7 строк дословно)
4. attack scenario
5. VERIFY_CMD (команда для подтверждения)
6. [E1-E6] evidence grade

В конце — COVERAGE_LOG: какие файлы читал, какие нет.
Отдай как structured list.
```

### Agent: `w1-runtime`

```
Проведи runtime/performance review проекта [path].
Стек: [из baseline]. НЕ применяй фиксы.

Checklist:
- [ ] N+1 queries — ORM/DB calls в циклах или per-item resolvers
- [ ] Data joins — merge по индексу вместо ключа? silent corruption?
- [ ] Cache invalidation — JSON.parse теряет типы? stale data?
- [ ] Hot paths — какие функции на критическом пути?
- [ ] Allocations — per-request instantiation? можно singleton?
- [ ] I/O — sync operations на async path? blocking calls?
- [ ] Complexity — O(n²) или хуже в hot loops?
- [ ] Renders/rebuilds — лишние re-renders в UI? missing memo?
- [ ] Timeouts — есть ли на внешних API calls? unbounded waits?
- [ ] Connection pools — DB/Redis connections managed?

Каждый finding ОБЯЗАН содержать:
1. severity — пройди SEVERITY_CALIBRATION
2. файл:строка
3. CODE_QUOTE (3-7 строк дословно)
4. impact estimate (latency/memory/throughput)
5. VERIFY_CMD
6. [E1-E6] evidence grade

В конце — COVERAGE_LOG.
```

### Agent: `w1-quality`

```
Проведи quality review проекта [path].
Стек: [из baseline]. НЕ применяй фиксы.

Checklist:
- [ ] DRY — дублированный код? одинаковая логика в разных файлах?
- [ ] SRP — файл делает больше одного? mixed responsibilities?
- [ ] Naming — нечитаемые имена? inconsistent conventions?
- [ ] Dead code — unreachable branches? unused exports/imports?
- [ ] Edge cases — null/undefined handling? empty arrays? boundary values?
- [ ] Error handling — consistent pattern? fail open vs fail closed?
- [ ] Error messages — информативные? не leakают internal details?
- [ ] Type safety — any/unknown? missing generics? loose types?
- [ ] API boundaries — чёткие контракты между модулями?

Каждый finding ОБЯЗАН содержать:
1. severity — пройди SEVERITY_CALIBRATION
2. файл:строка
3. CODE_QUOTE (3-7 строк дословно)
4. impact
5. VERIFY_CMD
6. [E1-E6] evidence grade

В конце — COVERAGE_LOG.
```

### Agent: `w1-structure`

```
Проведи structural review проекта [path].
Стек: [из baseline]. НЕ применяй фиксы.

Constructor Pattern Checklist:
- [ ] Файлы >200 строк — перечисли ВСЕ с точным LOC
- [ ] Функции >30 строк — перечисли ВСЕ с точным LOC
- [ ] 1 файл = 1 класс = 1 ответственность — нарушения?
- [ ] Types/interfaces определены ДО реализации?
- [ ] SSOT — типы, enum, routes, configs дублируются?

CI/Pipeline Check:
- [ ] Все команды из CI (lint/build/test) проходят?
- [ ] Есть ли пустые stubs/packages ломающие pipeline?
- [ ] Lock-файлы актуальны?

Dependency Health:
- [ ] Unused dependencies (installed but never imported)
- [ ] Outdated deps (major version behind)
- [ ] Conflicting deps (multiple packages для одной задачи)

Каждый finding ОБЯЗАН содержать:
1. severity — пройди SEVERITY_CALIBRATION
2. файл:строка
3. CODE_QUOTE или точные числа (LOC count)
4. VERIFY_CMD (например: `wc -l file.ts`)
5. [E1-E6] evidence grade

В конце — COVERAGE_LOG.
```

### Сбор результатов Wave 1

Lead собирает findings от всех 4 агентов в единый список.
Формат: `W1-{agent}-{N}: [severity] description [evidence grade]`

---

## Wave 2 — Verification (4 параллельных агента)

**Запустить 4 агента ОДНИМ сообщением**, передав КАЖДОМУ полный список findings из Wave 1.

### Agent: `w2-false-positive-filter`

```
Получи ВСЕ findings из Wave 1: [список]
Проект: [path], стек: [стек].

Для КАЖДОГО finding:
1. Прочитай код по указанному файлу:строке
2. Это РЕАЛЬНАЯ проблема или false positive?
3. Если false positive — ПОЧЕМУ (например: "anon key публичный by design Supabase")
4. Если реальная — подтверди severity или скорректируй

Output:
- CONFIRMED: [finding ID] — [подтверждённый severity]
- DOWNGRADED: [finding ID] — [новый severity] — [причина]
- REJECTED: [finding ID] — [причина]
```

### Agent: `w2-gap-hunter`

```
Получи ВСЕ findings из Wave 1: [список]
Проект: [path], стек: [стек].

Задача: найти то, что Wave 1 ПРОПУСТИЛА.

1. Variant analysis: каждый найденный баг = КЛАСС. Проверь аналогичные места:
   - Exact: тот же код в другом файле
   - Structural: тот же паттерн в другом модуле
   - Semantic: та же логическая ошибка с другим синтаксисом

2. Пробелы в покрытии:
   - Какие файлы/модули Wave 1 не проверяла?
   - Есть ли critical paths без findings? (подозрительно — или чисто, или пропущено)

3. Cross-domain gaps:
   - Security нашёл auth issue — runtime проверил performance auth flow?
   - Runtime нашёл N+1 — quality проверил error handling в том же resolver?

Output: список ДОПОЛНИТЕЛЬНЫХ findings с severity + файл:строка + [E1-E6].
```

### Agent: `w2-fix-impact`

```
Получи ВСЕ findings из Wave 1: [список]
Проект: [path], стек: [стек].

Для каждого finding с предложенным фиксом:
1. Какие файлы затронет фикс? (blast radius)
2. Может ли фикс сломать существующий функционал?
3. Нужна ли миграция данных/токенов/состояния?
4. Зависит ли фикс от других фиксов? (порядок применения)
5. Estimated complexity: trivial / moderate / significant

Output:
- [finding ID]: blast_radius=[N files], breaks=[что может сломать], deps=[зависит от], complexity=[level]
```

### Agent: `w2-architecture-guard`

```
Получи ВСЕ findings из Wave 1: [список]
Проект: [path], стек: [стек].

Для каждого предложенного фикса проверь:
1. Не нарушает ли Constructor Pattern? (файл останется <200? функция <30?)
2. Не создаёт ли новое дублирование? (SSOT)
3. Не вводит ли запрещённые паттерны? (миксины, DI-контейнеры, абстрактные фабрики)
4. Сохраняет ли API boundaries между модулями?
5. Совместим ли с текущей архитектурой проекта?

Если фикс нарушает архитектуру — предложи АЛЬТЕРНАТИВНЫЙ фикс.

Output:
- [finding ID]: architecture_safe=YES/NO, conflict=[описание], alternative=[если NO]
```

### Сбор результатов Wave 2

Lead объединяет:
- Confirmed/rejected/downgraded findings
- New findings от gap-hunter
- Impact analysis
- Architecture conflicts

---

## Wave 3 — Cross-Optimization (1 синтезатор)

**Один агент** получает ВСЕ данные из Wave 1 + Wave 2.

### Agent: `w3-synthesizer`

```
Получи ВСЕ данные:
- Wave 1 findings: [список]
- Wave 2 verifications: [confirmed/rejected/downgraded]
- Wave 2 new findings: [от gap-hunter]
- Wave 2 impact analysis: [blast radius, deps]
- Wave 2 architecture conflicts: [список]
Проект: [path], стек: [стек], baseline: [метрики].

Задачи:

1. CROSS-DOMAIN ANALYSIS
   Найди пересечения: security fix ломает performance?
   Runtime optimization создаёт security hole?
   Два фикса трогают один файл — порядок?

2. OPTIMAL SOLUTIONS
   Для каждой группы связанных findings — найди ОДНО решение вместо N патчей.
   Пример: 3 HTTP клиента с дублированным error handling → один base client.
   Пример: N+1 + cache issue в одном resolver → DataLoader + cache fix вместе.

3. PRIORITY MATRIX
   Ранжируй ВСЕ confirmed findings по формуле:
   score = severity_weight × (1 / fix_complexity) × blast_radius_factor
   severity: CRITICAL=10, HIGH=7, MEDIUM=4, LOW=1
   complexity: trivial=1, moderate=0.5, significant=0.3
   blast_radius: 1-3 files=1.0, 4-10=0.8, 10+=0.5

4. EVIDENCE GRADES
   Каждый finding — [E1]-[E6] с обоснованием.

5. GO/NO-GO VERDICT
   - GO: нет CRITICAL, все HIGH имеют простой фикс
   - GO with constraints: есть CRITICAL но workaround возможен
   - NO-GO: CRITICAL без workaround, или >3 HIGH без фиксов

6. APPLY PLAN (если GO/GO with constraints)
   Порядок фиксов: low-risk → medium-risk → high-risk.
   Каждый фикс = отдельный checkpoint commit.
   После каждого — re-run baseline checks.
```

---

## Формат итогового отчёта

```
=== WAVE AUDIT REPORT ===
Project: [name] | Stack: [detected] | Date: [date]

## Baseline
| Check | Status | Details |
|-------|--------|---------|
| lint  | PASS/FAIL | N warnings, M errors |
| build | PASS/FAIL | ... |
| tests | PASS/FAIL | N passed, M failed |

## Wave 1 — Discovery
### Security (w1-security): N findings
### Runtime (w1-runtime): N findings
### Quality (w1-quality): N findings
### Structure (w1-structure): N findings
Total: N findings

## Wave 2 — Verification
### Confirmed: N | Rejected: N | Downgraded: N
### New findings (gap-hunter): N
### Architecture conflicts: N
### High-risk fixes: [list]

## Wave 3 — Optimized Results

### Priority Matrix
| # | Severity | Finding | Fix | Complexity | Blast | Score | [E] |
|---|----------|---------|-----|------------|-------|-------|-----|
| 1 | CRITICAL | ...     | ... | trivial    | 2     | 10.0  | E1  |

### Optimal Solutions (grouped)
- Group A: [related findings] → [one fix]
- Group B: [related findings] → [one fix]

### Cross-Domain Conflicts
- [conflict description + resolution]

## Verdict: GO / GO with constraints / NO-GO
Reason: [explanation]

## Apply Plan
1. checkpoint: before wave-audit fixes
2. [fix #1] — [files] — low risk
3. [fix #2] — [files] — low risk
...
N. re-run baseline → compare with pre-audit
```

---

## Apply Phase (только после approve)

1. `checkpoint:` commit перед началом
2. Создать worktree: `git worktree add .claude/worktrees/wave-audit-fix`
3. Применять фиксы в порядке из apply plan (low-risk → high-risk)
4. Каждый фикс = отдельный commit
5. Re-run baseline checks в worktree
6. Strict mode: если хуже baseline по любому критерию → STOP
7. Hand-off: показать diff, результаты, остаточные риски

---

## Safety Constraints

Запрещено:
- Исправлять что-либо до завершения ВСЕХ 3 волн
- Пропускать Wave 2 ("Wave 1 всё нашла")
- Пропускать Wave 3 ("нет конфликтов") — Wave 3 ВСЕГДА
- `git reset --hard`, удаление файлов/веток без approve
- Менять секреты (`.env`, ключи, токены)
- Объединять findings без маркировки источника (W1/W2/W3)

---

## Стандартный формат Finding (обязателен для ВСЕХ агентов)

> Единый формат предотвращает потерю данных между волнами и упрощает дедупликацию.

```
ID: W{wave}-{agent}-{N}
SEVERITY: CRITICAL|HIGH|MEDIUM|LOW
CALIBRATION: [ответ на вопрос из SEVERITY_CALIBRATION]
FILE: path/to/file.ts:42-48
CODE_QUOTE: |
  const decoded = jwt.decode(idToken, { complete: true });
  // no jwt.verify() call — signature not checked
PROBLEM: [1-2 предложения]
IMPACT: [что произойдёт если не починить]
FIX: [конкретное изменение]
VERIFY_CMD: grep -n "jwt.decode" apps/api/src/external/oauth/apple.ts
EVIDENCE: [E1-E6]
```

Findings БЕЗ CODE_QUOTE или VERIFY_CMD → автоматически помечаются `[UNVERIFIED]` и понижаются на 1 severity.

---

## Deduplication Protocol

> 4 параллельных агента Wave 1 НЕИЗБЕЖНО найдут одно и то же. Без дедупликации — отчёт раздут на 30-40%.

### Lead между Wave 1 и Wave 2:
1. Собрать все findings в единый список
2. Группировать по `FILE` — findings на один и тот же файл:строку = кандидаты на дупликат
3. Для каждой группы:
   - Одинаковая проблема → оставить finding с лучшим CODE_QUOTE, удалить дубли, пометить `[DEDUP: merged W1-security-3 + W1-quality-7]`
   - Разные проблемы на одном файле → оставить оба
4. Пометить каждый finding источником: `SOURCE: w1-security`

### Ожидаемый dedup rate: 15-25%

---

## Lite Mode (`/wave-audit lite`)

> 9 агентов = дорого. Для малых проектов (<30 файлов) или быстрых проверок:

- Wave 1: **2 агента** (security+runtime merged, quality+structure merged)
- Wave 2: **пропуск** (lead делает quick false-positive check сам)
- Wave 3: **упрощённый** — priority list + Go/No-Go, без cross-domain analysis

Ожидаемое покрытие: ~75% от full mode. Стоимость: ~30% от full mode.

---

## Diff Mode (`/wave-audit diff`)

> Изменил 5 файлов — зачем аудитить все 90?

1. `git diff --name-only HEAD~N` → список изменённых файлов
2. Для каждого изменённого файла → найти зависимости (importers, tests)
3. Scope Wave 1 агентов ТОЛЬКО на: changed files + их прямые зависимости
4. Wave 2 gap-hunter дополнительно проверяет: не пропущены ли transitive dependencies

Ожидаемое покрытие: ~90% для changed code. Стоимость: ~40% от full mode.

---

## Lead Automation Protocol

> Lead = bottleneck. Между волнами lead вручную собирает, форматирует, дедуплицирует, передаёт. Минимизируем ручную работу.

### Между Wave 1 → Wave 2:
1. Собрать messages от 4 агентов (автоматически через team inbox)
2. Дедупликация по FILE (см. протокол выше)
3. Подсчитать coverage: объединить COVERAGE_LOG всех агентов
4. Если суммарный coverage <50% — **WARN пользователю**: "Агенты покрыли только N% файлов"
5. Сформировать единый список → передать в промпты Wave 2

### Между Wave 2 → Wave 3:
1. Собрать verifications от 4 агентов
2. Применить решения w2-false-positive-filter (удалить REJECTED)
3. Добавить new findings от w2-gap-hunter
4. Сформировать полный пакет → передать w3-synthesizer

### Если агент не ответил за 10 минут:
- Продолжить без него, пометить `[AGENT_TIMEOUT: w1-runtime]`
- В финальном отчёте указать неполное покрытие

---

## Wave 2 Context Overflow Protection

> Если Wave 1 выдала 30+ findings, Wave 2 агенты теряют фокус на последних.

### Правило: MAX 20 findings per Wave 2 agent
- Если findings >20: разбить на 2 batch по приоритету
- Batch 1 (HIGH+CRITICAL) → Wave 2 агентам
- Batch 2 (MEDIUM+LOW) → Wave 2 агентам вторым проходом ИЛИ lead проверяет сам
- В отчёте пометить: `BATCH_2_REVIEWED_BY: lead` (менее глубокая проверка)

---

## Метрика эффективности

После каждого запуска записать в отчёт:
```
AUDIT METRICS:
- Wave 1 findings: N (raw, before dedup)
- Wave 1 deduplicated: N (after dedup, -N%)
- Wave 1 coverage: N% (union of all agents' COVERAGE_LOGs)
- Wave 1 unverified: N (findings without CODE_QUOTE)
- Wave 2 rejected: N (false positive rate: N%)
- Wave 2 new findings: N (gap rate: N%)
- Wave 2 architecture conflicts: N
- Wave 3 groups: N (optimization rate: N%)
- Total agents: 9 (4+4+1) | lite: 3 (2+0+1)
- Total confirmed: N
- Agent timeouts: N
- Mode: full | lite | diff
```
