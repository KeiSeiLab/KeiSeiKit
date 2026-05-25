# KeiSeiKit Hooks & Blocks Catalogue

Comprehensive index of all 55 hooks and 85 blocks in KeiSeiKit-public as of 2026-05-02.

**Structure:**
- **Hooks (55):** Claude Code safety + observability rules, wired into `.claude/settings.json` hook events (PreToolUse, PostToolUse, UserPromptSubmit, Stop).
- **Blocks (85):** Reusable architectural patterns covering API design, auth, databases, deployment, security, stacks, testing, rules, and domain-specific practices.

---

## Part 1: Hooks Reference

All hooks live under `hooks/` directory. Format: `| Hook Name | Event | Severity | Purpose | Bypass Env |`

**Event types:**
- **PreToolUse:Bash** — called before any shell command
- **PreToolUse:Edit|Write** — called before modifying files
- **PreToolUse:Agent** — called before spawning sub-agent
- **PostToolUse:Agent** — called after agent returns
- **PostToolUse:Bash** — called after shell command completes
- **UserPromptSubmit** — called when user submits a prompt
- **Stop** — called when session ends

**Severity levels:**
- **block (exit 2)** — Claude Code aborts the tool call
- **enforce (exit 1)** — error; must fix before retrying
- **warn (exit 0 + stderr)** — advisory message, tool call proceeds
- **remind (exit 0 + stderr on trigger)** — passive reminder
- **advisory** — informational, never blocks

### Core Safety Hooks

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| no-github-push.sh | PreToolUse:Bash | block | Block accidental push / repo-create to github.com (opt-in; for code kept on a private remote) | KEI_NO_GITHUB_PUSH_BYPASS |
| no-python-without-approval.sh | PreToolUse:Bash | block | Optional Rust-first policy — Python requires explicit justification (opt-in, stack-gated) | none |
| rust-first.sh | UserPromptSubmit | remind | Remind about Rust-first default for new work | none |
| secrets-pre-guard.sh | PreToolUse:Edit\|Write | block | Detect hardcoded API keys, tokens, private keys before commit | KEI_SECRETS_GUARD_BYPASS |
| destructive-guard.sh | PreToolUse:Bash | block | Block dangerous commands (rm -rf /, git reset --hard main, truncate) | none |
| tomd-preread.sh | PreToolUse:Read | warn | Auto-convert .docx, .xlsx, .pptx to .md before reading | none |

### Disk & Resource Management

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| disk-headroom-check.sh | PreToolUse:Bash | block/warn (tiered) | RULE 0.17: 4-tier ladder (≥20G silent, 10-20 warn, 5-10 warn+suggest, 2-5 block-heavy, <2 hard-block) | DISK_GUARD_BYPASS |
| disk-reclaim.sh | launchd 03:30 | advisory | Nightly cleanup: orphan worktrees + stale target/ dirs (168h+ old, clean, unpushed, no live PID) | none |

### Numeric Claims & Evidence (RULE 0.18)

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| numeric-claims-guard.sh | PreToolUse:Edit\|Write | enforce | Block time/cost/effort claims without `[REAL:]` / `[FROM-JOURNAL:]` / `[ESTIMATE-HTC:]` marker | RULE_017_BYPASS |
| numeric-claims-record.sh | Post-Write | block | Auto-log numeric claims to `memory/time-metrics/*.jsonl` | NUMERIC_CLAIMS_RECORD_BYPASS |
| chat-numeric-prewarn.sh | UserPromptSubmit | remind | Inject RULE 0.18 reminder when user prompt contains time/cost keywords | RULE_018_CHAT_BYPASS |
| chat-numeric-postflag.sh | Stop | warn | Flag any bare numerics in last assistant message without evidence marker | RULE_018_CHAT_BYPASS |

