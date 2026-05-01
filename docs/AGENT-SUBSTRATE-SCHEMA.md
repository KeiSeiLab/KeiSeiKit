# KeiSeiKit Agent Substrate Schema v1

**STATUS:** Decisions resolved 2026-04-23 — see updated Decision log at bottom. LOCK active upon `AGENT-SCHEMA-LOCKED.md` commit. 3-week parallel phase window.

**PURPOSE:** Sibling SSoT to `SUBSTRATE-SCHEMA.md`. That one decomposes code primitives (atoms). This one decomposes **agent invocations** (capabilities).

**Motivation from substrate v1 orchestration pain:** across 7 agent spawns in audit+follow-up waves, the same friction recurred — 40% prompt boilerplate, self-reported green tests that broke at integration, scope violations surfacing only after merge. Fix: capabilities become **enforced triplets**, not suggestions in freetext prompts.

---

## Core concept: capability atom = triplet

An **agent capability** is not a reusable text block. It is a **declarative bundle + Rust implementation** that gives every restriction meaning across three layers:

| Artifact | Format | Who consumes |
|---|---|---|
| `capability.toml` | TOML declarative metadata (name, category, patterns, parameters) | kei-agent-runtime at compose + lint time |
| `text.md` | Markdown prompt fragment | Agent (via LLM context) |
| Rust module `gates/<slug>.rs` | Rust `impl Capability` trait | `kei-capability check` binary at PreToolUse |
| Rust module `verifies/<slug>.rs` | Rust `impl Capability` trait | `kei-capability verify` binary at on-return |

The two Rust modules live in `_primitives/_rust/kei-agent-runtime/src/` — one compilation unit, one registry, `cargo test` on all gates/verifies at once. Shell hooks are 3-line glue that `exec`s the binary.

**The invariant:** if any of the four artifacts is missing or fails, the capability did not hold. Self-reported compliance is not trusted — verification runs via **worktree short-circuit → simulated merge** pattern (see §Verify execution below) after agent return, catching integration regressions before merge to main.

---

## File layout

```
_capabilities/                        — DECLARATIVE artefacts (phase 1 writes these)
├── policy/
│   └── no-git-ops/
│       ├── capability.toml
│       └── text.md
├── scope/
│   ├── files-whitelist/{capability.toml, text.md}
│   └── files-denylist/{capability.toml, text.md}
├── quality/
│   ├── constructor-pattern/{capability.toml, text.md}
│   ├── cargo-check-green/{capability.toml, text.md}
│   └── tests-green/{capability.toml, text.md}
├── safety/
│   └── no-dep-bump/{capability.toml, text.md}
├── output/
│   ├── report-format/{capability.toml, text.md}
│   └── severity-grade/{capability.toml, text.md}
└── tools/
    ├── read-only/{capability.toml, text.md}
    └── cargo-only-bash/{capability.toml, text.md}

_roles/                               — DECLARATIVE (phase 2 writes these)
├── read-only.toml
├── explorer.toml
├── edit-local.toml
├── edit-shared.toml
└── git-ops.toml                      — documented; NOT spawnable (orchestrator-only)

_primitives/_rust/kei-agent-runtime/  — BINARY (phase 3 writes this)
├── Cargo.toml
├── src/
│   ├── lib.rs                        — exports Capability trait + registry
│   ├── main.rs                       — CLI: compose | spawn | verify | run
│   ├── compose.rs                    — task.toml + role + capabilities → prompt.md
│   ├── spawn.rs                      — Agent-tool invocation with composed prompt
│   ├── verify.rs                     — worktree short-circuit → simulated merge
│   ├── simulated_merge.rs            — create temp branch + apply diff + run checks
│   ├── registry.rs                   — &str → Box<dyn Capability> dispatch
│   ├── gates/                        — PreToolUse logic
│   │   ├── mod.rs
│   │   ├── policy_no_git_ops.rs
│   │   ├── scope_files_whitelist.rs
│   │   ├── scope_files_denylist.rs
│   │   ├── safety_no_dep_bump.rs
│   │   ├── tools_read_only.rs
│   │   └── tools_cargo_only_bash.rs  — 6 gates
│   └── verifies/                     — on-return logic
│       ├── mod.rs
│       ├── quality_constructor_pattern.rs
│       ├── quality_cargo_check_green.rs
│       ├── quality_tests_green.rs
│       ├── safety_no_dep_bump.rs
│       ├── scope_files_whitelist.rs
│       ├── scope_files_denylist.rs
│       ├── output_report_format.rs
│       └── output_severity_grade.rs   — 8 verifies
└── tests/

_primitives/_rust/kei-capability/     — BINARY (phase 3)
├── Cargo.toml                        — depends on kei-agent-runtime
└── src/main.rs                       — clap CLI:
                                         kei-capability check <name>   (stdin JSON, exit 0|2)
                                         kei-capability verify <name>  (env-driven, exit 0 or fail)

hooks/                                — 3-line shell glue (phase 4 ✓ shipped)
├── agent-capability-check.sh         — `exec kei-capability check "$KEI_CAPABILITY_NAME"` — PreToolUse:Bash|Edit|Write, no-op when env unset, fail-open on missing binary
└── agent-capability-verify.sh        — orchestrator-driven post-agent: `exec kei-capability verify "$KEI_CAPABILITY_NAME"` with AGENT_ID/TASK_TOML/WORKTREE_PATH/MAIN_REPO/RUN_MODE env

tasks/                                — ephemeral, gitignored
└── <agent-id>/{task.toml, prompt.md}

docs/AGENT-SUBSTRATE-SCHEMA.md       — this file
docs/AGENT-ROLES.md                  — human-readable role matrix (generated from _roles/*.toml)
docs/AGENT-SCHEMA-LOCKED.md          — lock marker
```

