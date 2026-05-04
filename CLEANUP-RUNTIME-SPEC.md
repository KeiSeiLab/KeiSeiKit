# kei-cleanup — Cleanup Runtime + Agent Specification

> Spec version: v1.draft (2026-05-04)
> Status: DESIGN — not yet implemented
> Author: kei-architect (read-only) → orchestrator approval required → kei-code-implementer
> Lock: this spec changes only via PR with explicit "spec-amend:" commit prefix

---

## §1 Vision

Codebase hygiene is recurring work performed manually across 100+ Rust crates in KeiSeiKit-public + downstream consumers (K2K, Cartoon Studio, leadgen, etc.). Nine categories of waste recur:

1. **Dead code** — `pub` items with zero callers, deprecated paths
2. **Unused workspace dependencies** — declared in `[dependencies]`, never imported
3. **Workspace dep drift** — version redeclared per-crate, diverges from `[workspace.dependencies]`
4. **Missing workspace test crates** — integration tests crossing 2+ sibling crates create dev-dep cycles
5. **Stale TODO/FIXME** — accumulating without age tracking, no triage
6. **Doc warnings** — broken intra-doc links, private-item refs
7. **Constructor Pattern violations** — files >200 LOC, functions >30 LOC (Rule ZERO)
8. **Cross-crate naming drift** — `D_INIT` vs `DEFAULT_D` style inconsistency
9. **Test coverage map gaps** — derivations / theorems with no corresponding test

ONE primitive crate (`kei-cleanup`) owns detection. ONE agent (`kei-cleanup`) owns proposal. The orchestrator owns the commit. Read-only by design. Self-correcting only by user approval.

This is **substrate**, not a product. Every KeiSei project gets the same hygiene loop one `cargo install kei-cleanup` away.

---

## §2 Architecture

`kei-cleanup` slots into KSK as a sibling of `kei-conflict-scan`, `kei-refactor-engine`, `kei-graph-check`, `kei-arch-map` — the "scan + report + propose" family.

It does not replace any of them. The boundary:

| Primitive | Concern |
|---|---|
| `kei-conflict-scan` | rule / hook / block / memory conflicts |
| `kei-refactor-engine` | structural refactor proposals (Phase C deep-sleep) |
| `kei-graph-check` | dependency graph validation |
| `kei-arch-map` | architecture-claim verification (PLAN.toml ↔ repo) |
| **`kei-cleanup` (NEW)** | **code hygiene** — dead code, unused deps, LOC violations, TODO age, naming drift |
| `kei-leak-matrix` | content-protection regex SSoT (different concern) |

Wiring:
- **Crate** at `_primitives/_rust/kei-cleanup/`, declared in `_primitives/_rust/Cargo.toml` workspace `members`.
- **Manifest** at `_manifests/kei-cleanup.toml` (assembler regenerates `agents/kei-cleanup.md` on edit).
- **Skill** at `skills/cleanup/SKILL.md` for manual `/cleanup` invocation; assembler indexes via `kei-registry`.
- **Hook** at `hooks/post-cargo-build-cleanup-reminder.sh` (PostToolUse:Bash, advisory, exit 0). Reminds orchestrator that cleanup hasn't run in N days. NOT a blocker.
- **Audit trail** via `kei-ledger` — every run inserts a row tagged `kind=cleanup-run`.

---

## §3 Cleanup Runtime Crate Spec

### §3.1 Crate metadata
- **Name:** `kei-cleanup`
- **Path:** `_primitives/_rust/kei-cleanup/`
- **Binary:** `kei-cleanup`
- **License:** Apache-2.0 (workspace inherit)
- **Edition:** 2021 (workspace inherit)
- **Description:** "Workspace-wide code-hygiene scanner: dead code, unused deps, LOC checks, TODO age, naming drift, doc warnings, coverage map."

### §3.2 Module structure (Constructor Pattern)