### Agent Lifecycle & Status Truth (RULE 0.12, 0.16)

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| agent-fork-logger.sh | PreToolUse:Agent | advisory | Log agent fork to ledger (RULE 0.12) — non-blocking, silent on missing kei-ledger | none |
| agent-event-spawn.sh | PreToolUse:Agent | block | Record parent-child agent relationship in /tmp/kei-active-children.tsv | KEI_EVENTS_BYPASS |
| agent-event-done.sh | PostToolUse:Agent | block | Mark agent as done in active-children ledger | KEI_EVENTS_BYPASS |
| agent-fork-done.sh | PostToolUse:Agent | block | Transition ledger row from 'running' → 'done' or 'fail' (pairs with agent-fork-logger) | none |
| agent-outcome-backfill.sh | PostToolUse:Agent | block | Parse STATUS-TRUTH MARKER from agent transcript + backfill shipped/stubs/cargo-check/behaviour-verified (RULE 0.16) | OUTCOME_BACKFILL_BYPASS |
| agent-stub-scan.sh | PostToolUse:Agent | enforce | RULE 0.16: validate shipped=functional ⟹ stubs=0 consistency; 7-day WARN → then ENFORCE | STATUS_TRUTH_BYPASS |
| agent-heartbeat-tick.sh | PostToolUse:\* | advisory | Heartbeat: auto-update `progress.json` in agent's `.claude/agents/<id>/` dir every 30s | KEI_PING_BYPASS |
| tool-use-event.sh | PostToolUse:\* | block | Flatten tool response + attribute to parent agent via active-children ledger | KEI_EVENTS_BYPASS |
| extract-task-durations.sh | Pre/Post Agent | block | Record agent start/end times in `memory/time-metrics/tasks.jsonl` | none |
| task-timer.sh | Pre/Post Agent + Stop | block | Master timer: compute wall-clock duration per task + session | none |

### Code Quality & Architecture

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| citation-verify.sh | PreToolUse:Edit\|Write | block | RULE 0.4: block academic citations without `[VERIFIED: url]` or `[UNVERIFIED]` marker | none |
| auto-dev-guard.sh | PreToolUse:Edit | warn | Warn on frontend (.tsx/.ts/.svelte/.vue/.dart) or DB-layer changes without corresponding tests | none |
| auto-register-on-edit.sh | PostToolUse:Edit | block | When substrate file (skill/hook/block/capability/role) edited → auto-register in manifest registry | AUTO_REGISTER_BYPASS |
| auto-encyclopedia-refresh.sh | PostToolUse:Edit | warn | Refresh docs/encyclopedia/ after substrate file changes (pairs with auto-register-on-edit) | AUTO_ENCYCLOPEDIA_BYPASS |
| no-hand-edit-agents.sh | PreToolUse:Edit | block | Prevent hand-editing `.claude/agents/<id>/` — edit manifest instead (RULE 0.12) | none |
| post-write-check.sh | PostToolUse:Write | warn | Async check: warn on large files (>5MB) + hardcoded secrets | none |
| post-commit-audit.sh | PostToolUse:Bash (git commit) | remind | Remind about double audit after commit | none |
| decompose-rules-on-edit.sh | PreToolUse:Edit (~/.claude/rules/\*.md) | warn | When rule file edited → re-run kei-decompose to update RULES.md registry | DECOMPOSE_RULES_BYPASS |

