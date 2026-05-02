---
name: dev-ship
description: Pre-merge/pre-deploy parallel gate — 4 backend agents (security, tests, deps, regression) plus optional 5th frontend-final-gate agent (production build, Lighthouse, axe full, visual diff full) run final checks BEFORE code reaches main. Last line of defense.
---

# /dev-ship — Pre-Merge Quality Gate

## When to use

- Running a final quality gate before merging a feature branch into main.
- Verifying security, tests, deps, and regression checks accumulated across the whole branch.
- After `/dev-guard` approvals: `/dev-ship go` merges into main after parallel checks pass.

> Последний барьер перед main. Если dev-start предотвращает, dev-guard ловит на лету,
> то dev-ship — финальная проверка всего что накопилось в ветке.

## Команды

- `/dev-ship` → полная pre-merge проверка (4 агента)
- `/dev-ship check` → только проверка, без commit/push
- `/dev-ship go` → проверка + merge в main (после approve)

---

## Phase 0 — Branch Summary (lead)

1. `git log main..HEAD --oneline` → все коммиты в ветке
2. `git diff main..HEAD --stat` → полный diff от main
3. `git diff main..HEAD --name-only` → список затронутых файлов
4. Запустить CI: lint + build + test
5. Зафиксировать baseline:

```
SHIP BASELINE:
- Branch: [name] → main
- Commits: N
- Files changed: N
- Lines: +N / -N
- lint: PASS/FAIL
- build: PASS/FAIL
- tests: PASS/FAIL (N passed, M failed)
```

---

## Phase 1 — Final Gate (4 агента параллельно)

### Agent: `sh-security-final`

```
Финальный security review ВСЕХ изменений в ветке.
Проект: [path], стек: [стек].

Diff от main:
[git diff main..HEAD]

Это НЕ incremental review — это ПОЛНЫЙ review всего что уходит в main.

Checklist:
- [ ] Новые endpoints → все за auth? rate limited?
- [ ] Новые inputs → все валидируются?
- [ ] JWT/tokens → verify (НЕ decode)? отдельные ключи?
- [ ] Secrets → ни одного в коде? все в env?
- [ ] Error responses → не leakают internal details?
- [ ] SQL/NoSQL → параметризованные запросы?
- [ ] File operations → нет path traversal?
- [ ] Dependencies → новые пакеты? CVE check?
- [ ] .env файлы → НЕ в git? (`git ls-files .env`)

VERIFY_CMD для каждого finding:
- `grep -rn "jwt.decode" [dir]`
- `grep -rn "hardcoded\|secret\|password" [dir]`
- `git ls-files "*.env"`

Каждый finding → CODE_QUOTE + SEVERITY_CALIBRATION.
```

### Agent: `sh-test-validator`

```
Проверь тестовое покрытие ВСЕХ изменений в ветке.
Проект: [path], стек: [стек].

Изменённые файлы:
[git diff main..HEAD --name-only]

Для КАЖДОГО изменённого source файла:
1. Есть ли тест? (Glob: *test*, *spec*, *_test*)
2. Если есть → покрывает ли новый код? (grep по function names)
3. Если нет → FINDING: untested file

Запусти тесты:
- [lint command]
- [test command]

Для каждого FAILED теста:
- Это regression (работало до изменений)?
- Это новый тест который не написан?
- Это flaky test (не связан с изменениями)?

Output:
- Coverage map: [file] → [test file] / [NO TEST]
- Test results: PASS/FAIL per test
- Untested critical paths: [list]
- VERIFY_CMD: [test commands]
```

### Agent: `sh-dependency-audit`

```
Проверь ВСЕ зависимости проекта.
Проект: [path], стек: [стек].

1. НОВЫЕ зависимости (diff package.json / go.mod / pubspec.yaml от main):
   - Maintainers count (1 = bus factor risk)
   - Last release date (>6 мес = red flag)
   - CVE history
   - Нет ли в node_modules `.github/FUNDING.yml` (donation-dependent maintainer)
   - Bundle size impact

2. DEAD зависимости:
   - Для каждого package: grep по import/require в source
   - Если 0 imports → DEAD dependency → remove

3. DUPLICATE зависимости:
   - Несколько пакетов для одной задачи? (e.g., 4 map libraries)
   - Пакет в devDeps но import в source? (wrong category)

4. LOCK FILE:
   - lock файл актуален? (`npm ls --all 2>&1 | grep "missing"`)
   - lock файл в git? (обязательно)

Output:
- New deps: [list with assessment]
- Dead deps: [list — recommend removal]
- Duplicates: [list]
- Lock status: OK / OUTDATED / MISSING
```

### Agent: `sh-regression-check`