---

## Capability atom — `capability.toml` shape

```toml
[capability]
name = "policy::no-git-ops"           # <category>::<slug> namespace
category = "policy"                   # policy | scope | quality | safety | output | tools
version = "1.0"
description = "RULE 0.13 — orchestrator owns git, agent writes files only"
rationale = "See ~/.claude/rules/orchestrator-branch-first.md"

[restricts]
# What this capability forbids. Runtime gate enforces.
tool-patterns = [                     # matched against tool_input.command
  '^git( |$)',
  '^gh (repo|api /repos)',
]
tools-denied = []                     # e.g. ["Edit", "Write"] for read-only

[parameterized]
# Is this capability instance-configurable per task?
accepts = []                          # e.g. ["files-whitelist"] for scope/* caps

[text]
path = "text.md"                      # relative to capability dir

[gate]
# Rust module path inside kei-agent-runtime — registry dispatches by capability.name
rust-module = "gates::policy_no_git_ops"   # or empty if capability has no gate (verify-only)
event = "PreToolUse:Bash"              # PreToolUse:Bash | PreToolUse:Edit|Write | PreToolUse:Agent
severity = "block"                     # block (exit 2) | warn (exit 0 + stderr) | advisory (log only)
bypass-env = "ORCHESTRATOR_META"       # optional env var to disable

[verify]
rust-module = "verifies::policy_no_git_ops"  # or empty if gate-only
run-mode = "simulated-merge"           # worktree | simulated-merge | both
when = "on-return"                     # on-return | per-tool-call
```

**`run-mode` values:**
- `worktree` — run predicate inside the agent's worktree (fastest; what the agent saw)
- `simulated-merge` — orchestrator creates `test-merge/<agent-id>` branch off main, applies agent diff, runs predicate from there (catches integration regressions of the E1-jsonschema-class — see §Verify execution)
- `both` — worktree first (fail-fast), then simulated-merge (integration guarantee). Default for `quality::*` capabilities.

---

## Capability `text.md` conventions

- Imperative, second-person, short.
- ≤ 200 words per fragment.
- No overlap — if two capabilities say the same thing, extract into a shared one.
- Fragment stands alone — composer concatenates multiple fragments with `\n\n---\n\n` separator; fragments must not reference each other.
- Lead with the rule ("You MUST NOT X"), follow with the why ("because Y").

Example (`_capabilities/policy/no-git-ops/text.md`):

```markdown
## No git operations

You MUST NOT invoke `git`, `gh repo`, `gh api /repos`, or any shell
command that modifies git state. Orchestrator handles all git operations
(commits, branches, pushes, rebases).

If your task requires staging a change, describe it in the return
file-list — the orchestrator will commit on your behalf.

Bypass exists for orchestrator-meta agents only; it is not available here.
```

---

