# `usage` handler — orchestrator integration

This file ships ALONGSIDE `usage.rs` + `usage_test.rs`. The agent that
wrote those files is sandboxed — it cannot edit `routes.rs`,
`handlers/mod.rs`, or `App.svelte`. The orchestrator wires the handler
in as a follow-up step.

## 1. `handlers/mod.rs` — register the module

Add ONE line, alphabetical order alongside the existing handler modules:

```rust
pub mod usage;
```

Place it between `pub mod tts;` and `pub mod voice_id;`.

## 2. `routes.rs` — mount under bearer middleware

a. Extend the imports:

```rust
use crate::handlers::{chat, health, ledger, memory, pet, portrait, stt, summary, tts, usage};
```

b. Add ONE route line inside `build_router()`, alongside the other
`/api/v1/cortex/*` routes (just before `.route_layer(...)`):

```rust
.route("/api/v1/cortex/usage", get(usage::usage))
```

The `tts` import already pulls `axum::routing::get`; no new import needed.
The bearer middleware (`require_bearer`) covers this path automatically
because it is mounted before `.route_layer(...)`. CORS is inherited from
the outer router.

## 3. `cortex-ui` — render the strip

a. The new files land at:

- `_ts_packages/packages/cortex-ui/src/components/BudgetStrip.svelte`
- `_ts_packages/packages/cortex-ui/src/lib/usage/usage-client.ts`
- `_ts_packages/packages/cortex-ui/tests/budget-strip.test.ts`
- `_ts_packages/packages/cortex-ui/tests/usage-client.test.ts`

b. Mount the strip in `App.svelte` header (or as a toggle-able panel).
Recommended placement — top of the main column, above the chat panel:

```svelte
<script lang="ts">
  import BudgetStrip from './components/BudgetStrip.svelte';
  // …existing imports…
</script>

{#if config}
  <header class="cortex-header">
    <BudgetStrip {config} />
  </header>
  <!-- existing panels -->
{/if}
```

c. The strip degrades gracefully — when `/api/v1/cortex/usage` returns
404 (ledger has no `cost_cents` column yet) the strip renders a single
muted line `ledger unavailable`. No alert, no console error spam.

## 4. Test commands (orchestrator runs after merge)

```sh
# Rust handler — inline tests are picked up automatically
cargo test -p kei-cortex usage

# TS — vitest matches the new tests by filename
pnpm --filter @keisei/cortex-ui test
```

## 5. Schema migration follow-up (separate ticket)

The handler intentionally returns 404 when ANY of the three
cost-tracking columns is absent: `cost_cents`, `provider`, `model`.
Adding them is the `kei-ledger` crate's responsibility — a v6
migration appending all three together:

```sql
ALTER TABLE agents ADD COLUMN provider TEXT;
ALTER TABLE agents ADD COLUMN model TEXT;
ALTER TABLE agents ADD COLUMN cost_cents INTEGER;
```

Partial migration (e.g. only `cost_cents` exists) ALSO routes to 404 —
the handler refuses to half-render the strip from incomplete data.
Until the v6 migration ships, the strip displays the graceful
"ledger unavailable" fallback. No production breakage, no UX regression.