```
Проверь что изменения НЕ сломали существующий функционал.
Проект: [path], стек: [стек].

Diff от main:
[git diff main..HEAD --stat]

1. Для каждого ИЗМЕНЁННОГО файла:
   - Какие функции/exports изменились?
   - Кто их импортирует? (grep reverse dependencies)
   - Есть ли breaking changes в signature/return type?

2. Constructor Pattern delta:
   - Файлы >200 LOC ДО изменений: [list]
   - Файлы >200 LOC ПОСЛЕ изменений: [list]
   - Новые нарушения? Ухудшение?

3. SSOT delta:
   - Новые дублирования типов?
   - Существующие дублирования усугублены?

4. Data flow integrity:
   - Изменён формат данных? → все consumers обновлены?
   - Изменён API response? → клиент обновлён?
   - Изменена DB schema? → миграция есть?

Output:
- Breaking changes: [list or NONE]
- Constructor Pattern delta: improved / same / degraded
- SSOT violations: [new / same / fixed]
- Data flow issues: [list or NONE]
```

### Agent: `sh-frontend-final-gate` (опциональный 5-й)

> Запускается если diff содержит `*.tsx | *.ts | *.svelte | *.vue | *.dart` ИЛИ
> затронут DB-layer (`migrations/*.sql`, `src/db/**`, `src/types/**`).

```
Спавнить с subagent_type: "frontend-validator". Prompt:

FINAL pre-merge frontend pass. Project: [path], stack: [stack].
Branch diff vs main: [git diff main..HEAD --stat].

This is the LAST line — do NOT skip any subsection. All must PASS or commit blocks.

1. **Production build** — `npm run build` (or `flutter build web`).
   PASS: zero errors, zero warnings about unused exports / unused imports.
   FAIL: any compile error, any runtime crash on build.

2. **Type-check strict** — `npx tsc --noEmit --strict` (force strict, even if tsconfig is lenient).
   PASS: zero errors.

3. **DB-contract drift** — `kei-db-contract <root> --strict`.
   PASS: drift_count == 0.

4. **Visual regression FULL** — `npm run visual-check` if script exists.
   PASS: zero pixel diff above 0.01 ratio across all routes × all viewports.
   No baseline yet → emit WARN, suggest `/visual-loop` first.

5. **A11y FULL** — `npm run a11y-check` if script exists, else `npx axe-cli` against built site.
   PASS: zero WCAG 2 AA violations.

6. **Lighthouse score gate** — `npx @lhci/cli autorun` if installed, else inline lighthouse via Playwright.
   Gates: perf >= 90, a11y >= 95, best-practice >= 90, seo >= 90 (production-build target).

Output format:
- BUILD: PASS / FAIL [details]
- TYPECHECK: PASS / FAIL [count]
- DB_CONTRACT: PASS / FAIL [drift_count]
- VISUAL: PASS / WARN / FAIL [diff_count]
- A11Y: PASS / FAIL [violations]
- LIGHTHOUSE: perf=N a11y=N best=N seo=N (PASS / FAIL)
- VERDICT: SHIP_OK / FIX_FIRST / REVIEW_NEEDED
```

Hard rules:
- BUILD FAIL → block ship.
- TYPECHECK FAIL → block ship.
- DB_CONTRACT FAIL → block ship (schema/types must align before merge).
- A11Y violations → block ship.
- VISUAL diff → REVIEW_NEEDED (user approves new baseline OR fixes regression).
- Lighthouse below threshold → WARN, ship with explicit user override.

---

## Phase 2 — Ship Decision (lead)

```
=== SHIP REPORT ===
Branch: [name] → main
Commits: N | Files: N | Lines: +N/-N

## Security Final
[findings or CLEAN]

## Test Coverage
[coverage map + test results]

## Dependencies
New: N | Dead: N | Duplicates: N

## Regression
Breaking: N | Pattern delta: [improved/degraded] | SSOT: [status]

## Frontend Final Gate (if frontend changes detected)
Build: PASS/FAIL | Typecheck: PASS/FAIL | DB-contract: PASS/FAIL/N drift
Visual: PASS/N diff | A11y: PASS/N violations | Lighthouse: perf=N/a11y=N

## Baseline Comparison
| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| lint warnings | N | N | +/-N |
| test failures | N | N | +/-N |
| files >200 LOC | N | N | +/-N |

## Verdict: SHIP / FIX / ABORT
```

### Verdict logic:
- **SHIP**: все CLEAN, tests PASS, no regression → merge
- **FIX**: findings есть но fixable → показать, ждать фикс, re-run
- **ABORT**: critical security, breaking changes, test regression → не мержить

### При `/dev-ship go` + SHIP verdict:
1. `checkpoint:` commit
2. `git checkout main && git merge [branch]`
3. Re-run tests on main
4. Если tests PASS → done
5. Если tests FAIL → `git reset --merge` + показать ошибку

---

## Strict Mode (default)

Merge блокируется если ПОСЛЕ хуже чем ДО по ЛЮБОМУ из:
- Больше lint errors
- Больше test failures
- Больше файлов >200 LOC
- Новые CRITICAL/HIGH security findings

---

## Safety

- НЕ мержить без verdict SHIP
- НЕ force-push в main НИКОГДА
- НЕ пропускать тесты ("потом починим")
- НЕ мержить с [UNVERIFIED] findings
- При ABORT — записать причину в DECISIONS.md