## Capability trait contract (Rust)

All gates and verifies implement the same trait, dispatched by string name. Registry in `kei-agent-runtime/src/registry.rs` maps `"policy::no-git-ops"` to `Box<dyn Capability>`.

```rust
// kei-agent-runtime/src/capability.rs

pub trait Capability: Send + Sync {
    fn name(&self) -> &'static str;

    /// PreToolUse gate. Called by `kei-capability check <name>` binary.
    /// Receives the hook JSON payload from Claude Code on stdin.
    /// Returns Allow / Deny{reason} / NotApplicable.
    fn check(&self, ctx: &GateContext) -> GateDecision {
        GateDecision::NotApplicable  // default: no gate, verify-only
    }

    /// On-return verification predicate. Called by `kei-capability verify <name>`.
    /// Receives task context (agent-id, worktree path, main repo, task.toml values).
    /// Returns Pass / Fail{reason}.
    fn verify(&self, ctx: &VerifyContext) -> VerifyResult {
        VerifyResult::Pass  // default: no verify, gate-only
    }
}

pub struct GateContext<'a> {
    pub tool_name: &'a str,
    pub tool_input: &'a Value,
    pub task: &'a TaskSpec,          // parsed task.toml
    pub env: &'a HashMap<String, String>,
}

pub enum GateDecision {
    Allow,
    Deny { reason: String },
    NotApplicable,
}

pub struct VerifyContext<'a> {
    pub agent_id: &'a str,
    pub task: &'a TaskSpec,
    pub worktree_path: &'a Path,
    pub main_repo: &'a Path,
    pub run_mode: RunMode,           // Worktree | SimulatedMerge | Both
}

pub enum VerifyResult {
    Pass,
    Fail { reason: String, detail: Option<String> },
}
```

Example implementation (`_primitives/_rust/kei-agent-runtime/src/gates/policy_no_git_ops.rs`):

```rust
use crate::capability::*;
use regex::Regex;
use once_cell::sync::Lazy;

pub struct NoGitOps;

static GIT_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| vec![
    Regex::new(r"(?m)(?:^|[;&|]|\s)git(?:\s|$)").unwrap(),
    Regex::new(r"(?m)(?:^|[;&|]|\s)gh\s+repo").unwrap(),
    Regex::new(r"(?m)(?:^|[;&|]|\s)gh\s+api\s+/?repos").unwrap(),
]);

impl Capability for NoGitOps {
    fn name(&self) -> &'static str { "policy::no-git-ops" }

    fn check(&self, ctx: &GateContext) -> GateDecision {
        if ctx.tool_name != "Bash" { return GateDecision::NotApplicable; }
        if ctx.env.get("ORCHESTRATOR_META").map(|v| v == "1").unwrap_or(false) {
            return GateDecision::Allow;
        }
        let cmd = ctx.tool_input.get("command").and_then(|v| v.as_str()).unwrap_or("");
        for pat in GIT_PATTERNS.iter() {
            if pat.is_match(cmd) {
                return GateDecision::Deny {
                    reason: format!("RULE 0.13 — git operation blocked (pattern {})", pat.as_str()),
                };
            }
        }
        GateDecision::Allow
    }
}
```

Example verify (`_primitives/_rust/kei-agent-runtime/src/verifies/quality_cargo_check_green.rs`):

```rust
use crate::capability::*;
use std::process::Command;

pub struct CargoCheckGreen;

impl Capability for CargoCheckGreen {
    fn name(&self) -> &'static str { "quality::cargo-check-green" }

    fn verify(&self, ctx: &VerifyContext) -> VerifyResult {
        let run_dir = match ctx.run_mode {
            RunMode::Worktree => ctx.worktree_path,
            RunMode::SimulatedMerge => &ctx.simulated_merge_path(),
            RunMode::Both => unreachable!("runtime runs `both` as two sequential calls"),
        };
        let out = Command::new("cargo")
            .arg("check")
            .arg("--workspace")
            .current_dir(run_dir.join("_primitives/_rust"))
            .output();
        match out {
            Err(e) => VerifyResult::Fail {
                reason: "cargo invocation failed".to_string(),
                detail: Some(e.to_string()),
            },
            Ok(o) if !o.status.success() => {
                let tail = String::from_utf8_lossy(&o.stderr).lines().rev().take(5).collect::<Vec<_>>();
                VerifyResult::Fail {
                    reason: "cargo check --workspace FAILED — agent-local green ≠ integration green".to_string(),
                    detail: Some(tail.into_iter().rev().collect::<Vec<_>>().join("\n")),
                }
            }
            Ok(_) => VerifyResult::Pass,
        }
    }
}
```