### Observability & Session State

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| session-end-dump.sh | Stop | block | Call `kei-memory ingest` (populate traces/) + `kei-sleep-sync.sh` (push to memory-repo) | none |
| skill-record.sh | PostToolUse:Agent | block | Record skill invocation for analytics (never blocks — exit 0 always) | SKILL_RECORD_BYPASS |
| tool-use-event.sh | PostToolUse:\* | block | Event logging for agent attribution + wall-clock tracking | KEI_EVENTS_BYPASS |
| check-error-patterns.sh | PostToolUse:\* | block | Detect error spikes (≥3 in last 20 calls) → append to audit-backlog.md | none |
| error-spike-detector.sh | PostToolUse:\* | block | Rolling 20-call error window; on spike → log pattern + suggest escalation | none |
| milestone-commit-hook.sh | PostToolUse:Bash (git commit) | block | On feat:/refactor:/merge → run `kei-memory analyze` + append to audit-backlog.md | none |
| alignment-check.sh | (internal) | block | Track 3-time recurrence: exp6, exp24-28, basecaller forgot alignment | none |
| graph-export-watcher.sh | PreToolUse:Edit | warn | When keimd graph file edited → watch for drift | GRAPH_EXPORT_BYPASS |
| site-wysiwyd-check.sh | PostToolUse:Edit | advisory | After frontend edit → WYSIWYG drift check against dev-server | none |

### Sleep Layer & Cloud Agent Triggers (RULE 0.15)

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| sleep-report-tg.sh | (nightly Phase B remote) | block | Cloud agent reads `reports/sleep-*.md` + sends Telegram summary | SLEEP_REPORT_TG_BYPASS |
| phase-b-rem.sh | (nightly Phase B REM consolidation) | block | Run `kei-conflict-scan` + `kei-refactor-engine` for NREM deep-sleep (v0.13) | none |
| affect-live-scan.sh | Stop | remind | Pairs with session-end-dump — writes affect file for pattern analysis | AFFECT_LIVE_BYPASS |

### Assembly & Manifest Management

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| assemble-agents.sh | PostToolUse:Edit (_manifests/\*.toml) | block | When agent manifest edited → rebuild that agent crate | none |
| assemble-validate.sh | PostToolUse:Edit (_manifests/) | block | Validate all manifests on edit; failure → block commit | none |
| no-hand-edit-agents.sh | PreToolUse:Edit (.claude/agents/) | block | Prevent hand-editing agent directories | none |

### Safety & Enforcement

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| stop-verify.sh | Stop | advisory | Check for uncommitted changes + running Modal compute before session ends | none |
| orchestrator-dirty-check.sh | UserPromptSubmit | advisory | Warn if orchestrator has uncommitted agent output from prior waves | none |
| orchestrator-branch-check.sh | PreToolUse:Agent | remind | Remind that orchestrator should be on feat/\* branch (not main) before spawning agent | ORCHESTRATOR_META |
| block-dangerous.sh | (internal) | block | Block dangerous patterns (defined inline) | none |
| safety-guard.sh | (internal) | block | Generic safety pattern matcher | none |

### Specialized Domain Hooks

| Hook | Event | Severity | Purpose | Bypass Env |
|------|-------|----------|---------|-----------|
| no-downgrade.sh | PreToolUse:Edit\|Write | enforce | RULE -1: when problem found in Edit/Write, require 3+ solution paths (not defeatism) | none |
| recurrence-suggest.sh | UserPromptSubmit | remind | RULE 0.10: detect same mistake ≥2× in session → suggest `/escalate-recurrence` skill | none |
| agent-capability-check.sh | (internal) | block | Check agent capability before spawn | none |
| agent-capability-verify.sh | (internal) | block | Verify agent claimed capability matches manifest | none |

---

## Part 2: Blocks Reference

All blocks live under `_blocks/` directory. Format: `| Block Name | Category | Purpose |`

**Categories:**
- **api** — HTTP/API design, versioning, conventions
- **auth** — authentication, authorization, sessions
- **ci** — continuous integration + release
- **db** — databases, migrations, SQL
- **deploy** — cloud/VPS deployment targets
- **docs** — documentation conventions
- **domain** — domain-specific rules (ML, secrets, paid APIs)
- **evidence** — grading framework
- **memory** — session memory protocol
- **mode** — reasoning modes (first-principles, maximalist, skeptic, etc.)
- **obs** — observability (metrics, logs, traces)
- **path** — file system paths
- **pipeline** — multi-phase project templates
- **rule** — development discipline rules
- **scraper** — web scraping tiers + patterns
- **security** — hardening, patching, audit logging
- **stack** — tech stack reference patterns
- **test** — testing strategies

