# KeiSeiKit Substrate Overview & Encyclopedia

**Last updated:** 2026-05-02  
**Status:** Comprehensive reference for substrate architecture, manifests, capabilities, roles, packages, and installer components.

---

## 1. Architecture Overview

KeiSeiKit is a **multi-tier agent composition substrate** built on four mutually-dependent layers:

```
┌─────────────────────────────────────────────────┐
│ _assembler (Rust binary)                        │
│ Composes agent .md files from:                  │
│ - _manifests/*.toml (agent spec)                │
│ - _blocks/*.md (reusable prompt fragments)      │
│ - _capabilities/*/text.md (capability rules)    │
│ - _roles/*.toml (capability bundles)            │
│ - ~/.claude/rules/*.md (user rules)             │
│ Outputs: .claude/agents/*.md (generated)        │
└─────────────────────────────────────────────────┘
         ↓ one-way dependency
┌─────────────────────────────────────────────────┐
│ _capabilities/ (declarative + Rust gates)       │
│ 18 capability atoms in 6 categories:            │
│ - policy (git scope, no-git-ops)                │
│ - scope (files whitelist/denylist)              │
│ - quality (Constructor Pattern, cargo-check)    │
│ - safety (no-dep-bump)                          │
│ - output (report-format, severity-grade)        │
│ - tools (read-only, bash-allowlist)             │
│ - verify (fork-audit)                           │
│ Each: capability.toml + text.md + Rust impl     │
└─────────────────────────────────────────────────┘
         ↓ one-way dependency
┌─────────────────────────────────────────────────┐
│ _roles/ (capability bundles)                    │
│ 7 roles = ordered list of capabilities +        │
│ tool allowlist + escalation policy.             │
│ Examples:                                        │
│ - read-only (10 capabilities, Bash forbidden)   │
│ - edit-local (8 capabilities, cargo-check req)  │
│ - edit-shared (12 capabilities, workspace auth) │
│ - git-ops (5 capabilities, full git-safe)       │
│ - auditor (6 capabilities, diff + audit focus)  │
│ - explorer (4 capabilities, research-first)     │
│ - merger (7 capabilities, merge ceremony)       │
└─────────────────────────────────────────────────┘
         ↓ many-to-one dependency
┌─────────────────────────────────────────────────┐
│ _manifests/ (agent specs)                       │
│ 38 agent manifests in 8 families:               │
│ - code-implementer (7 languages)                │
│ - critic (5 specializations)                    │
│ - researcher (web/code/hybrid)                  │
│ - ml-implementer & ml-researcher                │
│ - infra-implementer (4 specializations)         │
│ - security-auditor (3 specializations)          │
│ - validator (6 specializations)                 │
│ - architect, cost-guardian, etc.                │
│ Each: role, model, domain_in/out, handoffs      │
└─────────────────────────────────────────────────┘
```

**Data flow at design time:**
1. User edits manifest `_manifests/code-implementer-rust.toml`
2. `_assembler` reads manifest + referenced blocks/roles
3. `_assembler` invokes kei-registry to refresh `capabilities.toml` index
4. `_assembler` regenerates `.claude/agents/code-implementer-rust.md`
5. keimd re-indexes the new agent

**Data flow at runtime:**
1. User invokes agent via Claude Code Agent tool
2. Pre-spawn: `kei-agent-runtime compose` builds final prompt from role + capabilities
3. Agent executes with constraints from role (tools, file scope, capabilities)
4. Pre-tool-use: `kei-capability check <name>` gates operations (Bash, Edit, Write)
5. On agent return: `kei-capability verify <name>` validates output against constraints
6. Post-merge: simulated-merge test ensures agent output compiles and tests pass after integration

---

## 2. Agent Manifests Table