## Verify execution — worktree → simulated merge

The orchestrator runs verification in **two sequential passes** for `run-mode = "both"`:

```
Pass 1 — worktree (fail-fast)
  cd <agent-worktree>
  run capability.verify(RunMode::Worktree)
  if Fail → reject immediately, don't bother with pass 2

Pass 2 — simulated-merge (integration guarantee)
  git checkout -b test-merge/<agent-id> main   # from MAIN repo, not worktree
  git apply <agent-diff>                        # apply agent's changes on clean main
  cd <test-merge branch>
  run capability.verify(RunMode::SimulatedMerge)
  if Fail → reject with regression report
  if Pass → safe to merge, orchestrator proceeds
```

Why both: agent's worktree passing doesn't mean merged-main passing. E1's jsonschema regression was green in worktree (no real atoms there) but broke main integration (real atom schemas triggered the 0.17→0.18 breaking change). Simulated merge catches this class **before** it lands on main.

Implementation lives in `kei-agent-runtime/src/simulated_merge.rs` — creates a temp worktree via `git worktree add`, applies diff, runs verify, cleans up.

---

## Role — `_roles/<name>.toml` shape

```toml
[role]
name = "edit-local"
display-name = "code-implementer (local edit scope)"
description = "Write code + run cargo check/test + emit report. No git, no workspace touches."

[capabilities]
# Ordered list — text.md fragments concatenated in this order
required = [
  "policy::no-git-ops",
  "scope::files-whitelist",
  "scope::files-denylist",
  "quality::constructor-pattern",
  "quality::cargo-check-green",
  "quality::tests-green",
  "safety::no-dep-bump",
  "output::report-format",
]

[tools]
# Tool allowlist — anything not in this list is denied
allowed = ["Read", "Write", "Edit", "Glob", "Grep", "Bash"]
# Bash further restricted by quality/tools atoms
bash-patterns-allowed = ['^cargo( |$)', '^mkdir( |$)', '^rm -rf /tmp/']

[escalation]
policy = "ask-via-return"              # ask-via-return | orchestrator-notify | fail-fast
```

---

## Task spec — `task.toml` shape (orchestrator writes per spawn)

```toml
[task]
role = "edit-local"
agent-id = "abc123…"                  # allocated by kei-ledger fork
parent-agent = null                    # or parent ID for nested

[scope]
# Parameterizes scope::files-whitelist + scope::files-denylist
files-whitelist = [
  "_primitives/_rust/kei-forge/**",
]
files-denylist = [
  "_primitives/_rust/Cargo.toml",
  "_primitives/_rust/Cargo.lock",
  "scripts/**",
  ".github/**",
]

[verification]
# Parameterizes quality/* caps
cargo-check-crates = ["kei-forge"]
cargo-test-crates = ["kei-forge"]
test-count-min = 44

[output]
# Parameterizes output/report-format
report-fields-required = ["files-touched", "cargo-check", "cargo-test", "loc-delta"]

[body]
# Free-text task instructions, concatenated AFTER role capability fragments
text = """
Replace shell-out with pure-Rust templating. …
"""
```

---

## Runtime execution contract

`kei-agent-runtime` crate provides:

```bash
# Compose prompt from task spec
kei-agent-runtime compose <task.toml>
# → writes <task-dir>/prompt.md

# Spawn agent with composed prompt + install gates + record ledger
kei-agent-runtime spawn <task.toml>
# → returns agent-id; background-task notification semantics

# Run all capability verify predicates against agent's return
kei-agent-runtime verify <task.toml> <worktree-path>
# → exit 0 if all hold, non-zero with report of violations

# One-shot helper: compose + spawn + verify
kei-agent-runtime run <task.toml>
```

Execution flow:

