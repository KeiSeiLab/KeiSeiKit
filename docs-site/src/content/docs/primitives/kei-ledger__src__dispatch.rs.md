---
title: dispatch.rs
path: kei-ledger/src/dispatch.rs
dna_hash: sha256:47ec532a1c0aad47
language: rust
size_loc: 186
generated: by-keidocs
---

# kei-ledger/src/dispatch.rs

CLI dispatch helpers — one function per subcommand.

Constructor Pattern: extracted from `main.rs` to keep the entry point
under the 200-LOC cap. Each fn returns `ExitCode` directly so `main`
stays a flat match.

Module owner: the binary crate. Pulls library functions from the
`kei_ledger` crate (defined in `src/lib.rs`).

## Public API

- `pub fn cmd_record_cost` — Record cost metadata for an existing agent. Emits JSON to stdout so
- `pub fn cmd_record_skill` — Record a skill invocation row in `skill_invocations` (schema v8+).
- `pub fn cmd_aggregate_skills` — Thin pass-through so `main.rs` keeps all cmd_* in one import namespace.
- Emit the post-write JSON line. Split out to keep `cmd_record_cost`

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: kei_ledger, rusqlite, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
