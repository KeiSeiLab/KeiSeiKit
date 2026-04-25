---
name: docs-scaffold
description: 5-phase documentation scaffolder — auto-detect project type, audit existing docs, generate CLAUDE.md / DECISIONS.md / runbook / README / diagrams / CHANGELOG from KeiSeiKit templates. Each phase is a click-driven file in this skill directory.
argument-hint: <project directory | omit to use $PWD>
---

# Docs-Scaffold — Project Documentation Pipeline (index)

> See `_blocks/pipeline-5phase-template.md` for the 5-phase wizard contract
> and `_blocks/rule-pure-click-contract.md` for the AskUserQuestion rule.
> Skill-specific phase tables are inline below.

You are bootstrapping or auditing the documentation layer of a repository.
The pipeline runs 5 phases in order; each phase owns one concern and has
its own verify-criterion. Never skip a phase. Never re-order.

This `SKILL.md` is the INDEX. Each phase lives in its own file.

---

## Pipeline overview (5 phases)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-intake.md](phase-1-intake.md) | Auto-detect stack + audit existing docs + pick gaps | 1× |
| 2 | [phase-2-scaffold.md](phase-2-scaffold.md) | Run `kei-docs-scaffold.sh` with selected type | 1× |
| 3 | [phase-3-decisions.md](phase-3-decisions.md) | Walk through first ADR (optional) | 1× |
| 4 | [phase-4-diagrams.md](phase-4-diagrams.md) | Seed Mermaid architecture starter | 1× |
| 5 | [phase-5-changelog.md](phase-5-changelog.md) | Init CHANGELOG via `kei-changelog` | 1× |

Minimum AskUserQuestion count: **5** (one per phase). Phases 3-5 each have
an early "Skip this phase" option to keep a lightweight run short.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `DIR` | Phase 1a | Target repo directory (defaults to `$PWD`) |
| `STACK` | Phase 1a | Detected stack label (Rust / Flutter / Node / …) |
| `EXISTING` | Phase 1b | Set of docs already present (CLAUDE.md, DECISIONS.md, …) |
| `GAPS` | Phase 1c | User-selected subset to scaffold |
| `SCAFFOLDED` | Phase 2 | Files actually written (could be < `GAPS` if `--force` declined) |
| `ADR_N` | Phase 3 | Number of ADR entries appended (0 if skipped) |
| `DIAGRAMS` | Phase 4 | Mermaid files seeded (0 if skipped) |
| `CHANGELOG_STATUS` | Phase 5 | initialized / updated / skipped |

---

## Final report (emit after Phase 5)

```
=== DOCS-SCAFFOLD REPORT ===
Target:      <DIR>
Stack:       <STACK>
Existing:    <list>
Scaffolded:  <list of new files>
ADRs added:  <ADR_N>
Diagrams:    <DIAGRAMS>
Changelog:   <CHANGELOG_STATUS>
Next action: <what user should run / review / commit>
```

---

## Rules (apply throughout)

- **Pure-click contract.** Every phase has exactly one `AskUserQuestion`
  call. Only Phase 1a takes free-text (the target directory path).
- **NO DOWNGRADE (RULE -1).** If scaffolding fails, return 2-3 concrete
  alternative paths; never "can't be done".
- **NO HALLUCINATION (RULE 0.4).** Every primitive / block / template
  referenced MUST exist on disk. Phase 1 greps before citing. Phase 2
  verifies `kei-docs-scaffold.sh` is executable before invoking.
- **Plan Mode First (RULE 0.5).** This skill IS the plan. No Edit/Write
  before the corresponding phase's confirm click.
- **Constructor Pattern (RULE ZERO).** This `SKILL.md` stays < 200 LOC.
  Each phase file < 80 LOC.
- **Surgical Changes.** Scaffold only creates files; never modifies code.
  Only touches: `CLAUDE.md`, `DECISIONS.md`, `docs/runbook.md`,
  `README.md`, `docs/diagrams/*.mmd`, `CHANGELOG.md`. Never edits source.
- **Idempotent.** Re-runs skip existing files unless `--force` is passed
  in Phase 2. No duplicate ADR numbering in Phase 3.
- **Public-publish gate.** README scaffold refuses to write if the repo is
  on the banned-public list (see `~/.claude/rules/security.md`). User
  must type "yes, deploy" + "confirm publication" to override.

---

## References

- Phases: [phase-1-intake.md](phase-1-intake.md) · [phase-2-scaffold.md](phase-2-scaffold.md) · [phase-3-decisions.md](phase-3-decisions.md) · [phase-4-diagrams.md](phase-4-diagrams.md) · [phase-5-changelog.md](phase-5-changelog.md)
- Primitive: `_primitives/kei-docs-scaffold.sh` — detector + generator (POSIX sh)
- Primitive: `_primitives/_rust/kei-changelog/` — Conventional Commit → CHANGELOG.md
- Blocks: `_blocks/docs-claude-md.md`, `_blocks/docs-decisions-adr.md`, `_blocks/docs-runbook.md`, `_blocks/docs-readme-template.md`, `_blocks/docs-architecture-diagrams.md`
- Rules: `~/.claude/rules/doc-conventions.md`, `~/.claude/rules/security.md`
