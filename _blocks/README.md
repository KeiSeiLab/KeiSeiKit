# `_blocks/` — Composable Agent Content

Each `.md` file in this directory is a **block**: a single-concern, standalone-readable snippet that any agent manifest can include via its `blocks = [...]` list. The `_assembler` concatenates selected blocks + manifest metadata into the final agent `.md` that Claude Code loads.

Blocks are grouped by prefix:

| Prefix | Purpose |
|---|---|
| `baseline`, `evidence-grading`, `memory-protocol` | Obligatory base — every manifest must include these |
| `rule-*` | Discipline rules (`pre-dev-gate`, `test-first`, `error-budget`, `double-audit`, `math-first`) |
| `mode-*` | Cognitive mode blocks (see below) |
| `stack-*` | Language / framework constraints (Rust Axum, React Vite, Swift SPM, …) |
| `deploy-*` | Deployment target rules (Modal, AWS EC2, Cloudflare, Hetzner, …) |
| `api-*` | External API conventions (Apify, fal.ai, ElevenLabs, Anthropic, …) |
| `db-*` | Database rules (Postgres, SQLite, Drizzle, sqlx, migrations) |
| `auth-*`, `security-*`, `obs-*`, `ci-*`, `test-*`, `scraper-*`, `domain-*`, `docs-*` | Domain-specific rules |

## Cognitive mode blocks

Composable behavioural skews. Add any combination to a manifest's `blocks` list to stack the mode. Modes compose — e.g. `mode-skeptic` + `mode-minimalist` yields an adversarial pruner.

| Block | Purpose |
|---|---|
| `mode-skeptic.md` | Doubt the conclusion until proved; flag claims without E1/E2 grade |
| `mode-devils-advocate.md` | Steel-man the opposite; name the strongest objection before agreeing |
| `mode-minimalist.md` | Prefer deleting over adding; justify every addition against existing code |
| `mode-maximalist.md` | Explore 10× scope; return both maximum and minimum bounds; only when user invokes exploration |
| `mode-first-principles.md` | Derive from invariants; cite the physical / mathematical constraint, not "best practice" |

See `mode-matrix.md` for the **agent-role × recommended-modes** table used by the `skills/new-agent` wizard (Phase 3.6). It is the suggested starting set per role — modes remain a free pick per manifest.

## Adding a new block

1. Pick a stable prefix (existing category or a new one documented here).
2. One concern per file. 20–50 LOC target, `<200 LOC` hard cap (Constructor Pattern).
3. Imperative voice (`"Do X"` not `"the agent should do X"`) — these land verbatim in agent prompts.
4. Standalone-readable — do not assume sibling blocks are present. Cross-references OK, hard dependencies not.
5. Reference from a manifest's `blocks = [...]` list; the assembler validates existence.

## Ownership

Blocks are **kit-owned** — `install.sh` overwrites `_blocks/` on re-run, backing up local edits to `_blocks.bak-TIMESTAMP/`. User-owned content belongs in `_manifests/*.toml` (which are never overwritten).