### API & Integration (8 blocks)

| Block | Purpose |
|-------|---------|
| api-anthropic.md | Anthropic API integration (models, tokens, rate limits) |
| api-apify.md | Apify web scraping API (actors, storage, datastores) |
| api-elevenlabs.md | ElevenLabs TTS API (voices, models, billing) |
| api-fal-ai.md | fal.ai image generation API (models, async webhooks) |
| api-graphql.md | GraphQL design conventions (schema, queries, subscriptions) |
| api-openapi-first.md | OpenAPI/Swagger as SSoT for API contracts |
| api-rest-conventions.md | REST verbs, status codes, resources, idempotency, ETags (RFC 9110, 9457) |
| api-versioning-pagination-ratelimit.md | API versioning strategy, cursor/offset pagination, rate-limit headers |

### Authentication & Authorization (4 blocks)

| Block | Purpose |
|-------|---------|
| auth-authorization.md | RBAC, ABAC, scopes, permission delegation |
| auth-oauth2-oidc.md | OAuth 2.0 / OpenID Connect flows (authz code, PKCE, ID tokens) |
| auth-passkeys.md | WebAuthn / FIDO2 passkey registration + assertion |
| auth-sessions.md | Session management (secure cookies, CSRF tokens, refresh tokens) |

### CI/CD & Release (3 blocks)

| Block | Purpose |
|-------|---------|
| ci-forgejo-actions.md | Forgejo Actions (internal runner, workflow.yml, secrets) |
| ci-github-actions.md | GitHub Actions workflows (matrix, artifacts, deployment) |
| ci-release-automation.md | Semantic versioning, changelog, auto-release tags |
| ci-security-gate.md | Pre-commit scanning (secrets, CVE, licenses) |

### Database (5 blocks)

| Block | Purpose |
|-------|---------|
| db-drizzle.md | Drizzle ORM (schema, relations, migrations in TypeScript) |
| db-migration-hygiene.md | Reversible migrations, zero-downtime deployments, test safety |
| db-postgres.md | PostgreSQL (setup, performance tuning, connection pooling) |
| db-sqlite.md | SQLite (embedded, dev fixtures, backup strategy) |
| db-sqlx.md | sqlx Rust macro (compile-time checked SQL, migrations) |

### Deployment (7 blocks)

| Block | Purpose |
|-------|---------|
| deploy-aws-ec2.md | AWS EC2 instances, security groups, Elastic IPs |
| deploy-cloudflare.md | Cloudflare Workers, KV, R2, Pages (edge compute + CDN) |
| deploy-docker.md | Docker image best practices, multi-stage builds, scanning |
| deploy-hetzner-cloud.md | Hetzner Cloud VPS (volume, networking, backup) |
| deploy-local-only.md | Local/private deployment (Tailscale, no public IP) |
| deploy-modal.md | Modal.com GPU cloud (Functions, Volumes, checkpointing, KILL GUARD) |
| deploy-vps-generic.md | Generic VPS pattern (Systemd, Caddy, SSH hardening) |

### Documentation (5 blocks)

| Block | Purpose |
|-------|---------|
| docs-architecture-diagrams.md | Mermaid, C4, ADR diagrams for architecture |
| docs-claude-md.md | CLAUDE.md project-specific instructions + umbrella rules |
| docs-decisions-adr.md | Architecture Decision Records (format, storage, reversals) |
| docs-readme-template.md | Minimal README structure (goal, quickstart, docs link) |
| docs-runbook.md | Operational runbooks for prod incidents, backup/restore, scaling |

### Domain-Specific Rules (3 blocks)