```
src/
  main.rs                    # CLI entry, clap derive, ≤80 LOC
  lib.rs                     # pub fn run_all() + re-exports, ≤60 LOC
  report.rs                  # CleanupReport + Finding structs, ≤180 LOC
  config.rs                  # cleanup.toml loader, ≤120 LOC
  scanners/
    mod.rs                   # pub use of each scanner, ≤30 LOC
    dead_code.rs             # cargo-udeps + machete shell-out, ≤180 LOC
    unused_deps.rs           # syn AST + Cargo.toml diff, ≤200 LOC
    dep_drift.rs             # workspace-deps SSoT check, ≤150 LOC
    loc_check.rs             # walkdir + syn for fn boundaries, ≤180 LOC
    naming_consistency.rs    # regex pairs from cleanup.toml, ≤150 LOC
    coverage_map.rs          # doc-comment ↔ test grep, ≤180 LOC
    workspace_tests.rs       # cross-crate test detector, ≤180 LOC
    todo_age.rs              # git blame batched, ≤150 LOC
    doc_warnings.rs          # cargo doc -D warnings parse, ≤120 LOC
  ledger.rs                  # kei-ledger row insertion, ≤80 LOC
```

Every file ≤200 LOC. Every public function ≤30 LOC. Rule ZERO compliant.

### §3.3 Public API

```rust
// lib.rs
pub fn run_all(workspace: &Path, cfg: &Config) -> Result<CleanupReport>;
pub fn run_scanner(name: &str, workspace: &Path, cfg: &Config) -> Result<Vec<Finding>>;

// report.rs
pub struct CleanupReport {
    pub schema_version: u32,
    pub workspace: PathBuf,
    pub workspace_sha: String,
    pub timestamp_utc: DateTime<Utc>,
    pub runtime_ms: u64,
    pub findings: Vec<Finding>,
    pub counts: Counts,
    pub scanners_run: Vec<String>,
    pub scanners_skipped: Vec<(String, String)>,
}

pub struct Finding {
    pub kind: FindingKind,
    pub severity: Severity,
    pub location: Location,
    pub message: String,
    pub fix_hint: Option<String>,
    pub rule_ref: Option<String>,
    pub confidence: Confidence,
}

pub enum FindingKind {
    DeadCode, UnusedDep, DepDrift, LocFile, LocFunction,
    NamingDrift, CoverageGap, WorkspaceTestNeeded, StaleTodo, DocWarning,
}
```

### §3.4 CLI

```
kei-cleanup [OPTIONS] [PATH]

Options:
  --config <FILE>           cleanup.toml (default: ./cleanup.toml or workspace root)
  --json <FILE>             Emit JSON report
  --sarif <FILE>            Emit SARIF for IDE / CI
  --only <SCANNERS>         Comma-list: dead_code,unused_deps,...
  --skip <SCANNERS>         Inverse of --only
  --severity-min <LEVEL>    high|medium|low
  --no-ledger               Skip kei-ledger row insertion
  --quiet                   Suppress TTY output (for CI)
  --fail-on <LEVEL>         Exit 1 if findings ≥ level
  --workspace-deps-only     Skip scanners that require nightly
```

### §3.5 Configuration (cleanup.toml)

```toml
[loc]
file_max = 200
fn_max = 30
exclude_paths = ["target/", "_archive/"]

[naming_pairs]
pairs = [
  ["D_INIT", "DEFAULT_D", "INITIAL_D"],
  ["MAX_BUF", "BUFFER_LIMIT"],
]

[coverage_map]
derivation_marker = "// derive:"
test_id_marker = "// covers:"

[todo_age]
warn_days = 30
fail_days = 180

[scanners]
enabled = ["dead_code", "unused_deps", "dep_drift", "loc_check",
           "naming_consistency", "coverage_map", "workspace_tests",
           "todo_age", "doc_warnings"]

[ignore]
unused_deps = []
dead_code = []
```

### §3.6 Output formats

**Pretty TTY (default):**
```
kei-cleanup v0.1.0  •  workspace=KeiSeiKit-public  •  sha=72d257e

═══ findings: 23 [REAL: cleanup-run-2026-05-04T03-30Z] ═══
  [HIGH]    3
  [MEDIUM]  12
  [LOW]     8

[HIGH] dead_code  _primitives/_rust/kei-foo/src/bar.rs:42
  pub fn foo() — zero callers in workspace
  fix: remove or mark #[allow(dead_code)] with justification
```

**JSON (`--json out.json`):** the `CleanupReport` struct, serde_json.
**SARIF (`--sarif out.sarif`):** OASIS SARIF 2.1.0.

### §3.7 Implementation notes

