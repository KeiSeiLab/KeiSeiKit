# DNA Migration — two formats coexist

> Status lock 2026-05-14. Authoritative on which format to use when.

## Two formats, two granularities

| Format | Layout | Used by | Purpose |
|---|---|---|---|
| **task-class** (4-segment, legacy) | `<role>::<caps>::<scope8>::<body8>-<nonce8>` | `kei-ledger` agent forks (RULE 0.12), internal agent invocations | Internal-agent identity; same prompt re-runs cluster on same task-class |
| **agent-shell** (5-segment, new) | `agent-shell::<provider>:<model>:<caps>::<scope16>::<body16>-<nonce16>` | `keisei-marketplace` user-level invocations | User-shell identity; carries provider+model in the wire format so the marketplace UI / billing pipeline can join on it without parsing JSON |

Hex lengths differ on purpose — 8-char nonce was sufficient for single-machine
ledger; 16-char (64-bit) is required for marketplace where N concurrent
sessions across the public install base push birthday collision into reach.

## Which format does my code emit?

- Writing a substrate-internal agent spawn (sub-agent of an orchestrator,
  background ML run, ledger row for `kei-fork`) → **task-class** via
  `kei-shared::dna::compose(...)` (or equivalent helper in your crate).
- Writing a marketplace user-facing invocation (chat message hitting
  `/v1/chat/completions`, agent the user picked from the public catalog) →
  **agent-shell** via `cryptoid.ts::agentDna(...)` in the marketplace.

When in doubt, ask: "does this row in the ledger correspond to a particular
human user clicking a button?" If yes → agent-shell. If no → task-class.

## Parser table

| You have a string | Parse it with |
|---|---|
| `kei-shared::dna::*` or `kei-model-router::dna_class::*` | `dna_class::role` / `dna_class::caps` / `dna_class::task_class_dna` / `dna_class::agent_class_dna` |
| `agent-shell::*` | `kei-model-router::agent_shell_dna::parse` (Rust) or `cryptoid.ts::parseAgentDna` (TS) |

Both parsers tolerate `None` / `null` on malformed input — never panic.

## Ledger join

When `agent_runs` (marketplace, agent-shell) needs to join `kei-ledger.agents`
(KSK, task-class), use the explicit translation:

```
agent-shell DNA → drop prefix → use (provider, model, caps) as filter,
  use scope_sha+body_sha as join keys
```

There is intentionally **no** lossless round-trip between the two formats —
they carry different information. agent-shell carries provider+model that
task-class does not.

## Cross-language contract

Field names on the parsed struct are aligned per 2026-05-14:

| Rust (`agent_shell_dna::AgentShellDna`) | TypeScript (`ParsedAgentDna`) |
|---|---|
| `provider`  | `provider`  |
| `model`     | `model`     |
| `caps`      | `caps`      |
| `scope_sha` | `scope_sha` |
| `body_sha`  | `body_sha`  |
| `nonce`     | `nonce`     |

snake_case on both sides (TS field names exempted from camelCase convention
for cross-language consistency). JSON round-trip is byte-equal.

## Migration history

- 2026-05-13 — `agent_shell_dna` cube added to `kei-model-router` (issue: marketplace needs provider-aware DNA).
- 2026-05-13 — `cryptoid.ts::agentDna` added in marketplace.
- 2026-05-14 — fields aligned snake_case; legacy 8-hex DNAs explicitly REJECTED by TS `parseAgentDna` (return `null`).

## Not migrated yet

- `kei-shared/src/dna.rs` does not exist as a separate crate in this tree
  yet; the canonical 4-segment implementation lives in
  `_primitives/_rust/kei-model-router/src/dna_class.rs`. When kei-shared
  is extracted, `dna_class` moves there and `agent_shell_dna` follows.
  Update `docs/DNA-FORMAT.md::SSoT` pointer at that time.
