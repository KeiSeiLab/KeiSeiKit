# KeiSeiKit blocks registry

> SSoT for the assembler. All blocks available for `blocks = [...]` in `_manifests/<agent>.toml`.
> Auto-generated from `_blocks/*.md` via `bash build-index.sh`.
> Each file = one atomic cube (Constructor Pattern).

Example:
```toml
blocks = ["baseline", "rule-pre-dev-gate", "api-anthropic"]
```

## By category

### API

- `api-anthropic` — API — Anthropic (Claude)
- `api-apify` — API — Apify (web scraping platform)
- `api-elevenlabs` — API — ElevenLabs (voice)
- `api-fal-ai` — API — fal.ai (image / video / 3D)
- `api-graphql` — API — GraphQL (schema-first, DataLoader, subscriptions, persisted queries)
- `api-openapi-first` — API — OpenAPI-First (3.1 as single source of truth)
- `api-rest-conventions` — API — REST Conventions (verbs, status codes, resources, idempotency, ETag)
- `api-versioning-pagination-ratelimit` — API — Versioning, Pagination, Rate Limiting

### AUTH

- `auth-authorization` — AUTH — Authorization (RBAC / ABAC / ReBAC)
- `auth-oauth2-oidc` — AUTH — OAuth2 + OIDC (Authorization Code + PKCE)
- `auth-passkeys` — AUTH — Passkeys (WebAuthn / FIDO2)
- `auth-sessions` — AUTH — Sessions & Cookies (+JWT tradeoff)

### CI

- `ci-forgejo-actions` — CI — Forgejo Actions (self-hosted, Tailscale-only admin)
- `ci-github-actions` — CI — GitHub Actions (OIDC, matrix, cache, reusable workflows)
- `ci-release-automation` — CI — Release automation (SemVer, changelog, tagging)
- `ci-security-gate` — CI — Security gate (secrets, SCA, SBOM, semgrep, licenses)

### DB

- `db-drizzle` — DB — Drizzle ORM (TypeScript) patterns
- `db-migration-hygiene` — DB — Migration hygiene (universal)
- `db-postgres` — DB — PostgreSQL (current major — 17 as of 2026-04) patterns
- `db-sqlite` — DB — SQLite (prod-suitable) patterns
- `db-sqlx` — DB — SQLx (Rust) patterns

### DEPLOY

- `deploy-aws-ec2` — DEPLOY — AWS EC2 (Instance Connect + Elastic IP)
- `deploy-cloudflare` — DEPLOY — Cloudflare (Workers / Pages / R2 / KV)
- `deploy-docker` — DEPLOY — Docker
- `deploy-hetzner-cloud` — DEPLOY — Hetzner Cloud (CX22 / CAX11 + TF + Cloud Firewall)
- `deploy-local-only` — DEPLOY — LOCAL ONLY (sensitive / pre-disclosure project)
- `deploy-modal` — DEPLOY — Modal (GPU compute)
- `deploy-vps-generic` — DEPLOY — Generic VPS (provider-agnostic cloud-init + ssh-first-contact)

### DOCS

- `docs-architecture-diagrams` — DOCS — Architecture diagrams (Mermaid)
- `docs-claude-md` — DOCS — `CLAUDE.md` (project bootstrap template)
- `docs-decisions-adr` — DOCS — `DECISIONS.md` / ADR template (MADR 4.0)
- `docs-readme-template` — DOCS — Public `README.md` scaffold
- `docs-runbook` — DOCS — Operational runbook template

### DOMAIN

- `domain-has-secrets` — DOMAIN — Secrets handling
- `domain-ml-training` — DOMAIN — ML Training
- `domain-paid-apis` — DOMAIN — Paid APIs (Anthropic / OpenAI / fal.ai / Apify / Modal / AWS / GCP / ElevenLabs)

### MODE

