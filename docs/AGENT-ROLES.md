# Agent Roles — Human-Readable Matrix

**SSoT:** `_roles/*.toml` (5 files).
**Schema:** [AGENT-SUBSTRATE-SCHEMA.md](./AGENT-SUBSTRATE-SCHEMA.md) §Role.
**Lock marker:** [AGENT-SCHEMA-LOCKED.md](./AGENT-SCHEMA-LOCKED.md).

Five roles compose locked capability atoms from `_capabilities/<category>/<slug>/` into prompt bundles for the Agent tool. Each role is a declarative TOML; the `kei-agent-runtime compose` step concatenates the listed capability `text.md` fragments in order to produce the final `prompt.md` for an agent invocation. Tool and bash-pattern allowlists are enforced at PreToolUse by `kei-capability check`; verify-class capabilities are enforced on agent return by `kei-capability verify`.

This document is derived from the 5 role TOMLs by hand. When `kei-agent-runtime` ships (phase 3), a `kei-agent-runtime doc-roles` subcommand will regenerate this file mechanically.

---

## 1. `read-only`

**Display name:** explorer (read-only analyst)
**Spawnable:** yes
**Escalation:** `ask-via-return`

Pure inspection agent. Reads code and docs, optionally fetches a URL, emits a structured report with severity grades. No shell, no edits, no git. Cheapest and safest role.

**Capabilities bundled:**

- [`tools::deny-tools`](../_capabilities/tools/deny-tools/text.md) — denies `Edit` and `Write` entirely at PreToolUse (renamed from `tools::read-only` in v0.17; alias still resolves)
- [`output::report-format`](../_capabilities/output/report-format/text.md) — verify: parse report, assert required fields present
- [`output::severity-grade`](../_capabilities/output/severity-grade/text.md) — verify: each finding tagged with E1-E6 evidence grade

**Tools allowed:**

| Tool | Allowed | Notes |
|---|---|---|
| Read | yes | — |
| Glob | yes | — |
| Grep | yes | — |
| WebFetch | yes | external references |
| Edit | no | blocked by `tools::deny-tools` |
| Write | no | blocked by `tools::deny-tools` |
| Bash | no | not in allowlist |

**Typical use cases:**

- Code audit / critic passes (anti-pattern sweep, Constructor Pattern compliance)
- Prior-art search, docs survey, dependency research

---

## 2. `explorer`

**Display name:** explorer + cargo-check (read-only analyst with build probe)
**Spawnable:** yes
**Escalation:** `ask-via-return`

Read-only baseline plus a single permitted shell family: `cargo` invocations. Use when an audit needs `cargo check`, `cargo test`, `cargo tree`, `cargo metadata` to ground findings in actual build state — still cannot edit, still cannot git.

**Capabilities bundled:**

- [`tools::deny-tools`](../_capabilities/tools/deny-tools/text.md)
- [`tools::bash-allowlist`](../_capabilities/tools/bash-allowlist/text.md) — PreToolUse:Bash denies unless command matches one of the allowlist regexes (default: cargo/rustc/rustup/mkdir/ls/pwd/`rm -rf /tmp/`). Renamed from `tools::cargo-only-bash` in v0.17; alias still resolves.
- [`output::report-format`](../_capabilities/output/report-format/text.md)
- [`output::severity-grade`](../_capabilities/output/severity-grade/text.md)

**Tools allowed:**

| Tool | Allowed | Notes |
|---|---|---|
| Read | yes | — |
| Glob | yes | — |
| Grep | yes | — |
| WebFetch | yes | — |
| Bash | yes | only `^cargo( |$)` patterns |
| Edit | no | blocked by `tools::deny-tools` |
| Write | no | blocked by `tools::deny-tools` |

**Typical use cases:**

- Build-state-grounded audits (test counts, compile errors, workspace graph)
- Reproduction of integration-test failures without risk of edits

---

## 3. `edit-local`

**Display name:** code-implementer (local edit scope)
**Spawnable:** yes
**Escalation:** `ask-via-return`

The default code-writing role. Writes to task-whitelisted files, runs cargo-family commands, emits a required-field report. Cannot touch git, cannot bump deps, cannot edit files outside its whitelist or inside its denylist. On return, four verify-class capabilities run: Constructor Pattern limits, workspace cargo check, per-crate tests, and lock-file stability.

**Capabilities bundled:**

