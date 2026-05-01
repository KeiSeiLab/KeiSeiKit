# Phase 2 — Propose Candidates

Analyse `SCAN` from Phase 1 and produce `CANDIDATES` — a list of proposed
agents, hooks, and primitives with confidence scores. Zero AskUserQuestion
calls in this phase (it's pure analysis; Phase 3 does the mode pick).

## 2a — Candidate kinds

For each project in `PATHS`, emit up to:

- **1 agent** — a project-specialist (maybe 0 if the project is too generic
  or already has a specialist)
- **M hooks** — stack-specific enforcement (0-3 typical)
- **K primitives** — install-queued shell/rust helpers (0-5 typical)

## 2b — Agent proposal (per project)

Compose a dry-run `/new-agent` input based on the scan:

- **Slug**: derive from `basename $PATH` lowercased, kebab-case
- **Proposed name**: `kei-<slug>-specialist`
- **Stack block** (from `_blocks/stack-*.md`):
  - `package.json` + Next.js dep → `stack-nextjs`
  - `package.json` + React + Vite → `stack-react-vite`
  - `package.json` + SvelteKit → `stack-sveltekit`
  - `package.json` + Astro → `stack-astro`
  - `Cargo.toml` + `[[bin]]` + axum → `stack-rust-axum`
  - `Cargo.toml` + `[[bin]]` no axum → `stack-rust-cli`
  - `pyproject.toml` + FastAPI → `stack-fastapi-postgres`
  - `pyproject.toml` + ML deps (torch/jax) → `stack-python-ml`
  - `pubspec.yaml` → `stack-flutter`
  - `go.mod` → `stack-go-server`
  - `Package.swift` + macOS → `stack-swift-spm`
  - `Package.swift` + iOS → `stack-swift-ios`
  - STM32/ESP32 toolchain hint → `stack-embedded-stm32`
  - None detected → **confidence=speculative**, do not propose stack block
- **Deploy block**:
  - `docker-compose*.yml` → `deploy-docker`
  - `Dockerfile` alone → `deploy-docker`
  - `wrangler.toml` → `deploy-cloudflare`
  - `modal.toml` or `Modal` string in code → `deploy-modal`
  - AWS EC2 hint (README/infra/terraform) → `deploy-aws-ec2`
  - None detected → skip deploy block
- **Conditional domain blocks**:
  - Env vars include `*_API_KEY` / `*_TOKEN` (paid-API names) →
    `domain-paid-apis`
  - ML stack detected → `domain-ml-training` + `rule-math-first`
  - `secrets/*.env.example` present → `domain-has-secrets`
- **Handoffs** (from kit-12 set, verify each exists via
  `ls ~/.claude/agents/_manifests/kei-*.toml` or
  `ls _manifests/kei-*.toml`):
  - always: `kei-code-implementer`, `kei-critic`, `kei-validator`
  - paid-APIs → `kei-cost-guardian`
  - ML → `kei-ml-implementer` + `kei-ml-researcher`
  - deploy detected → `kei-infra-implementer`
  - Rust/Swift/Go → `kei-security-auditor`

Confidence rubric (E3-E4 range — scan-derived, never E1):

- **high**: manifest file + dep signature both match (e.g. `Cargo.toml` +
  `axum = "..."` → `stack-rust-axum` high)
- **medium**: manifest file matches, no dep signature (e.g. `Cargo.toml`
  with no crate hint → `stack-rust-cli` medium)
- **speculative**: only weak signal (README prose, dir name)

## 2c — Hook proposal (per project)

Map scan → hook suggestions:

- Python detected → `no-python-without-approval` bypass hint (confidence
  high if `pyproject.toml` present; pattern already exists at
  `~/.claude/hooks/no-python-without-approval.sh`). Propose: "Document the
  RULE 0.2 exception for this project in its CLAUDE.md; hook stays global."
- Rust detected → propose `cargo-check-preedit` (pre-edit hook running
  `cargo check --message-format=short`). If it doesn't exist on disk, it's
  a CREATE via `/escalate-recurrence`.
- TypeScript detected (`tsconfig.json` + `.ts` files) → propose
  `tsc-on-save` equivalent (PostToolUse:Edit running `tsc --noEmit`).
- Go detected → propose `gofmt-check` PreToolUse:Edit.
- Flutter detected → propose `flutter-analyze-precommit` hint.
- CI files present → propose `kei-ci-lint` PostToolUse:Edit (verifies
  workflow YAML on every edit; primitive `kei-ci-lint.sh` exists).

For each hook candidate:

- Verify the pattern already exists: `ls ~/.claude/hooks/<name>.sh` or the
  Rust binary under `~/.claude/hooks/_rust/<name>/`.
- If exists → confidence high, action = "document / enable".
- If not exists → confidence medium, action = "delegate to
  `/escalate-recurrence` to author".

## 2d — Primitive proposal (per project)

Read `_primitives/MANIFEST.toml` (already on disk). Map scan → primitives:

- CI detected → `kei-ci-lint` (high confidence)
- Doc heavy (`docs/`, many `.md`) → `kei-docs-scaffold` (medium)
- DB migrations (schema files, `migrations/` dir) → `kei-migrate` (high)
- Frontend + live preview needed → `live-preview`, `design-scrape`,
  `frontend-inspect`, `screenshot-decode` (bundle as `frontend` profile
  suggestion)
- Ops / VPS → `provision-hetzner`, `provision-vultr`, `harden-base` (ops
  profile suggestion)
- Non-native docs (.docx/.xlsx/.pptx) in repo → `tomd` (high)

For each primitive candidate:

- Verify it exists in `MANIFEST.toml` via grep.
- Recommend install mode:
  - `install.sh --add=<primitive>` (one-off)
  - `install.sh --profile=<profile>` (if multiple primitives in same
    profile)
  - `kei-sleep-queue add` (if the user wants it queued for a later sleep
    session — useful for big installs)

## 2e — Output structure

Emit a structured summary (display only — no file write):

```
## Candidates for <project-path>

### Agent (1 proposal)
- kei-<slug>-specialist  [confidence: high]
  Blocks: baseline, evidence-grading, memory-protocol, rule-pre-dev-gate,
          <stack>, <deploy>, <domain-blocks>
  Handoffs: <list>
  Rationale: <1-2 lines tied to scan evidence>

### Hooks (M proposals)
- <hook-name>   [high | medium | speculative]
  Action: document / enable | delegate to /escalate-recurrence
  Rationale: <scan evidence>

### Primitives (K proposals)
- <primitive>   [confidence]
  Install:  install.sh --add=<primitive>
  Rationale: <scan evidence>
```

Store the full list as `CANDIDATES` for Phase 4 consumption.

## Verify-criterion

- Every proposed block name exists under `_blocks/` (ls-verify before
  citing).
- Every proposed handoff target exists under `_manifests/kei-*.toml`.
- Every proposed primitive exists in `_primitives/MANIFEST.toml`.
- Confidence scores are present on every candidate.
- No candidate invented from thin air — rationale must cite one or more
  scan lines from Phase 1.
- Zero candidates is a valid result — if scan produced nothing actionable,
  emit an empty list with a note ("scan was inconclusive; recommended
  action: Phase 3 → pick Full manual to walk `/new-agent` yourself").