| Block | Purpose |
|-------|---------|
| domain-has-secrets.md | Projects with credentials / keys / auth (setup, RULE 0.8) |
| domain-ml-training.md | ML experiment discipline (pre-reg, math-first, observability) |
| domain-paid-apis.md | Cost guard for Modal, AWS, fal.ai, Apify billing |

### Evidence & Grading (1 block)

| Block | Purpose |
|-------|---------|
| evidence-grading.md | E1-E6 evidence grades (E1=fact, E6=speculation) |

### Memory & Session (1 block)

| Block | Purpose |
|-------|---------|
| memory-protocol.md | 3-layer memory architecture (CLAUDE.md → memory/{project}.md → MEMORY.md index) |

### Reasoning Modes (6 blocks)

| Block | Purpose |
|-------|---------|
| mode-devils-advocate.md | Challenge assumptions; find counter-examples |
| mode-first-principles.md | Derive from axioms, question inheritance |
| mode-matrix.md | Multi-dimensional tradeoff analysis (2D/3D matrices) |
| mode-maximalist.md | Add all features; find essential ones via ablation |
| mode-minimalist.md | Start with one thing; add only if it changes behavior |
| mode-skeptic.md | Assume negative; require evidence |

### Observability (3 blocks)

| Block | Purpose |
|-------|---------|
| obs-metrics.md | Prometheus/StatsD metrics, cardinality, alerting |
| obs-structured-logs.md | JSON logs, correlation IDs, log levels |
| obs-traces.md | Distributed tracing (OpenTelemetry, span attributes, sampling) |

### Paths & Configuration (3 blocks)

| Block | Purpose |
|-------|---------|
| path-user-hooks.md | User-level hook paths (`~/.claude/hooks/`) |
| path-user-memory.md | User-level memory paths (`~/.claude/memory/`) |
| path-user-rules.md | User-level rules paths (`~/.claude/rules/`) |

### Pipeline Templates (1 block)

| Block | Purpose |
|-------|---------|
| pipeline-5phase-template.md | 5-phase multi-agent orchestration (Setup, Foundations A-B, Implementation, Testing, Deploy) |

### Rules & Discipline (6 blocks)

| Block | Purpose |
|-------|---------|
| rule-double-audit.md | Phase 1 (find) → Phase 2 (verify) → Phase 3 (report) → Phase 4 (fix) |
| rule-error-budget.md | 3-level escalation: attempt 2 fail → review; attempt 3 fail → research; stuck → escalate |
| rule-math-first.md | Derive prediction before numerics; ask "what is unnecessary?" |
| rule-pre-dev-gate.md | Analogues check, stack compatibility, duplication check before coding |
| rule-pure-click-contract.md | Skill response = pure clicks (AskUserQuestion) — never free-text input |
| rule-test-first.md | Critical paths: tests before code (TDD); rest: tests with code |

### Scraping (3 blocks)

| Block | Purpose |
|-------|---------|
| scraper-free-tier.md | YouTube API v3, Telegram Telethon, GitHub API, Twitter twscrape |
| scraper-paid-tier.md | Apify, Bright Data (fallback, cost-guarded) |
| scraper-unified-output.md | UnifiedProfile / UnifiedContent normalizer across all sources |

### Security (5 blocks)