```
1. orchestrator writes task.toml
2. `kei-agent-runtime compose` → prompt.md
3. `kei-agent-runtime spawn` →
     a. kei-ledger fork <agent-id>
     b. install PreToolUse gates parameterized by task.scope
     c. Agent tool call with isolation=worktree + composed prompt
4. [agent executes]
5. `kei-agent-runtime verify` →
     a. run each capability verify.sh from MAIN repo (not worktree)
     b. collect all violations
     c. exit 0 if empty, non-zero with report
6. orchestrator decides: merge | reject + respawn | reject + rollback
```

---

## Initial capability atom inventory (phase 1 builds these 10)

| Name | Category | text / gate / verify | Core restriction |
|---|---|---|---|
| `policy::no-git-ops` | policy | ✓/✓/✓ | Block `git`, `gh repo`, `gh api /repos` |
| `scope::files-whitelist` | scope | ✓/✓/✓ | PreToolUse:Edit\|Write denies paths outside whitelist; on-return git diff check |
| `scope::files-denylist` | scope | ✓/✓/✓ | PreToolUse:Edit\|Write denies paths in denylist (overrides whitelist) |
| `quality::constructor-pattern` | quality | ✓/—/✓ | On return: no file > 200 LOC, no fn > 30 LOC |
| `quality::cargo-check-green` | quality | ✓/—/✓ | On return: `cargo check --workspace` from MAIN passes |
| `quality::tests-green` | quality | ✓/—/✓ | On return: `cargo test -p <crate>` passes, count ≥ task min |
| `safety::no-dep-bump` | safety | ✓/✓/✓ | PreToolUse:Edit on Cargo.toml denies unless task opts in; on-return lock-diff check |
| `output::report-format` | output | ✓/—/✓ | On return: parse report, assert required fields present |
| `tools::deny-tools` | tools | ✓/✓/— | PreToolUse denies Edit/Write entirely (renamed from `tools::read-only` in v0.17; old name resolves via alias) |
| `tools::bash-allowlist` | tools | ✓/✓/— | PreToolUse:Bash denies unless command matches allowlist pattern (renamed from `tools::cargo-only-bash` in v0.17; old name resolves via alias) |

---

## Initial role inventory (phase 2 builds these 5)

| Role | Capabilities | Tools |
|---|---|---|
| `read-only` | tools::deny-tools + output::report-format + output::severity-grade | Read / Glob / Grep / WebFetch |
| `explorer` | read-only caps + tools::bash-allowlist (for `cargo check`) | + Bash-cargo |
| `edit-local` | policy::no-git-ops + scope::* + quality::* + safety::no-dep-bump + output::report-format | + Edit / Write / Bash-cargo |
| `edit-shared` | edit-local caps + permission for specified SSoT patterns | Same + SSoT paths |
| `git-ops` | Documented-only, NOT spawnable (orchestrator holds this) | All |

---

## Decision log — resolved 2026-04-23

| # | Question | Decision | Rationale |
|---|---|---|---|
| 1 | Layout per capability | **Declarative bundle (`capability.toml` + `text.md`) + Rust modules in runtime crate** | Declarative artefacts live with capability; executable logic lives with its sibling capabilities in one Rust crate for shared tests + type safety |
| 2 | Gate language | **Rust** via `kei-capability check <name>` binary; shell hook = 3-line `exec` glue | Type safety, unit tests, one compilation unit for all gates. Shell remains only as Claude-Code-hook-protocol adapter |
| 3 | Verify language | **Rust** same binary, `kei-capability verify <name>` subcommand | Same reasoning. Cargo output parsing, LOC checks, diff analysis — all better in Rust |
| 4 | Config format (capability.toml / role.toml / task.toml) | **TOML** | Consistent with Cargo ecosystem. YAML reserved only for locked atom `.md` frontmatter (immutable under atom substrate v1 lock) |
| 5 | Capability ID separator | **`::`** | Consistent with atom IDs. Rust-native |
| 6 | Capability path layout | **Nested `_capabilities/<category>/<slug>/`** | Scales to 50+ capabilities, category browsability |
| 7 | Text fragment max | **200 words per capability** | Agent context budget; forces atomicity |
| 8 | Verify execution | **worktree short-circuit → simulated-merge** (default `both` for `quality::*`) | Catches E1-jsonschema-class integration regressions before main merge. See §Verify execution |

**Locked values:** all 8 above. Breaking changes require explicit user revocation + all-phases sync.

---

## Phase plan (post-lock, parallel)

