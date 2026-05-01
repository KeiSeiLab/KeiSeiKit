# API — Anthropic (Claude)

Full text: Anthropic docs (WebFetch https://docs.anthropic.com/en/api before any new feature). Claude API skill trigger: code imports `anthropic` / `@anthropic-ai/sdk`.

**Model IDs (from env, never hard-code):**
- Opus tier — max effort, 1M input tokens on the `[1m]` variant
- Sonnet tier — balanced cost / capability
- Haiku tier — cheapest, latency-critical
- Keep ID in env var (`ANTHROPIC_MODEL`) — swapping Opus→Sonnet should be 0 code changes.

**Prompt caching (up to ~90% cost reduction + latency drop on cache hit):**
- 4 cache breakpoints per request (`cache_control: {type: "ephemeral"}`)
- Two TTLs: default 5-min (cheap writes) and 1-hour (premium writes, higher $/token)
- Same prefix sent >N times → MUST `cache_control` — missing caching on a long system prompt is free money left on the table
- Log cache_read_input_tokens vs cache_creation_input_tokens every call — if read is zero across N calls, cache is mis-wired

**Tool use:**
- Fine-grained tool streaming supported (parse tool_use deltas, don't wait for full turn)
- `tool_choice: "auto" | "any" | {type: "tool", name}` — pick `any` when you need *some* tool but don't care which
- Cap turn loop with `max_iterations` (default 10) — infinite loop on broken tool = infinite cost
- Every tool_use MUST have matching tool_result — orphan tool_use errors mid-turn

**Batch API:** 50% discount, 24h window. Use for offline eval / bulk-ingest / non-interactive tasks. Polling via batch ID.

**Extended thinking:** `thinking: {type: "enabled", budget_tokens: N}`. Higher budget → deeper reasoning. Visible thinking is billed; hidden is not streamed but still billed.

**Cost tracking (mandatory per-call log):** `input_tokens`, `output_tokens`, `cache_read_input_tokens`, `cache_creation_input_tokens` → `memory/{project}.md`. Rates change — WebFetch https://www.anthropic.com/pricing before any budgeted run [VERIFY: live pricing page].

**Forbidden:** hard-coding model strings in source (use env var); using deprecated IDs without a migration note citing the replacement; sending the same >2K-token prefix >3 times without `cache_control`; skipping per-call cost log (no data → no decisions).