- [`policy::no-git-ops`](../_capabilities/policy/no-git-ops/text.md) — blocks `git`, `gh repo`, `gh api /repos` at PreToolUse:Bash
- [`scope::files-whitelist`](../_capabilities/scope/files-whitelist/text.md) — PreToolUse:Edit|Write denies paths outside whitelist; on-return git diff check
- [`scope::files-denylist`](../_capabilities/scope/files-denylist/text.md) — denies paths in denylist (overrides whitelist)
- [`quality::constructor-pattern`](../_capabilities/quality/constructor-pattern/text.md) — verify: no file > 200 LOC, no fn > 30 LOC
- [`quality::cargo-check-green`](../_capabilities/quality/cargo-check-green/text.md) — verify: `cargo check --workspace` from MAIN passes (simulated-merge)
- [`quality::tests-green`](../_capabilities/quality/tests-green/text.md) — verify: `cargo test -p <crate>` passes, count ≥ task min
- [`safety::no-dep-bump`](../_capabilities/safety/no-dep-bump/text.md) — PreToolUse:Edit on Cargo.toml denies unless task opts in
- [`output::report-format`](../_capabilities/output/report-format/text.md)

**Tools allowed:**

| Tool | Allowed | Notes |
|---|---|---|
| Read | yes | — |
| Write | yes | scope-gated |
| Edit | yes | scope-gated |
| Glob | yes | — |
| Grep | yes | — |
| Bash | yes | patterns: `^cargo( |$)`, `^mkdir( |$)`, `^rm -rf /tmp/` |

**Typical use cases:**

- Implement a new crate / feature within a bounded file whitelist
- Refactor inside one crate with dep-lock stability guaranteed

---

## 4. `edit-shared`

**Display name:** code-implementer (shared-SSoT edit scope)
**Spawnable:** yes
**Escalation:** `orchestrator-notify`

Same capabilities as `edit-local`. The difference is **operational, not declarative**: for `edit-shared`, the orchestrator parameterizes `scope::files-whitelist` in `task.toml` to include one or more SSoT paths (e.g. workspace `Cargo.toml`, a registry file, a cross-crate type definition). Escalation is tightened from `ask-via-return` to `orchestrator-notify` so SSoT edits surface immediately.

**Capabilities bundled:** identical to `edit-local` — the SSoT relaxation rides on per-task `scope::files-whitelist` parameterization, not on a different capability set.

**Tools allowed:** identical to `edit-local`.

**Typical use cases:**

- Workspace-level edits (add a member crate, bump a shared version)
- Cross-crate API changes where one type SSoT must be edited alongside its consumers

**Difference from `edit-local` at a glance:**

| Dimension | `edit-local` | `edit-shared` |
|---|---|---|
| Capability set | identical | identical |
| Whitelist (task.toml) | local crate paths only | local crate + one SSoT path |
| Escalation | `ask-via-return` | `orchestrator-notify` |
| Typical use | inside one crate | crosses a crate or workspace boundary |

---

## 5. `git-ops` — NON-SPAWNABLE (orchestrator-only)

**Display name:** git operator (orchestrator-only, NOT spawnable)
**Spawnable:** **no**
**Escalation:** `fail-fast` (not reachable at runtime)

Documented boundary of git authority — not a live role. Present in the inventory so that "who can run git" has an explicit declarative answer.

Per [RULE 0.13 — ORCHESTRATOR BRANCH FIRST](../../.claude/rules/orchestrator-branch-first.md), only the orchestrator (main session) holds git power:

- branch creation (`git checkout -b …`)
- commit (`git add <paths> && git commit -m …`)
- push (`git push <remote> <branch>`)
- merge (`git merge --no-ff`, `git merge --squash`)
- rebase, reset, tag

Agents running inside `.claude/worktrees/<agent>/` cannot invoke `git` — the sandbox denies Bash inside the worktree path. This role exists to make that boundary visible in the capability substrate, not to enable spawns.

`kei-agent-runtime spawn` MUST refuse any `task.toml` whose `[task].role = "git-ops"` with a pointer to RULE 0.13. The refusal is a hard error, not a warning.

**Rationale (why documented at all):**

- Future contributors see "git is a role" and "that role is unreachable" in the same place
- `kei-agent-runtime doc-roles` regeneration will surface the non-spawnable notice so it never goes stale
- Matches how `_roles/git-ops.toml` holds `spawnable = false` explicitly — declarative, greppable

---

## Cross-role capability matrix

Capabilities as rows, roles as columns. A ✓ means the role lists the capability in `[capabilities].required`; ✗ means it does not.