| Phase | What | Depends on | Agent | Estimate |
|---|---|---|---|---|
| 0 | This schema + lock marker | — | me | 0.5 day ✓ |
| 1 | Capability library — 10 × (`capability.toml` + `text.md`) = **20 declarative files** | phase 0 | 1 code-implementer | 1-2 days |
| 2 | Role matrix — 5 `_roles/*.toml` + auto-gen `docs/AGENT-ROLES.md` | phase 0 | 1 code-implementer | 0.5 day |
| 3 | `kei-agent-runtime` + `kei-capability` binaries — compose/spawn/verify CLI + 6 gate modules + 8 verify modules + registry + simulated-merge executor | phase 0 | 1 code-implementer | 5-6 days |
| 4 ✓ | Hook wiring — `agent-capability-check.sh` + `agent-capability-verify.sh` 3-line glue + settings.json registration | phases 1+3 | 1 code-implementer | 0.5 day (shipped) |
| 5 ✓ | Migration — 5 kit-shipped agents (code-implementer / critic / architect / security-auditor / validator) adopt role+task-spec invocation via new `substrate_role` manifest field | phases 1+2+3+4 | 1 code-implementer | 1 day (shipped) |

**Phases 1, 2, 3 start in parallel immediately after lock** (different dirs, zero file overlap).
Phase 4 depends on 1+3.
Phase 5 depends on everything.

Total wall-time with parallel phases 1+2+3: **~7-8 days from lock** (phase 3 is critical path).

---

## Integration with substrate v1

This schema is **additive** to locked `SUBSTRATE-SCHEMA.md`. The two SSoTs sit side by side:

- `SUBSTRATE-SCHEMA.md` — how code decomposes into atoms (locked 2026-04-22)
- `AGENT-SUBSTRATE-SCHEMA.md` — how agent invocation decomposes into capabilities (this doc)

Cross-ref: agent capability `quality::cargo-check-green` verifies that atoms compiled; atom agents produced via `kei-forge` can themselves be invoked through `kei-runtime` (atom substrate) OR composed into role definitions (agent substrate).

Eventually (post-both-locks): **agents compose atoms, atoms compose agents**. Symmetric substrates.

---

## Lock declaration

Once this document is approved by the user and `docs/AGENT-SCHEMA-LOCKED.md` is committed, the capability-triplet shape + role shape + task-spec shape + runtime contract are **immutable for 3 weeks** (shorter lock than atom substrate because agent substrate is greenfield, expected revisions).

Breaking changes during lock require:
1. Explicit revocation by user
2. All parallel phase agents paused
3. Lock marker amended with revocation reason
4. `kei-ledger` row: bypass reason + revocation timestamp

Non-breaking additions (new capability atoms beyond the initial 10, new roles, new parameterized fields on existing capabilities) are allowed during lock.

---

## Migrated agents

Phase 5 wired the 5 kit-shipped agents to role+task-spec invocation via a new `substrate_role` field on the manifest. The assembler reads the declared role, expands each of its capability `text.md` fragments, and emits them under a `# AGENT SUBSTRATE — role <name>` section placed immediately after `# ROLE` and before the first behavioural block.

| Agent manifest | Role | Capabilities expanded |
|---|---|---|
| `_manifests/kei-code-implementer.toml` | `edit-local` | `policy::no-git-ops`, `scope::files-whitelist`, `scope::files-denylist`, `quality::constructor-pattern`, `quality::cargo-check-green`, `quality::tests-green`, `safety::no-dep-bump`, `output::report-format` |
| `_manifests/kei-critic.toml` | `read-only` | `tools::deny-tools`, `output::report-format`, `output::severity-grade` |
| `_manifests/kei-architect.toml` | `read-only` | `tools::deny-tools`, `output::report-format`, `output::severity-grade` |
| `_manifests/kei-security-auditor.toml` | `read-only` | `tools::deny-tools`, `output::report-format`, `output::severity-grade` |
| `_manifests/kei-validator.toml` | `read-only` | `tools::deny-tools`, `output::report-format`, `output::severity-grade` |

Backward compatibility: the `substrate_role` field is optional. The 7 non-migrated kit agents (`kei-cost-guardian`, `kei-fal-ai-runner`, `kei-infra-implementer`, `kei-ml-implementer`, `kei-ml-researcher`, `kei-modal-runner`, `kei-researcher`) continue to assemble without change; a deferred v0.24 migration wave will promote them. Task-spec examples showing how the orchestrator invokes each migrated agent live under `_templates/task-examples/`.

