# kei-model-router

Model selection (Haiku 4.5 / Sonnet 4.6 / Opus 4.7) for Claude Code Agent
spawns. Empirical-posterior decision rule keyed on task-class DNA + Beta
posterior + cost minimization, with kernel-smoothing for unseen task
classes.

## Math

Decision rule:

    m*(d̂) = argmin_{m ∈ M} { c(d̂, m) | P[q(d̂, m) ≥ q*] ≥ 1 − δ }

Empty feasible set → fallback (top tier) per RULE -1 NO DOWNGRADE.

Posterior: q(d, m) ~ Beta(α₀ + n⁺, β₀ + n⁻) with uniform prior. n⁺ counts
rows where outcome='functional' AND escalation_depth=0; n⁻ everything else.

Kernel-smoothed transfer for unseen task classes:

    K(d, d') = α_role · 1[role=role'] +
               α_caps · |caps ∩ caps'| / |caps ∪ caps'| +
               α_scope · 1[scope=scope'] +
               α_body · jaccard_bigram(body, body')

## Verified pricing

[VERIFIED: https://platform.claude.com/docs/en/docs/about-claude/pricing 2026-04-30]

| Model     | Input/MTok | Output/MTok |
| --------- | ---------- | ----------- |
| Haiku 4.5 | $1.00      | $5.00       |
| Sonnet 4.6| $3.00      | $15.00      |
| Opus 4.7  | $5.00      | $25.00      |

Opus 4.7 uses a new tokenizer that may produce up to 35% more tokens on
identical text — multiply quote accordingly when comparing against
Haiku/Sonnet on the same input.

## CLI

    kei-model-router pricing                  # print pricing table
    kei-model-router select <agent> [--prompt P]
    kei-model-router calibrate                # re-fit kernel weights
    kei-model-router --help

## Orchestrator integration (Path B — runtime)

Per RULE 0.13, the orchestrator owns Agent spawning. Before spawning a
non-trivial agent the orchestrator can consult the router and pass an
explicit `model` parameter:

    kei-model-router select code-implementer-rust \
        --prompt "Add multi-tool integration test for parser"
    # → model: sonnet (if posterior built up); model: opus (initial fallback)

Then in the orchestrator's Agent invocation:

    Agent({ subagent_type: "code-implementer-rust", model: "sonnet", ... })

Until posterior data accumulates the router conservatively returns
top-tier (Opus). As `outcome` column fills via `agent-fork-done.sh` STATUS-TRUTH
parsing, posterior diversifies and cheaper tiers begin to qualify.

## Assembler integration (Path A — compose-time, deferred)

Rebaking the model into generated `.md` files at assemble time is
deferred. Current default `model: opus` in 55/55 manifests is safe;
adopt Path B (orchestrator discipline) until ledger has ≥100
outcome-tagged rows per common task class.

## Cubes

| File          | LOC | Concern                                      |
| ------------- | --- | -------------------------------------------- |
| pricing.rs    | 167 | Verified per-MTok constants (microcents)     |
| dna_class.rs  | 113 | DNA component extraction (role/caps/scope)   |
| complexity.rs | 178 | τ-estimator (heuristic regex+role+length)    |
| posterior.rs  | 197 | Beta posterior from ledger + Wilson lower b. |
| kernel.rs     | 134 | Substrate similarity kernel for unseen DNAs  |
| escalate.rs   |  73 | Retry-ladder bookkeeping                     |
| select.rs     | 197 | Decision rule (argmin cost s.t. q_lb ≥ q*)   |
| calibrate.rs  | 193 | Offline LOO weight refit (grid search)       |
| main.rs       | 142 | CLI dispatch                                 |
| lib.rs        |  35 | Module barrel + re-exports                   |

All cubes within Constructor Pattern budgets (≤200 LOC, ≤30 LOC/fn).

## Schema dependency

Requires `kei-ledger` schema v9+ which adds:

    tokens_in INTEGER
    tokens_out INTEGER
    stubs_count INTEGER DEFAULT 0
    outcome TEXT CHECK (outcome IN ('functional','partial','scaffolding','fail',NULL))
    escalation_depth INTEGER DEFAULT 0
    task_class_dna TEXT GENERATED ALWAYS AS (...) VIRTUAL
    INDEX idx_agents_task_class ON agents(task_class_dna)

## Hooks

- `agent-fork-logger.sh` (PreToolUse:Agent, advisory) — writes 'running'
  row with DNA at spawn.
- `agent-fork-done.sh` (PostToolUse:Agent) — closes row + parses
  STATUS-TRUTH MARKER from agent's tool_response → updates outcome,
  stubs_count, tokens_in, tokens_out.

## Lock

2026-04-30. Phases 1-9 of kei-model-router rollout complete (Phase 9
orchestrator-discipline; assembler refactor deferred until ≥100
outcome-tagged rows accumulate).