- **Rust deps (workspace, no new external):** `clap`, `serde`, `serde_json`, `toml`, `walkdir`, `regex`, `syn`, `anyhow`, `thiserror`, `chrono`. Internal: `kei-ledger`.
- **External CLIs (shell out):** `cargo`, `cargo-udeps` (nightly, optional), `cargo-machete` (stable, optional), `git`.
- **Toolchain detection:** if `cargo-udeps` absent, fall back to `cargo machete`; if both absent, mark scanner skipped.
- **Performance budget:** ≤30 s on 105-crate workspace (warm cache).
- **No persistence beyond ledger row.** Reports live on disk only if user redirects `--json` / `--sarif`.
- **Detection only.** All "fix" suggestions are textual; cleanup agent drafts hand-offs; orchestrator commits.

---

## §4 Cleanup Agent Manifest Spec

### §4.1 Manifest TOML (`_manifests/kei-cleanup.toml`)

```toml
[meta]
name = "kei-cleanup"
role = "read-only"
description = "Reads kei-cleanup CleanupReport, drafts prioritized fix proposals, never commits."
version = "v1"

[capabilities]
deny-tools = ["Edit", "Write"]
allow-tools = ["Read", "Glob", "Grep", "Bash[kei-cleanup,cargo,git-readonly]"]

[blocks]
include = [
  "_blocks/role-readonly.md",
  "_blocks/output-report-format.md",
  "_blocks/output-severity-grade.md",
  "_blocks/baseline-rules.md",
  "_blocks/cleanup-runtime-protocol.md",
]

[trigger]
modes = ["manual", "skill", "scheduled"]
manual = "Agent tool spawn by orchestrator"
skill = "/cleanup"
scheduled = "weekly via kei-scheduler"

[runtime]
isolation = "worktree"
timeout_min = 15
output = "report"

[handoff]
on_findings_high = "kei-code-implementer"
on_findings_medium = "kei-critic"
on_findings_low = "log-only"
```

### §4.2 Agent prompt structure

```
# ROLE
You are a cleanup agent. You consume CleanupReport JSON, prioritize findings,
and emit a fix-plan for the orchestrator. You NEVER edit files, NEVER run cargo,
NEVER commit.

# WORKFLOW
1. Read CleanupReport from <path-passed-by-orchestrator>
2. For each finding:
   - Classify [HIGH/MEDIUM/LOW] (severity already in report)
   - Draft 1-line fix proposal
   - Estimate effort: trivial / small / medium / large
3. Emit final report (see OUTPUT FORMAT)
4. Append STATUS-TRUTH MARKER per RULE 0.16

# OUTPUT FORMAT
=== KEI-CLEANUP AGENT REPORT ===
Run: <ts>  workspace=<path>  sha=<sha>
Findings: total=<N> high=<H> med=<M> low=<L>
[REAL: cleanup-run-<ts>]

## High-priority fixes (block merge if any)
[HIGH] <kind> <file:line>
  Issue: <message>
  Fix:   <proposed-action>
  Effort: trivial|small|medium|large
  Handoff: kei-code-implementer / orchestrator

## Medium-priority fixes
## Low-priority fixes
## Deferred / not actionable

=== STATUS-TRUTH MARKER ===
shipped: <functional|partial|scaffolding>
stubs: 0
cargo-check: NOT-RUN (sandbox)
behaviour-verified: not-applicable (read-only analysis)
follow-up-required:
  - orchestrator: triage HIGH findings
```

### §4.3 New block: `_blocks/cleanup-runtime-protocol.md`

```
# Cleanup Runtime Protocol

You consume `CleanupReport` JSON produced by `kei-cleanup` runtime.

INPUT contract:
- `report_path`: absolute path to JSON report
- `workspace_root`: absolute path to scanned workspace
- `severity_min`: minimum severity to action (default: medium)

OUTPUT contract:
- Markdown report with HIGH/MEDIUM/LOW sections
- Each finding: 1 issue line + 1 fix line + 1 effort line + 1 handoff line
- Final STATUS-TRUTH MARKER block (RULE 0.16)

PROHIBITED:
- Editing files (read-only role)
- Running cargo (sandbox + RULE 0.13)
- Committing (RULE 0.13)
- Estimating durations without [REAL:]/[FROM-JOURNAL:]/[ESTIMATE-HTC:] (RULE 0.18)

DECISION rules:
- HIGH = needs fix before next merge to main
- MEDIUM = schedule into next cleanup window (≤7d)
- LOW = batch with next refactor, can defer indefinitely

If report has 0 findings → emit single-line "clean" report + STATUS-TRUTH.
```