- `mode-devils-advocate` — MODE — Devil's Advocate
- `mode-first-principles` — MODE — First Principles
- `mode-matrix` — MODE — Agent × Cognitive-Mode Matrix
- `mode-maximalist` — MODE — Maximalist
- `mode-minimalist` — MODE — Minimalist
- `mode-skeptic` — MODE — Skeptic

### OBS

- `obs-metrics` — OBSERVABILITY — Metrics (Prometheus + OTel + RED/USE)
- `obs-structured-logs` — OBSERVABILITY — Structured logs (JSON-lines)
- `obs-traces` — OBSERVABILITY — Distributed traces (OpenTelemetry + W3C traceparent)

### PATH

- `path-user-hooks` — Path atom — user-hooks
- `path-user-memory` — Path atom — user-memory
- `path-user-rules` — Path atom — user-rules

### RULE

- `rule-double-audit` — DOUBLE AUDIT PROTOCOL (mandatory when 3+ files touched)
- `rule-error-budget` — ERROR BUDGET — 3-Level Escalation
- `rule-math-first` — MATH FIRST (mandatory for ML / physics / theory work)
- `rule-pre-dev-gate` — PRE-DEV GATE — three checks before any new code
- `rule-pure-click-contract` — Pure-Click Contract
- `rule-test-first` — TEST-FIRST

### SCRAPER

- `scraper-free-tier` — DOMAIN — Scrapers Tier 1 (free APIs + open-source)
- `scraper-paid-tier` — DOMAIN — Scrapers Tier 3 (Apify / Bright Data paid)
- `scraper-unified-output` — DOMAIN — Scraper unified output invariant

### SECURITY

- `security-audit-logging` — SECURITY — Audit Logging (auditd + journald forwarding)
- `security-firewall-ufw` — SECURITY — Firewall (ufw default-deny + rate limiting + nftables alt)
- `security-patching` — SECURITY — Patching (unattended-upgrades + needrestart + reboot window)
- `security-ssh-hardening` — SECURITY — SSH Hardening (sshd_config.d/99-kei.conf)
- `security-tls-caddy` — SECURITY — TLS via Caddy (automatic ACME, HTTP-01 / DNS-01)

### STACK

- `stack-astro` — STACK — Astro 6 (Content + Marketing + Islands)
- `stack-embedded-stm32` — STACK — Embedded Rust STM32 (embassy / cortex-m)
- `stack-fastapi-postgres` — STACK — FastAPI + async SQLAlchemy 2.0 + PostgreSQL
- `stack-flutter` — STACK — Flutter + Riverpod + Clean Architecture
- `stack-go-server` — STACK — Go server
- `stack-nextjs` — STACK — Next.js 15/16 (App Router + TS + Server Components)
- `stack-python-ml` — STACK — Python ML (PyTorch / JAX)
- `stack-react-vite` — STACK — Vite + React 19 + TypeScript (SPA)
- `stack-rust-axum` — STACK — Rust HTTP server (axum + tokio + sqlx)
- `stack-rust-cli` — STACK — Rust CLI / tooling
- `stack-sveltekit` — STACK — SvelteKit (Svelte 5 Runes + TS)
- `stack-swift-ios` — STACK — Swift iOS (UIKit / SwiftUI hybrid)
- `stack-swift-spm` — STACK — Swift SPM executable (macOS)
- `stack-tailwind` — STACK — Tailwind CSS 4 (compositional add-on)

### TEST

- `test-e2e` — TEST — End-to-end (Playwright browser automation)
- `test-fuzz` — TEST — Fuzzing (input-space exploration)
- `test-load` — TEST — Load / performance testing (baseline → profile → fix)
- `test-property` — TEST — Property-based testing (invariants + shrinking)

### Misc (no category prefix)

- `baseline` — BASELINE — inherit from Main Claude (never violate)
- `evidence-grading` — EVIDENCE GRADING
- `memory-protocol` — MEMORY PROTOCOL
- `pipeline-5phase-template` — Pipeline 5-Phase Wizard Template (shared preamble)

---

Total blocks: 84.
Regenerate via: `bash _blocks/build-index.sh`.