| Block | Purpose |
|-------|---------|
| security-audit-logging.md | Audit trail logging (who, what, when, where, result) |
| security-firewall-ufw.sh | UFW firewall rules (inbound/outbound, rate-limit) |
| security-patching.md | OS + dependency patching cadence (monthly, emergency) |
| security-ssh-hardening.md | SSH config (key-only, no password, limited users, audit logging) |
| security-tls-caddy.md | Caddy reverse proxy (Let's Encrypt ACME, auto-renewal, HTTPS redirect) |

### Tech Stacks (14 blocks)

| Block | Purpose |
|-------|---------|
| stack-astro.md | Astro.build SSR framework (components, islands, integrations) |
| stack-embedded-stm32.md | STM32 microcontroller (HAL, firmware, JTAG debug) |
| stack-fastapi-postgres.md | FastAPI + PostgreSQL (asyncio, SQLAlchemy async, dependency injection) |
| stack-flutter.md | Flutter mobile + web (Riverpod state, Clean Architecture, testing) |
| stack-go-server.md | Go HTTP server (chi router, context, middleware, graceful shutdown) |
| stack-nextjs.md | Next.js 14+ app router (server components, API routes, middleware) |
| stack-python-ml.md | Python ML (PyTorch, scikit-learn, wandb logging, reproducibility) |
| stack-react-vite.md | React + Vite (SPA, hooks, component patterns) |
| stack-rust-axum.md | Rust Axum web framework (extractors, middleware, error handling) |
| stack-rust-cli.md | Rust CLI (clap, error handling, testing, benchmarking) |
| stack-sveltekit.md | SvelteKit (reactive components, server load functions, API routes) |
| stack-swift-ios.md | Swift iOS (SwiftUI, state management, networking) |
| stack-swift-spm.md | Swift Package Manager (library, executable, tests, dependencies) |
| stack-tailwind.md | Tailwind CSS (utility-first, JIT, dark mode, component plugins) |

### Testing (4 blocks)

| Block | Purpose |
|-------|---------|
| test-e2e.md | End-to-end testing (Playwright, browser automation, visual regression) |
| test-fuzz.md | Fuzzing / property-based testing (cargo-fuzz, proptest) |
| test-load.md | Load testing (k6, locust, capacity planning) |
| test-property.md | Property-based testing (invariants, shrinking, counterexamples) |

### Baseline (1 block)

| Block | Purpose |
|-------|---------|
| baseline.md | Inherit from ~/.claude/CLAUDE.md — NO DOWNGRADE, NO HALLUCINATION, PLAN MODE FIRST, CONSTRUCTOR PATTERN, THINK BEFORE CODE, SURGICAL CHANGES, GOAL-DRIVEN |

---

## Summary Stats

**Hooks:** 55 total
- **block (exit 2):** ~18 hooks (hard deny)
- **enforce (exit 1):** ~6 hooks (error state)
- **warn (exit 0 + stderr):** ~15 hooks (advisory + proceed)
- **remind / advisory:** ~16 hooks (passive)

**Blocks:** 85 total
- **API & Integration:** 8
- **Auth:** 4
- **CI/CD:** 4
- **Database:** 5
- **Deployment:** 7
- **Documentation:** 5
- **Domain:** 3
- **Evidence:** 1
- **Memory:** 1
- **Modes:** 6
- **Observability:** 3
- **Paths:** 3
- **Pipelines:** 1
- **Rules:** 6
- **Scraping:** 3
- **Security:** 5
- **Stacks:** 14
- **Testing:** 4
- **Baseline:** 1

---

## Usage Examples

### How to add a new hook

1. Write `hooks/my-hook.sh` with shebang `#!/bin/sh` or `#!/bin/bash`
2. Include event name in first 3 comment lines: `# my-hook.sh — PreToolUse:Edit advisory hook`
3. Define bypass env at top: `if [ "${MY_HOOK_BYPASS:-0}" = "1" ]; then exit 0; fi`
4. Exit with appropriate code (0, 1, or 2)
5. Register in `.claude/settings.json` under `hooks.PreToolUse` / `hooks.PostToolUse` / etc.

### How to use a block

1. Reference the block in your project CLAUDE.md: `See [[../../../_blocks/api-rest-conventions]].`
2. The block is a living template — copy relevant sections, adapt to your project.
3. Blocks are shared across KeiSeiKit projects; do NOT fork them.
4. Update a block if you discover a better practice → file a PR to main.

---

**Maintained by:** KeiSeiKit orchestrator
**Last updated:** 2026-05-02
**Source:** `_blocks/` and `hooks/` directories