### §4.4 Tool restrictions

`role = "read-only"` auto-injects `deny-tools = ["Edit", "Write"]` via `_blocks/role-readonly.md`. `Bash` allowed only for read-only subcommands (`kei-cleanup --json`, `cargo metadata`, `git rev-parse HEAD`). Orchestrator's PreToolUse:Bash hook chain filters via `safety-guard.sh`.

### §4.5 Agent output for orchestrator

The agent's final markdown is consumed by the orchestrator:
1. Read HIGH section.
2. For each HIGH, decide: fix-now / defer / dismiss-with-reason.
3. Spawn `kei-code-implementer` per fix-now finding (RULE 0.13: orchestrator branch + commit).
4. Log ledger row tagged `kind=cleanup-action` linking back to `cleanup-run` row.

---

## §5 RULE compliance

| Rule | How spec complies |
|---|---|
| **RULE 0.4 NO HALLUCINATION** | Every numeric grade-tagged E1-E6. CleanupReport mechanically produced. |
| **RULE 0.5 PLAN MODE FIRST** | This spec IS the plan. Implementation requires user approval. |
| **RULE 0.13 ORCHESTRATOR-BRANCH-FIRST** | Agent prompt includes ban-phrase "MUST NOT invoke git". Worktree isolation. Orchestrator commits. |
| **RULE 0.16 STATUS-TRUTH** | Agent output footer mandatory. `behaviour-verified: not-applicable` since read-only. |
| **RULE 0.17 DISK-HEADROOM** | Reports default TTY only. JSON/SARIF only via flag. |
| **RULE 0.18 NUMERIC EVIDENCE** | TTY auto-emits `[REAL: cleanup-run-<ts>]` markers. |
| **RULE 0.10 RECURRENCE-ESCALATE** | Hook upgrades REMIND→WARN→ENFORCE on recurrence (v0.3). |
| **Constructor Pattern (Rule ZERO)** | All modules ≤200 LOC, all fns ≤30 LOC. Self-checked by `loc_check` — dogfooded. |

---

## §6 Phasing

### v0.1 — Read-only detection (target post-approval: ~1 week [ESTIMATE-HTC])

Scope:
- Crate scaffolding + workspace registration
- Modules: `dead_code` (machete fallback), `unused_deps`, `dep_drift`, `loc_check`
- CLI: `kei-cleanup [PATH]`, `--json`, `--quiet`, `--fail-on`
- Tests: integration test on KSK-public itself
- No agent yet — runtime alone, invokable via Bash

Acceptance:
- `cargo run --release -p kei-cleanup -- _primitives/_rust/` returns in <30 s [ESTIMATE-HTC: not benchmarked]
- JSON output validates against schema_version=1
- Self-application: `loc_check` passes on `kei-cleanup` own source

### v0.2 — Agent + workspace test crate auto-detection (~+1 week [ESTIMATE-HTC])

Scope:
- Manifest, block, skill, hook
- Module `workspace_tests`: detects integration tests crossing 2+ crates
- Hook `post-cargo-build-cleanup-reminder.sh` (REMIND tier)

Acceptance:
- `/cleanup` skill invokable; agent emits markdown report
- Hook fires reminder when `cargo build` runs in workspace not seen cleanup in 7 days
- Workspace-tests scanner correctly identifies the K2K-pattern dev-dep cycle

### v0.3 — Full hygiene sweep (~+2 weeks [ESTIMATE-HTC])

Scope:
- Modules: `naming_consistency`, `coverage_map`, `todo_age`, `doc_warnings`
- Hook severity ladder (REMIND → WARN → ENFORCE per RULE 0.10)
- SARIF output
- `cleanup.toml` config loader
- `cargo-udeps` integration

Acceptance:
- Self-applies cleanly to KSK-public (zero HIGH on main)
- K2K integration: `cargo install kei-cleanup` + CI gate
- Documented in `docs/CLEANUP.md`

### v0.4 — Quality of life (opportunistic)

- Parallelize via `rayon` if profiling shows >30 s wall
- IDE integration via SARIF + LSP-friendly JSON
- `kei-cleanup explain <finding-id>` subcommand
- Cross-project share: `kei-cleanup --remote` — substrate-wide hygiene dashboard

---

## §7 Cross-project applicability