## Orchestrator ergonomics — `prepare` command

`compose` emits a prompt, `spawn` writes `tasks/<id>/` on disk, `verify` runs on return. Between compose and spawn, the orchestrator needs to invoke Claude Code's `Agent` tool — which lives inside Claude Code, not in Rust. `kei-agent-runtime prepare` bridges that step: it parses a `task.toml` and emits every argument the Agent-tool call needs in one copy-paste-ready block.

```
kei-agent-runtime prepare <task.toml> [--kit-root .] [--format human|json|toml]
```

Human output:

```
=== AGENT SUBSTRATE v1 — PREPARED SPAWN ===
agent-id: <id>
subagent_type: <role-derived>
isolation: worktree
description: <role> agent <short>

--- PROMPT (copy into Agent tool `prompt` param) ---
<composed prompt content>
--- END PROMPT ---

on return:
  kei-agent-runtime verify tasks/<id>/task.toml --worktree <path-from-harness>
  (orchestrator harness returns worktree path in the task-notification)

ledger: running agent-id=<id> role=<role> parent=<parent-or-none>
```

`--format=json` and `--format=toml` emit the same `AgentInvocation` struct for scriptable wrappers (e.g. future `/spawn-agent` Claude Code skill).

### Role → Claude subagent_type mapping

Claude Code's `Agent` tool takes a `subagent_type` string. Roles map to subagent_type via an optional `claude-subagent-type` field on `[role]` in `_roles/<name>.toml`. If unset, the runtime falls back to defaults:

| Role | Default `claude-subagent-type` |
|---|---|
| `edit-local`  | `code-implementer` |
| `edit-shared` | `code-implementer` |
| `explorer`    | `Explore` |
| `read-only`   | `critic` (override per-task for architect-flavour reviews) |
| `git-ops`     | `NOT-SPAWNABLE` (never composed — `spawnable = false`) |

`isolation = "worktree"` is auto-set for `edit-local` and `edit-shared`; other roles default to no isolation.

### Non-spawnable refusal

`prepare` refuses roles with `spawnable = false` and cites RULE 0.13 in the error. `git-ops` is the only shipped example; the refusal keeps "who can do git" boundary visible both in the role manifest AND at invocation time.

### Contract

`prepare` does NOT write to disk (inspection helper) and does NOT touch the ledger DB (the "ledger row" field is a pretty-printed string for the orchestrator to verify before calling `kei-ledger fork`). `spawn` remains the disk-writing step; `prepare` is additive and read-only.

### `kei-capability fork` — clone a capability

`kei-capability fork <source> --as <new-name> [--kit-root <dir>]` copies an existing `_capabilities/<src-cat>/<src-slug>/` directory under a new `<cat>::<slug>` name and records lineage so downstream tooling can trace the fork back to its parent.

```
kei-capability fork policy::no-git-ops --as policy::no-git-ops-lax
```

Behaviour:

1. Both `<source>` and `<new-name>` must parse as `<cat>::<slug>` with each half matching the shared slug regex (`^[a-z][a-z0-9-]{0,63}$`); upper-case or path-traversal input is rejected before any filesystem write.
2. Target directory `_capabilities/<new-cat>/<new-slug>/` must NOT exist — fork refuses to clobber.
3. `capability.toml` is parsed, rewritten with `[capability].name = "<new-name>"` (and `category` set to `<new-cat>`), then augmented with a new `[lineage]` table:

   ```toml
   [lineage]
   fork_from = "<source-name>"
   parents   = ["<source-name>"]
   creator   = "<env KEI_CREATOR_ID or 'unknown'>"
   created   = "<ISO-8601 UTC at fork time>"
   ```

4. `text.md` is copied byte-identical — the operator is expected to edit it afterwards to reflect the fork's new semantics.
5. On success the CLI prints source→target, the new directory, the number of fields rewritten, and a next-steps hint reminding the operator to edit `text.md` and ensure `[gate].rust-module` / `[verify].rust-module` match the new slug.

Fork is local-only; no ledger row is written. It is an ergonomic shortcut for authoring a derived capability; the resulting files are still subject to the normal review + merge workflow.