| Agent | Family | Model | Domain | Handoffs | Stability |
|-------|--------|-------|--------|----------|-----------|
| **code-implementer** | impl-generic | sonnet | Rust/Swift/Python/Go/Flutter/TS | ml-impl / infra / architect / critic / security | stable |
| **code-implementer-rust** | impl-lang | sonnet | Rust default; Constructor Pattern; tests-first | ml-impl / infra / architect / critic | stable |
| **code-implementer-swift** | impl-lang | sonnet | SwiftUI + SPM; menubar/iOS; Constructor Pattern | infra / architect | stable |
| **code-implementer-python** | impl-lang | sonnet | Python (RULE 0.2 exception #N only) | ml-impl / infra / architect | stable |
| **code-implementer-go** | impl-lang | sonnet | Go; mesh/CLI/embedded; Constructor Pattern | infra / architect | stable |
| **code-implementer-flutter** | impl-lang | sonnet | Flutter/Dart; Riverpod; Clean Architecture | infra / architect | stable |
| **code-implementer-typescript** | impl-lang | sonnet | Next.js/Node/browser; type-safe contracts | ml-impl / infra / architect | stable |
| **critic** | critic-generic | opus | Ruthless code review; anti-patterns, tech debt, perf, security | code-impl (fixes) | stable |
| **critic-bug** | critic-spec | opus | Off-by-one, error-swallowing, race conditions | code-impl | stable |
| **critic-perf** | critic-spec | opus | N+1, hot loops, unbounded retention | code-impl | stable |
| **critic-anti-pattern** | critic-spec | opus | God classes, deep inheritance, shotgun surgery | code-impl | stable |
| **critic-tech-debt** | critic-spec | opus | Dead code, TODOs, version-skew, stale deps | code-impl | stable |
| **researcher** | research-generic | opus | Web + codebase research; E1-E6 grading | validator / code-impl | stable |
| **researcher-web** | research-spec | opus | WebFetch/WebSearch only; fact-finding | validator | stable |
| **researcher-code** | research-spec | opus | Glob/Grep/Read; codebase discovery | code-impl | stable |
| **researcher-hybrid** | research-spec | opus | Parallel web + code research orchestrator | validator / code-impl | stable |
| **ml-implementer** | ml | sonnet | Training/inference; Modal jobs; Math-First paradigm | code-impl / infra / modal-runner | stable |
| **ml-researcher** | ml | opus | Paper review, benchmarks, reproducibility, tooling | ml-impl | stable |
| **infra-implementer** | infra-generic | sonnet | Deploy, CI/CD, secrets, IaC; credential isolation | code-impl / security | stable |
| **infra-implementer-cicd** | infra-spec | sonnet | GitHub Actions, GitLab CI, build pipelines | code-impl | stable |
| **infra-implementer-container** | infra-spec | sonnet | Dockerfile, OCI, multi-stage, distroless | code-impl | stable |
| **infra-implementer-iac** | infra-spec | sonnet | Terraform, Pulumi, CDK; Constructor Pattern | code-impl | stable |
| **infra-implementer-secrets** | infra-spec | sonnet | Vault, sops, age, env-var injection | code-impl | stable |
| **security-auditor** | security | opus | Risk-classified (HIGH/MEDIUM/LOW) audit with 9-point review | code-impl | stable |
| **security-auditor-differential** | security-spec | opus | Auth bypass, injection, deserialization, race conditions | code-impl | stable |
| **security-auditor-variant** | security-spec | opus | Pattern grep after vuln found; systematic coverage | code-impl | stable |
| **security-auditor-supply-chain** | security-spec | opus | New deps: maintainers, CVE, transitive deps | code-impl | stable |
| **validator** | validation | opus | API existence, version compat, code reality, benchmarks | code-impl | stable |
| **validator-api** | validator-spec | opus | API/OpenAPI verification | code-impl | stable |
| **validator-benchmark** | validator-spec | opus | External benchmark claim verification | code-impl | stable |
| **validator-code-reality** | validator-spec | opus | Behavioural claims vs running code | code-impl | stable |
| **validator-version** | validator-spec | opus | Semver, MSRV, transitive dep compatibility | code-impl | stable |
| **validator-doc** | validator-spec | opus | Documentation claim verification | code-impl | stable |
| **architect** | specialization | opus | System design, dependency analysis, patterns | code-impl | stable |
| **cost-guardian** | specialization | sonnet | Modal/AWS/GCP/fal.ai/Apify cost pre-launch gate | modal-runner | stable |
| **fal-ai-runner** | specialization | sonnet | Image/video/3D generation; model catalog & pricing | code-impl | stable |
| **modal-runner** | specialization | sonnet | Modal jobs: cost est, GPU compat, observability | code-impl | stable |
| **frontend-validator** | specialization | sonnet | tsc --noEmit, eslint, kei-db-contract, visual snapshots | code-impl | stable |

**Notes:**
- Model: `opus` (Opus 4 for complex reasoning), `sonnet` (Haiku 4.5 for routine implementation)
- All manifests reference role via `substrate_role = "<role>"` (see §3 below)
- Handoff targets are agents the manifest explicitly names for task delegation
- All stable as of 2026-05-02; no experimental agents currently active

---

## 3. Capabilities Table

| Capability | Category | Description | Gate/Verify | Event | Severity | Bypass Env |
|------------|----------|-------------|-------------|-------|----------|-----------|
| **policy::no-git-ops** | policy | Forbid git, gh repo, gh api /repos | gate | PreToolUse:Bash | block | ORCHESTRATOR_META |
| **policy::git-ops-scope** | policy | Allow git within scope (branch, commit, push) | gate | PreToolUse:Bash | warn | GIT_OPS_OVERRIDE |
| **scope::files-whitelist** | scope | Restrict Edit/Write to declared file glob patterns | gate+verify | PreToolUse:Edit\|Write | block | — |
| **scope::files-denylist** | scope | Blacklist specific file globs from any touch | gate+verify | PreToolUse:Edit\|Write | block | — |
| **scope::read-only** | scope | Deny all Edit/Write/Bash; allow Read/Glob/Grep only | gate | PreToolUse:Edit\|Write\|Bash | block | — |
| **quality::constructor-pattern** | quality | Enforce: file ≤ 200 LOC, function ≤ 30 LOC | verify | on-return | fail-if-exceed | — |
| **quality::cargo-check-green** | quality | Verify: `cargo check --workspace` exits 0 | verify | on-return | fail-if-error | — |
| **quality::tests-green** | quality | Verify: `cargo test` passes; min test-count floor | verify | on-return | fail-if-error | — |
| **safety::no-dep-bump** | safety | Forbid Cargo.toml [dependencies] edits; no `cargo add` | gate+verify | PreToolUse:Edit / on-return | block | DEPENDENCY_OVERRIDE |
| **output::report-format** | output | Require: Files written / cargo-check / cargo-test / LOC-delta / blockers / next (fields) | verify | on-return | fail-if-missing | — |
| **output::severity-grade** | output | Require: E1-E6 evidence grade on all numeric/claim output | verify | on-return | warn-if-missing | — |
| **output::merge-result** | output | Require: STATUS-TRUTH MARKER (shipped:functional\|partial\|scaffolding) | verify | on-return | warn-if-missing | STATUS_TRUTH_BYPASS |
| **tools::read-only** | tools | Deny Edit/Write; Bash patterns restricted to read-only | gate | PreToolUse:Bash\|Edit\|Write | block | — |
| **tools::cargo-only-bash** | tools | Restrict Bash to cargo-only patterns + safe mkdirs | gate | PreToolUse:Bash | block | CARGO_ONLY_OVERRIDE |
| **tools::bash-allowlist** | tools | Restrict Bash to declared allow-patterns | gate | PreToolUse:Bash | warn | — |
| **tools::deny-tools** | tools | Blacklist specific tools (e.g. NotebookEdit) | gate | PreToolUse:Any | block | — |
| **verify::fork-audit** | verify | Validate 6-file artefact bundle on agent branch (RULE 0.12) | verify | on-return | warn-if-missing | AGENT_AUDIT_SKIP |

**Notes:**
- **Gate vs Verify:** gate=PreToolUse (blocks before action), verify=on-return (validates after)
- **Severity:** block=exit 2 (fail), fail-if-*=test failure, warn=stderr advisory (exit 0), —=no bypass
- **Stage:** design-time (manifest load) vs runtime (agent execution)
- **Rust impl:** gates + verifies in `_primitives/_rust/kei-agent-runtime/src/{gates,verifies}/`

---

## 4. Roles Table

| Role | Spawnable | Capabilities | Tools Allowed | Bash Patterns | Description |
|------|-----------|--------------|---------------|---------------|-------------|
| **read-only** | yes | policy::no-git-ops, scope::read-only, tools::read-only, output::report-format | Read, Glob, Grep | none | Research/investigation only; no mutations. Read docs/files, find info, report findings. |
| **explorer** | yes | policy::no-git-ops, scope::read-only, tools::read-only | Read, Glob, Grep, Bash (log-only) | grep, find (log output) | Exploration+discovery; grep codebase, scan logs. Pre-implementation research gate. |
| **edit-local** | yes | no-git-ops, files-whitelist, files-denylist, constructor-pattern, cargo-check-green, tests-green, no-dep-bump, report-format | Read, Write, Edit, Glob, Grep, Bash | ^cargo, ^mkdir, ^rm -rf /tmp | Local file edits within whitelist; cargo check/test required; no git, no deps. Typical code-implementer sandbox. |
| **edit-shared** | yes | no-git-ops, files-whitelist, files-denylist, constructor-pattern, cargo-check-green, tests-green, no-dep-bump, workspace-auth, report-format | Read, Write, Edit, Glob, Grep, Bash, Agent | ^cargo, ^mkdir, ^rustc, ^tsc | Workspace edits; workspace-level auth; can spawn sub-agents. No git; Cargo workspace-level checks. |
| **git-ops** | no | git-ops-scope (allow safe git), files-whitelist, files-denylist, no-dep-bump, report-format, merge-result | Read, Write, Edit, Glob, Grep, Bash, Agent | git (scoped), gh (safe patterns), cargo | Orchestrator-only. Full git (branch, commit, push) within declared scope. Merge ceremony + verification gates. |
| **auditor** | yes | policy::no-git-ops, scope::read-only (plus diff access), constructor-pattern, cargo-check-green, tests-green, output::report-format | Read, Glob, Grep, Bash (cargo only) | ^cargo, audit-specific | Post-code-review role. Runs cargo check/test, diffs, double-audit protocol. Reports findings without fixes. |
| **merger** | no | git-ops-scope, constructor-pattern, cargo-check-green, tests-green, output::merge-result | all | all (git-scoped) | Orchestrator-only. Merges agent branches; runs pre-merge verification (cargo + test count); manages merge ceremony. |

**Notes:**
- **Spawnable:** agents marked "no" cannot be invoked by user; only orchestrator can invoke them
- **Capabilities:** ordered list; text.md fragments concatenated in this order into final prompt
- **Tools Allowed:** hard deny if not in this list (except git-ops / merger which are orchestrator-only)
- **Bash Patterns:** additional restriction via regex; agent Bash restricted to matching patterns only
- **Escalation policy:** most roles escalate via file-return field list; git-ops/merger escalate via merge ceremony

---

## 5. TypeScript Packages Table

| Package | Version | Purpose | Key Dependencies | Build Target |
|---------|---------|---------|------------------|--------------|
| **@keisei/mcp-server** | 0.14.0 | MCP server exposing KeiSeiKit Rust primitives as tools | @modelcontextprotocol/sdk ^1.0.0, execa ^9.0.0, zod ^3.23.0 | Node.js ≥18.0.0; native binaries via bun compile (darwin/linux/windows) |
| **@keisei/gmail-adapter** | 0.5.2 | Gmail API integration for email-based task intake | @google-cloud/gmail ^1.3.0, nodemailer-mock ^2.0.0 | Node.js ≥18.0.0 |
| **@keisei/grok-adapter** | 0.3.1 | Grok (xAI) LLM provider bridge | openai-compatible ^1.0.0 | Node.js ≥18.0.0 |
| **@keisei/telegram-adapter** | 0.6.0 | Telegram Bot API integration for notifications + input | telegram-typings ^4.10.0, node-telegram-bot-api ^0.65.0 | Node.js ≥18.0.0 |
| **@keisei/recall-adapter** | 0.2.1 | Recall.ai (browser automation) integration | recall-ai-sdk ^1.0.0 | Node.js ≥18.0.0; browser environment recommended |
| **@keisei/youtube-adapter** | 0.1.8 | YouTube API integration for video transcript + metadata | googleapis ^118.0.0 | Node.js ≥18.0.0 |

**Notes:**
- All packages scoped under `@keisei/` on npm (published to keigit.com npm registry)
- All use TypeScript 5.5+ with strict mode; zod for runtime validation
- Build output lives in `dist/` (generated from `src/` via `tsc -b`)
- MCP server ships as multi-target native binaries (darwin/linux/windows arm64 + x64)

---

## 6. Top-Level Docs (25 files)

| Document | Purpose | Key Sections |
|----------|---------|--------------|
| **SUBSTRATE-SCHEMA.md** | Atom + capability schema SSoT | Core concept, file layout, Cargo.toml metadata, JSON Schema conventions, discovery |
| **AGENT-SUBSTRATE-SCHEMA.md** | Agent capability atoms (gates+verifies) | Core concept triplet, file layout, capability.toml shape, Rust trait contract |
| **ARCHITECTURE.md** | Stack overview + trait impl matrix | Compute/Git/Memory/Auth/Notify/Network/LLM/ServiceManager impls per backend |
| **AGENT-ROLES.md** | Generated role matrix (human-readable) | 7 roles × capabilities × tools × escalation (auto-generated from _roles/*.toml) |
| **PHILOSOPHY.md** | Substrate philosophy + design principles | Single-source-of-truth, Constructor Pattern, no overlays, decomposability |
| **DNA-INDEX.md** | Agent DNA (deterministic identity) format | 80-char DNA breakdown: role::caps::scope-sha8::body-sha8-nonce |
| **IMPORT-RUNTIME.md** | Foreign-project ingestion pipeline | Decompose → match → extract skills → plan → execute (Hermes proof-of-concept) |
| **PUBLISHING.md** | Community npm registry + scoped package publishing | keigit.com npm, OAuth, per-user PAT, `npm publish` / `npm install` flow |
| **RULES-AS-BLOCKS.md** | How user ~/.claude/rules/*.md become prompt blocks | Rule fragment extraction, RULE re-composition, link-tracking |
| **QUICKSTART.md** | 60-second install guide | 11 install profiles (minimal/core/full + MCP/Cortex/Cursor/Continue/etc) |
| **INSTALL.md** | Full installation docs | Prereqs, profiles, lib-*.sh breakdown, troubleshooting |
| **CONVERGENCE-PLAN.md** | Multi-stream parallel work roadmap | UI / Atoms refactor / Graph / Runtime phases (2026-06 closure target) |
| **REFERENCE.md** | Command-line reference for kei-* binaries | kei-runtime, kei-sage, kei-registry, kei-import, kei-capability, kei-forge |
| **SCHEMA-LOCKED.md** / **SCHEMA-UNLOCKED.md** | Lock markers for breaking change gates | SUBSTRATE-SCHEMA.md locked 2026-05-02; 6-week parallel window |
| **AGENT-SCHEMA-LOCKED.md** | Lock marker for agent substrate | AGENT-SUBSTRATE-SCHEMA.md locked 2026-04-23; 3-week parallel window |
| **SECURITY.md** | Substrate security model | Sandboxing, capability enforcement, audit gates, simulated-merge verification |
| **SLEEP-LAYER.md** | Three-phase nightly consolidation (REM/NREM) | Phase A incubation (tasks), Phase B consolidation (reports), Phase C deep-sleep (conflicts) |
| **TAXONOMY.md** | Metadata taxonomy for all substrate objects | kingdom, mechanism, domain, layer, stage, stability, language (Dublin Core + custom) |
| **HANDOFF-WAKE.md** | Agent handoff orchestration + wake-on-complete | Async task queue, named handoffs, web hook triggers |
| **WHY.md** | Motivation + pain points solved | Context loss, agent collisions, duplicate work, no fork hygiene |
| **USB-BRAIN-GUIDE.md / *.md** | Portable offline KeiSeiKit on USB | Self-contained substrate on external drive; no internet required |

---

## 7. Installer Components (install/lib-*.sh)

| Library | Purpose | Functions |
|---------|---------|-----------|
| **lib-log.sh** | Logging + output formatting | color_info, color_success, color_warn, color_error, log_step |
| **lib-prereqs.sh** | System prerequisite checks + installation | check_rust, check_node, check_git, install_cargo_deps |
| **lib-profile.sh** | Profile selection (minimal/core/full + target) | detect_client, show_profiles, select_profile |
| **lib-pathway.sh** | Pathway builder (install → scaffold → verify) | build_pathway, run_checks, verify_install |
| **lib-primitives.sh** | Rust primitive crate building | cargo_build_release, strip_binaries, copy_to_bin |
| **lib-agents.sh** | Agent manifest + generated .md setup | copy_agents, generate_agents, verify_agent_dna |
| **lib-hooks.sh** | Hook installation + registration | install_hooks, register_hook_event, test_hook |
| **lib-bridges.sh** | Cross-tool bridge generation | generate_cursorrules, generate_windsurf_rules, generate_github_copilot |
| **lib-scaffold.sh** | Workspace scaffolding (dirs, templates, config) | create_workspace, setup_config_toml, init_memory_db |
| **lib-wizard.sh** | Interactive setup wizard | show_welcome, ask_profile, ask_features, summary |
| **lib-plan.sh** | Install plan builder + preview | build_plan, show_plan, confirm_plan |
| **lib-dev-hub-*.sh** | Optional dev-hub tools | restic (backup), mdbook (docs), gdrive-import (foreign-project), datasette (SQL UI) |

**Key entry points:**
- `install.sh` — main orchestrator; sources lib-*.sh files and runs pathway
- `install.sh --profile=minimal` — install without optional tools
- `install.sh --profile=full --with-cortex` — full suite + Cortex stack
- `install.sh --dry-run` — preview without mutating filesystem

---

## 8. Manifest Block References

The `_blocks/` directory contains reusable prompt fragments included via TOML `blocks = [...]` list in manifests. Examples:

| Block Name | Used By | Content |
|------------|---------|---------|
| **baseline** | ALL manifests (obligatory) | RULE -1, RULE 0.1, RULE 0.2, RULE 0.4, RULE 0.5, RULE ZERO, KARPATHY (full text, never summarized) |
| **evidence-grading** | ALL manifests (obligatory) | E1-E6 grading table + rules for claims |
| **memory-protocol** | ALL manifests (obligatory) | 3-layer memory architecture + session save protocol |
| **rule-pre-dev-gate** | code-implementer* | Pre-Dev Gate (analogues, stack compat, duplication checks) |
| **rule-test-first** | code-implementer* | Test-First discipline (TDD for critical, alongside for rest) |
| **rule-error-budget** | code-implementer* | 3-Level Escalation (2 fails → review, 3 → research) |
| **rule-double-audit** | code-implementer*, critic* | Phase 1/2/3/4 double-audit protocol |
| **rule-no-patching** | code-implementer* | Root-cause fixes (never overlay patches) |
| **rule-constructor-pattern** | code-implementer*, infra-impl* | Constructor Pattern enforcement + split criteria |
| **rule-no-hallucination** | researcher*, validator* | RULE 0.4 citation verify + confidence grading |
| **rule-git-conventions** | infra-impl, git-ops | Commit types + timing SSoT |
| **rule-api-cost-guard** | cost-guardian, modal-runner | Dashboard check, price estimation, batch caution |
| **rule-security-review** | security-auditor* | Risk classification + 9-point differential review |

**Discovery:** blocks are indexed in `_blocks/blocks.toml` (auto-generated by assembler) with versions + keywords for kei-sage search.

---

## 9. Capability Text Fragments

Each capability has a `text.md` fragment (≤ 200 words) that is concatenated into the final agent prompt. Examples:

### policy::no-git-ops (text.md)

```markdown
## No git operations

You MUST NOT invoke git, gh repo, gh api /repos, or any shell command
that modifies git state. Orchestrator handles all git operations
(commits, branches, pushes, rebases).

If your task requires staging a change, describe it in the return
file-list — the orchestrator will commit on your behalf.

Bypass exists for orchestrator-meta agents only; it is not available here.
```

### quality::constructor-pattern (text.md)

```markdown
## Constructor Pattern compliance

Every file you write or edit MUST stay under 200 lines of code.
Every function MUST stay under 30 lines of code. No exceptions.

If your change pushes a file past 200 LOC or a function past 30 LOC,
split it on the spot. Never commit with "TODO: refactor later".

Comments, blank lines, and use statements count toward LOC —
the verifier counts lines as wc -l sees them.
```

These are concatenated in role-declared order, with `\n\n---\n\n` separators between fragments.

---

## 10. Runtime Execution Flow

**At agent spawn time:**
```
1. Orchestrator: kei-agent-runtime compose <task.toml> <role> → prompt.md
   - Reads role + ordered capability list
   - Concatenates blocks + capability text.md fragments
   - Outputs final prompt to stdout / file
2. Orchestrator: Agent({ prompt, model, tools }) ← Claude Code Agent tool
3. Agent runs with PreToolUse hooks wired to kei-capability check
```

**On each tool call:**
```
4. Claude Code: PreToolUse:Bash → calls hook
5. Hook: exec kei-capability check --capability "tools::cargo-only-bash" < <stdin-JSON>
6. kei-capability: dispatch to Rust gate impl
7. Gate: Allow | Deny | NotApplicable
8. Hook: exit 0 (allow) | exit 2 (deny) | return
9. Tool call proceeds or fails
```

**On agent return:**
```
10. Agent emits final report with file list + status-truth-marker
11. Orchestrator: kei-capability verify --capability "quality::cargo-check-green" \
      --worktree /path --mode simulated-merge
12. kei-capability: dispatch to Rust verify impl
13. Verify: run `cargo check` in worktree OR on temp merge branch
14. Verify: Pass | Fail {reason}
15. Orchestrator: if any verify fails, block merge + report findings
16. User: approves merge or requests fixes (via return questionnaire)
```

---

## 11. Substrate Invariants

| Invariant | Enforced By | Failure Mode |
|-----------|-------------|--------------|
| Manifests declare role + handoffs | assembler `validate-manifest` + CI | spawn rejected; agent unknown |
| Role names match _roles/*.toml | assembler `validate-role` + CI | manifest ref error |
| Capabilities exist + have both toml + text.md | assembler `validate-capability` + CI | compose fails; missing fragments |
| JSON Schemas are valid draft-07 | `kei-schema-lint` + CI | atom discovery skips malformed schema |
| Gates/verifies registered in kei-agent-runtime | `cargo test --all` | gate lookup fails; aborting capability |
| Agent DNA is 80 chars unique per invocation | kei-ledger fork + DNA-INDEX.md | collision risk; query ambiguity |
| No file edits outside files-whitelist | scope::files-whitelist gate | merge blocked; scope violation |
| Files ≤ 200 LOC, functions ≤ 30 LOC | quality::constructor-pattern verify | merge blocked; pattern violation |
| cargo check + tests green | quality::cargo-check-green + tests-green | merge blocked; integration failure |
| No Cargo.toml [dependencies] edits | safety::no-dep-bump gate | merge blocked; supply-chain review pending |
| STATUS-TRUTH MARKER present | output::merge-result verify | merge blocked; shipped-vs-functional drift flagged |

---

## 12. Key Concepts Glossary

| Term | Definition | Example |
|------|-----------|---------|
| **Atom** | One verb operation on a primitive; independently composable | `kei-task::create`, `kei-task::search` |
| **Capability** | Declarative bundle (TOML + text.md + Rust impl) that enforces agent constraint | `quality::constructor-pattern`, `scope::files-whitelist` |
| **Block** | Reusable prompt fragment referenced by multiple manifests | `rule-pre-dev-gate`, `baseline` |
| **Manifest** | Agent spec declaring role, tools, domain_in/out, handoffs | `code-implementer.toml`, `ml-implementer.toml` |
| **Role** | Bundle of capabilities + tool allowlist + escalation policy | `edit-local`, `read-only`, `git-ops` |
| **Gate** | PreToolUse Rust impl blocking tool calls (deny) | `policy::no-git-ops` blocks `^git` |
| **Verify** | On-return Rust impl validating agent output + integration | `quality::cargo-check-green` runs `cargo check --workspace` |
| **DNA** | 80-char deterministic agent identity; "did this run before?" without embeddings | `code-impl::edit-local::7f1a2c3d::5e9b4f2a-nonce` |
| **Simulated merge** | Orchestrator creates test branch, applies agent diff, runs checks from there | catches integration regressions pre-merge |
| **Hands off** | Agent delegates work to another agent via manifest `[[handoff]]` table | `code-implementer` hands off to `ml-implementer` |

---

## 13. Status & Completeness

**As of 2026-05-02:**

| Layer | Stability | Notes |
|-------|-----------|-------|
| Manifests (38 agents) | stable | All roles + models declared; DNA format frozen |
| Capabilities (18 atoms) | stable | All gates + verifies implemented; tests green |
| Roles (7 roles) | stable | edit-local / read-only / git-ops / etc. locked; no churn |
| Assembler (compose logic) | stable | Generates .md from TOML + blocks; keimd integration active |
| Cortex stack | beta | kei-cortex (HTTP) + kei-tty (TUI) build clean; browser/VSCode frontends concept |
| MCP Server (@keisei/mcp-server) | alpha | Exports Rust primitive CLIs as MCP tools; published to keigit.com (Forgejo npm registry, public DNS) under @keisei scope |
| Bridges | stable | 11 cross-tool format generators (.cursorrules, .windsurf/rules, GEMINI.md, etc.) |
| Sleep Layer (Phase A/B/C) | stable | Incubation (tasks), REM consolidation (reports), NREM deep-sleep (conflicts) |
| Foreign-project ingestion | stable | kei-import <repo> proof-of-concept via Hermes validation |
| Nightly consolidation | active | Running 2026-05-02; Phase A + B + C observed; reports 495 DNA indices |

**Roadmap:**
- Model router (Bayesian posterior, currently manual routing)
- Hosted Phase B/C (cross-machine memory sync)
- Encyclopedia-as-API (query substrate by DNA / role / capability)
- Browser + VSCode frontends for Cortex

---

## References

- `docs/SUBSTRATE-SCHEMA.md` — atom + capability SSoT
- `docs/AGENT-SUBSTRATE-SCHEMA.md` — agent gate/verify triplet spec
- `docs/ARCHITECTURE.md` — implementation trait matrix
- `docs/DNA-INDEX.md` — agent identity format
- `.claude/rules/orchestrator-branch-first.md` (RULE 0.13) — agent git model
- `.claude/rules/agent-git-model.md` (RULE 0.12) — fork/ledger lifecycle
- `_primitives/_rust/kei-agent-runtime/src/` — gate + verify impls
- `_primitives/_rust/kei-registry/` — atom discovery + indexing
- `_assembler/src/` — manifest → agent.md composition