**Primary consumer: K2K** (`~/Projects/K2K/`)

```bash
cargo install kei-cleanup --git https://keigit.com/denis/KeiSeiKit-public
cd ~/Projects/K2K
kei-cleanup --json .cleanup/report.json --fail-on high --severity-min medium
```

K2K's CI step gates merges on HIGH findings. MEDIUM findings show in PR comment but don't block.

**Other projects (opt-in):** Cartoon Studio, leadgen, Recruiter, Scrapers, Vortex (cyber-banned, runs locally only — reports stay on user's machine per `security.md`).

For each: identical install, identical CLI. Per-project `cleanup.toml` tunes naming pairs / LOC limits / ignored deps.

**Substrate dogfood:** KSK-public itself runs `kei-cleanup` nightly via `kei-cron-scheduler`; output feeds Phase B REM consolidation report (RULE 0.15) so the morning markdown surfaces drift.

---

## §8 Open questions / decisions needed from user

1. **Crate name.** `kei-cleanup` (matches user's word "уборка"), parallels `kei-conflict-scan` style. **Decision: confirm or pick alternative.**
2. **Hook trigger event.** (a) PostToolUse:Bash on `cargo build` (immediate, frequent — risk: noisy); (b) Stop event with N-day debounce (calmer); (c) `kei-cron-scheduler` weekly. Recommend (b)+(c) parallel; (a) opt-in env. **Decision needed.**
3. **`cargo-udeps` nightly dep.** Graceful fallback to `machete` recommended. **Decision: hard requirement nightly OR graceful fallback?**
4. **`rayon` for parallelism.** Defer to v0.4. **Decision: add now or defer?**
5. **CI integration shape.** GitHub Actions YAML in repo OR documented in `docs/CLEANUP.md`. KSK-public uses Forgejo CI primarily. **Decision: which CI vendor(s)?**
6. **Phase B / sleep-layer integration.** HIGH-only auto-feed; MEDIUM/LOW go to backlog. **Decision: confirm or change.**
7. **Coverage-map format.** In-code markers (`// derive:`/`// covers:`) OR out-of-band TOML. **Decision needed.**
8. **Fix-now severity for code-implementer hand-off.** v0.2 proposes HIGH-only auto-handoff. Alternative: also MEDIUM if `effort: trivial`. **Decision needed.**

---

## Appendix A — Scanner I/O specs (compact)

| Scanner | Input | External tools | Output kind | Confidence |
|---|---|---|---|---|
| dead_code | workspace root | cargo-udeps OR cargo-machete | DeadCode | High (udeps) / Low (machete) |
| unused_deps | Cargo.toml + src/ | syn AST | UnusedDep | High |
| dep_drift | workspace Cargo.toml | none | DepDrift | High |
| loc_check | walk src/ | syn AST | LocFile / LocFunction | High |
| naming_consistency | cleanup.toml + grep src/ | regex | NamingDrift | High |
| coverage_map | doc-comment ↔ tests/ | regex | CoverageGap | Medium |
| workspace_tests | tests/ + Cargo.toml | syn | WorkspaceTestNeeded | High |
| todo_age | grep src/ + git blame | git | StaleTodo | High |
| doc_warnings | cargo doc -D warnings | cargo | DocWarning | High |

## Appendix B — Test fixture layout (v0.1 acceptance)

```
tests/
  fixtures/
    fx-clean-workspace/         # zero findings expected
    fx-dirty-workspace/         # known counts: 2H 5M 3L
    fx-todo-aged/               # 3 TODOs of varying age
  integration_self.rs           # runs kei-cleanup on KSK-public
  integration_fixtures.rs       # runs against fx-* fixtures
```

## Appendix C — Out of scope

- **Auto-applying fixes.** RULE 0.13 forbids. Cleanup proposes; orchestrator commits.
- **Patent-IP scanning.** Owned by `kei-leak-matrix`.
- **Architecture-claim verification.** Owned by `kei-arch-map`.
- **Conflict resolution.** Owned by `kei-conflict-scan` + `kei-refactor-engine`.
- **Memory store maintenance.** Owned by `kei-memory` + sleep layer.
- **Cross-language linting.** Rust-only for v0.x.
- **AI-driven refactor proposals.** Cleanup detects mechanically; LLM-rewrite is `kei-code-implementer` territory.

---

End of spec v1.draft.