## Deferred extension candidates (non-breaking post-lock)

Capability atoms NOT in the initial 10 but good follow-up PRs (non-breaking additions during lock window):

- `safety::no-mass-delete` — PreToolUse denies `rm -rf` on more than N files
- `output::ledger-row-required` — verify agent emitted ledger row per RULE 0.12
- `quality::no-warnings` — `cargo build --workspace` with `-D warnings`
- `scope::no-rule-edits` — denies edits to `~/.claude/rules/*.md` unless orchestrator-meta

Role `git-ops` — documented in `docs/AGENT-ROLES.md` only; `_roles/git-ops.toml` has `spawnable = false` field. Orchestrator code refuses to spawn it. Exists for documentation of "who can do git" boundary.

Task spec persistence: task.toml files are ephemeral (gitignored under `tasks/`). Ledger row includes spec-SHA so historical specs are recoverable from `kei-sage` archive if someone wants cold-storage replay.

---

## Layer E — Role expression (extends / relaxes)

Roles compose via three optional fields on `[capabilities]`:

```toml
[capabilities]
extends = "<parent-role-slug>"    # optional — flattened first
required = ["cap-a", "cap-b"]     # optional — appended after parent
relaxes  = ["cap-c"]               # optional — dropped from flattened list
```

Resolution order:

1. If `extends` is present, recursively resolve the parent and take its flattened `required` list.
2. Append every local `required` entry not already present (order preserved).
3. Remove every entry named in `relaxes`. If a relaxed cap wasn't inherited, a stderr warning is emitted (no-op, not an error).
4. Cycle detection — an `extends` chain that loops back to an already-visiting role raises an error naming the offender.

Shipped examples:

- `_roles/read-only.toml` — base, no `extends`
- `_roles/explorer.toml` — `extends = "read-only"`, adds `tools::bash-allowlist`
- `_roles/edit-local.toml` — base
- `_roles/edit-shared.toml` — `extends = "edit-local"`, `required = []`, `relaxes = []` (the SSoT relaxation rides on `task.scope.files-whitelist`, not on capability drop)

Consumers: `compose::compose_prompt`, `prepare::prepare`, `verify::load_role_capabilities`, `dna::Dna::compose` — all go through `role::resolve_role`.

---

## Layer G — DNA identity

Every `AgentInvocation` carries a `dna` string encoding the composition:

```
<role>::<caps-bitmap>::<scope-hash>::<body-hash>-<nonce>
```

Segments:

- **role** — role slug from `task.role`
- **caps-bitmap** — hyphen-joined 2-char codes from the resolved capability list (see `dna::CAP_CODES`)
- **scope-hash** — 4-char `SHA-256` prefix of canonicalised scope (sorted whitelist + denylist)
- **body-hash** — 4-char `SHA-256` prefix of `task.body.text`
- **nonce** — 4-char random hex (disambiguates re-runs of identical specs)

Example (edit-local task touching `kei-forge`):

```
edit-local::NG-FW-FD-CP-CG-TG-ND-RF::A7B2::C9F1-xa7c
```

Round-trip: `Dna::compose(task, resolved)` → `.render()` → `Dna::parse(s)` returns an equal `Dna`. `render_human` prepends `dna: …` to the printable block; `render_json` and `render_toml` emit it as a `dna` field.

### Ledger integration

`kei-ledger` schema v2 adds a nullable `dna TEXT` column plus `idx_agents_dna_prefix` (first 30 chars) for DNA-prefix lookup. `kei-ledger fork … --dna <string>` persists it; legacy calls without the flag leave the column NULL so pre-v2 callers keep working.

### Capability atom codes (stable table)

| Name | Code |
|---|---|
| `policy::no-git-ops` | `NG` |
| `scope::files-whitelist` | `FW` |
| `scope::files-denylist` | `FD` |
| `quality::constructor-pattern` | `CP` |
| `quality::cargo-check-green` | `CG` |
| `quality::tests-green` | `TG` |
| `safety::no-dep-bump` | `ND` |
| `output::report-format` | `RF` |
| `output::severity-grade` | `SG` |
| `tools::deny-tools` | `DT` |
| `tools::bash-allowlist` | `BA` |

Additions are allowed; removals are not. Unknown names render as `??` so missing entries are visible rather than silently dropped.
