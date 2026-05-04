---
title: usage_INTEGRATION
path: kei-cortex/src/handlers/usage_INTEGRATION.md
dna_hash: sha256:3225d6fb32262bb7
language: markdown
size_loc: 94
generated: by-keidocs
---

# kei-cortex/src/handlers/usage_INTEGRATION.md

## Public API

- ``usage` handler — orchestrator integration` — This file ships ALONGSIDE `usage.rs` + `usage_test.rs`. The agent that
- `1. `handlers/mod.rs` — register the module` — Add ONE line, alphabetical order alongside the existing handler modules:
- `2. `routes.rs` — mount under bearer middleware` — a. Extend the imports:
- `3. `cortex-ui` — render the strip` — a. The new files land at:
- `4. Test commands (orchestrator runs after merge)` — ```sh
- `Rust handler — inline tests are picked up automatically` — cargo test -p kei-cortex usage
- `TS — vitest matches the new tests by filename` — pnpm --filter @keisei/cortex-ui test
- `5. Schema migration follow-up (separate ticket)` — The handler intentionally returns 404 when ANY of the three

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