| Capability | `read-only` | `explorer` | `edit-local` | `edit-shared` | `git-ops` |
|---|:-:|:-:|:-:|:-:|:-:|
| `policy::no-git-ops` | ✗ | ✗ | ✓ | ✓ | ✗ |
| `scope::files-whitelist` | ✗ | ✗ | ✓ | ✓ | ✗ |
| `scope::files-denylist` | ✗ | ✗ | ✓ | ✓ | ✗ |
| `quality::constructor-pattern` | ✗ | ✗ | ✓ | ✓ | ✗ |
| `quality::cargo-check-green` | ✗ | ✗ | ✓ | ✓ | ✗ |
| `quality::tests-green` | ✗ | ✗ | ✓ | ✓ | ✗ |
| `safety::no-dep-bump` | ✗ | ✗ | ✓ | ✓ | ✗ |
| `output::report-format` | ✓ | ✓ | ✓ | ✓ | ✗ |
| `output::severity-grade` | ✓ | ✓ | ✗ | ✗ | ✗ |
| `tools::deny-tools` | ✓ | ✓ | ✗ | ✗ | ✗ |
| `tools::bash-allowlist` | ✗ | ✓ | ✗ (¹) | ✗ (¹) | ✗ |

(¹) `edit-local` and `edit-shared` do not compose `tools::bash-allowlist` as a capability atom; instead they carry an inline `bash-patterns-allowed` list in `[tools]` that encodes the same restriction. Both routes converge at the PreToolUse:Bash gate. Phase 3 runtime may later collapse the inline list into a parameterized `tools::bash-allowlist` atom — non-breaking.

(v0.17 rename: `tools::read-only` → `tools::deny-tools`; `tools::cargo-only-bash` → `tools::bash-allowlist`. Old names still resolve via registry alias with a one-shot stderr deprecation warning.)

## Tool allowlist matrix

| Tool | `read-only` | `explorer` | `edit-local` | `edit-shared` | `git-ops` (²) |
|---|:-:|:-:|:-:|:-:|:-:|
| Read | ✓ | ✓ | ✓ | ✓ | ✓ |
| Glob | ✓ | ✓ | ✓ | ✓ | ✓ |
| Grep | ✓ | ✓ | ✓ | ✓ | ✓ |
| WebFetch | ✓ | ✓ | ✗ | ✗ | ✓ |
| Edit | ✗ | ✗ | ✓ | ✓ | ✓ |
| Write | ✗ | ✗ | ✓ | ✓ | ✓ |
| Bash | ✗ | cargo-only | cargo+mkdir+tmp | cargo+mkdir+tmp | any |

(²) `git-ops` values are documentation only — the role is non-spawnable.

## Escalation policy matrix

| Role | Policy | Meaning |
|---|---|---|
| `read-only` | `ask-via-return` | Surface questions in the final report; orchestrator reads them |
| `explorer` | `ask-via-return` | Same |
| `edit-local` | `ask-via-return` | Same |
| `edit-shared` | `orchestrator-notify` | Touching SSoT ⇒ notify orchestrator before completing |
| `git-ops` | `fail-fast` | Unreachable; any spawn attempt errors |

---

## Agent role assignments (migrated to v0.16 substrate)

Twelve of the kit-shipped agents carry `substrate_role = "..."` in their `_manifests/<name>.toml`. The assembler reads the role, pulls the listed capability fragments from `_capabilities/<cat>/<slug>/text.md`, and injects them into the generated agent `.md` under `# AGENT SUBSTRATE — role <name>`.

| Role | Agent | Notes |
|---|---|---|
| `read-only` | `kei-architect` | structural review, no edits |
| `read-only` | `kei-critic` | severity-graded findings |
| `read-only` | `kei-security-auditor` | risk/differential/variant/supply-chain sweeps |
| `read-only` | `kei-validator` | citation / no-hallucination gate |
| `read-only` | `kei-cost-guardian` | GO/NO-GO compute-cost report card |
| `read-only` | `kei-ml-researcher` | literature + tooling-reuse audit |
| `read-only` | `kei-researcher` | generic web/code research, E1-E6 graded |
| `edit-local` | `kei-code-implementer` | Rust-first production code + tests |
| `edit-local` | `kei-infra-implementer` | deploy/CI/CD/IaC with secrets hygiene |
| `edit-local` | `kei-ml-implementer` | training/inference code + Modal jobs |
| `edit-local` | `kei-modal-runner` | Modal compute orchestration, anti-stop guard |
| `edit-local` | `kei-fal-ai-runner` | fal.ai asset generation |

Unassigned agents (no substrate role yet): `edit-shared` and `git-ops` are role slots only — no kit-shipped agent currently binds to them. `edit-shared` is reached by parameterizing an `edit-local` task's `scope::files-whitelist` to include an SSoT path; `git-ops` is orchestrator-only per RULE 0.13 and non-spawnable.

## Maintenance

- Changes to any `_roles/*.toml` require updating this file in the same commit.
- Changes to `substrate_role` on any `_manifests/<name>.toml` require updating the "Agent role assignments" table in the same commit.
- New roles are added as new sections 6+ with the same structure, and new columns added to the two matrices above.
- When `kei-agent-runtime doc-roles` ships in phase 3, it replaces the hand-authored matrix; the top-of-file "derived by hand" note is removed then.
