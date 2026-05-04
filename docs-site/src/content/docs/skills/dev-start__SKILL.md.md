---
title: SKILL
path: dev-start/SKILL.md
dna_hash: sha256:d22887af642ce84a
language: markdown
size_loc: 203
generated: by-keidocs
---

# dev-start/SKILL.md

## Public API

- `/dev-start — Parallel Feature Kickoff` — ---
- `When to use` — - Starting a new feature or project slice — design contracts, scaffold tests, and security review BEFORE any code is written.
- `Команды` — - `/dev-start [описание фичи]` → полный kickoff (4 агента)
- `Anti-Hallucination (наследуется от wave-audit)` — Каждый агент ОБЯЗАН:
- `Phase 0 — Context Discovery (lead)` — 1. Определить стек проекта (читать package.json / go.mod / pubspec.yaml — НЕ из memory)
- `Phase 1 — Parallel Kickoff (4 агента одним сообщением)` — ### Agent: `ds-contract`
- `Phase 2 — Synthesis (lead)` — 1. Собрать выводы 4 агентов
- `Contracts` — [types/interfaces from ds-contract]
- `Test Plan` — [test files from ds-tests]
- `Security Requirements` — [checklist from ds-security]
- `File Plan` — [structure from ds-structure]
- `Warnings` — [potential issues caught before coding]
- `Ready to code: YES/NO` — [если NO — что нужно решить сначала]
- `Safety Constraints` — - НЕ писать код до завершения Phase 2

## Related

- parent: `dev-start`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
