# MODE — Agent × Cognitive-Mode Matrix

Composable cognitive-mode blocks live in `_blocks/mode-*.md`. Any agent manifest can append them to its `blocks = [...]` list to stack the behavioural skew; modes compose (e.g. `mode-skeptic` + `mode-minimalist` = adversarial pruner).

This table is the suggested starting set per agent role. It is a **guide, not a rule** — pick what fits the agent's actual job.

| Agent role | Recommended modes | Reason |
|---|---|---|
| critic | `skeptic` · `devils-advocate` | Doubt-first review; name the strongest objection before agreeing |
| validator | `skeptic` | Every claim needs an E1/E2 grade — no plausibility shortcuts |
| security-auditor | `devils-advocate` · `skeptic` | Steel-man the attacker; threat-model the worst case |
| researcher | `skeptic` | Cross-check every source; honest gaps over confident guesses |
| ml-researcher | `skeptic` · `first-principles` | Observable classification + invariant-derived priors |
| architect | `first-principles` · `minimalist` | Derive from constraints, prefer subtraction over addition |
| code-implementer | `minimalist` | Surgical edits; remove before adding |
| refactor specialist | `minimalist` | Delete dead code; prove every kept line |
| ml-implementer | `minimalist` · `first-principles` | Math-First — count params before code, derive over tune |
| brainstorm / concept-explorer | `maximalist` | Return 10× version + minimum bounds; user invokes exploration |
| physics-deriver | `first-principles` | Cite the invariant; no arguments from "best practice" |

Intentionally **unbiased** roles (pick 0 modes by default):
- `infra-implementer`, `modal-runner`, `fal-ai-runner`, `cost-guardian`, most `kei-<project>-specialist` agents.

Modes are not free — each one lands verbatim in the prompt and consumes context. Stack only what you need.
