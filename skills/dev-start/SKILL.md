---
name: dev-start
description: Parallel-agent project/feature kickoff — 4 agents design contracts, scaffold tests, review security, validate structure BEFORE any code is written. Prevents 80% of audit findings at the root.
---

# /dev-start — Parallel Feature Kickoff

> 80% проблем из аудитов = код написан без подготовки. Нет контрактов, нет тестов, нет security review.
> dev-start запускает 4 агента ПАРАЛЛЕЛЬНО до первой строки кода.

## Команды

- `/dev-start [описание фичи]` → полный kickoff (4 агента)
- `/dev-start lite [описание]` → быстрый (2 агента: contract + tests)

---

## Anti-Hallucination (наследуется от wave-audit)

Каждый агент ОБЯЗАН:
- **CODE_QUOTE** — цитировать существующий код при ссылках на него
- **VERIFY_CMD** — давать команду для проверки каждого утверждения
- **COVERAGE_LOG** — перечислить прочитанные файлы
- **SEVERITY_CALIBRATION** — не раздувать severity

---

## Phase 0 — Context Discovery (lead)

1. Определить стек проекта (читать package.json / go.mod / pubspec.yaml — НЕ из memory)
2. Найти аналогичные фичи в проекте (grep/glob по похожим паттернам)
3. Прочитать CLAUDE.md, DECISIONS.md если есть
4. Сформулировать scope: какие файлы будут затронуты

---

## Phase 1 — Parallel Kickoff (4 агента одним сообщением)

### Agent: `ds-contract`

```
Проект: [path], стек: [стек].
Фича: [описание].

Задача: спроектировать API контракт ДО реализации.

1. Определи ВСЕ types/interfaces которые нужны для фичи
2. Определи API boundaries — какие модули взаимодействуют
3. Проверь существующие типы — НЕ дублируй (SSOT):
   - Grep по проекту: есть ли похожие типы?
   - Если есть — переиспользуй, НЕ создавай новые
4. Определи data flow: откуда данные → куда → в каком формате
5. Для каждого join/merge — определи ключ (НЕ индекс массива!)

Output:
- Список types/interfaces с полями
- Data flow diagram (текстом)
- Список переиспользуемых типов (с путями)
- Предупреждения о потенциальных N+1 / data integrity issues

CODE_QUOTE существующих типов обязательна.
```

### Agent: `ds-tests`

```
Проект: [path], стек: [стек].
Фича: [описание].

Задача: написать скелет тестов ДО реализации (TDD).

1. Определи critical paths фичи
2. Для каждого path — напиши test case:
   - Happy path
   - Edge cases (null, empty, boundary values)
   - Error cases (network fail, invalid input, timeout)
   - Auth cases (unauthorized, wrong role)
3. Определи test framework проекта (jest/vitest/pytest/go test)
4. Создай файлы тестов с описательными `it()`/`test()` — тела пустые (TODO)

Output:
- Список test files с путями
- Содержимое каждого test file (skeleton)
- Какие mocks/fixtures нужны

НЕ пиши реализацию — только тесты.
```

### Agent: `ds-security`

```
Проект: [path], стек: [стек].
Фича: [описание].

Задача: security review ДО написания кода.

1. Классифицируй фичу по risk level:
   - HIGH: auth, payments, user data, crypto, file upload
   - MEDIUM: API endpoints, forms, search, notifications
   - LOW: UI-only, docs, formatting

2. Для HIGH/MEDIUM — чек-лист требований:
   - [ ] Input validation — какие поля, какие constraints?
   - [ ] Auth/authz — кто может вызывать? какие роли?
   - [ ] Rate limiting — нужен? какой лимит?
   - [ ] Data exposure — что НЕ должно попасть в response?
   - [ ] Secrets — нужны ли новые ключи/токены? Где хранить?
   - [ ] Crypto — если JWT/подписи: verify ОБЯЗАТЕЛЕН (НЕ decode)
   - [ ] Pagination — max limit обязателен

3. Проверь существующие auth/security паттерны в проекте:
   - Как другие endpoints делают auth?
   - Есть ли middleware? Используется ли?
   - Есть ли rate limiting? Работает ли? (grep по коду)

Output:
- Risk level фичи
- Security requirements checklist (заполненный)
- Паттерны из проекта которые НАДО переиспользовать
- Антипаттерны из проекта которые НЕ НАДО повторять
- CODE_QUOTE существующих security паттернов
```

### Agent: `ds-structure`

```
Проект: [path], стек: [стек].
Фича: [описание].

Задача: спланировать файловую структуру.

1. Определи какие файлы нужно создать / изменить
2. Для каждого файла — оценка LOC:
   - Если >150 LOC predicted → split на 2+ файла СРАЗУ
   - Если функция >25 LOC predicted → split на подфункции СРАЗУ
3. Проверь Constructor Pattern:
   - 1 файл = 1 класс = 1 ответственность
   - Types отдельно от реализации
   - Нет ли существующих файлов >200 LOC которые фича усугубит
4. Проверь зависимости:
   - Нужны ли новые пакеты?
   - Есть ли уже установленные пакеты для этой задачи?
   - Нет ли dead dependencies которые можно переиспользовать?

Output:
- File plan: [path] — [responsibility] — [estimated LOC]
- Files to modify: [path] — [current LOC] → [estimated LOC after]
- Constructor Pattern warnings
- Dependency recommendations
```

---

## Phase 2 — Synthesis (lead)

1. Собрать выводы 4 агентов
2. Проверить конфликты:
   - ds-contract предлагает тип, ds-security говорит не экспонировать поле?
   - ds-structure предлагает файл, ds-tests ожидает другой путь?
3. Сформировать **Development Brief**:

```
=== DEVELOPMENT BRIEF ===
Feature: [название]
Risk: HIGH/MEDIUM/LOW

## Contracts
[types/interfaces from ds-contract]

## Test Plan
[test files from ds-tests]

## Security Requirements
[checklist from ds-security]

## File Plan
[structure from ds-structure]

## Warnings
[potential issues caught before coding]

## Ready to code: YES/NO
[если NO — что нужно решить сначала]
```

4. Показать пользователю для approve
5. ТОЛЬКО после approve → начинать кодить

---

## Safety Constraints

- НЕ писать код до завершения Phase 2
- НЕ создавать новые типы если есть существующие (SSOT)
- НЕ добавлять зависимости без обоснования
- НЕ пропускать ds-security для HIGH risk фич
- При конфликте между агентами — спросить пользователя
