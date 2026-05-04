---
title: ledger_sh.rs
path: kei-spawn/src/ledger_sh.rs
dna_hash: sha256:e8c1589ddd44ea1b
language: rust
size_loc: 199
generated: by-keidocs
---

# kei-spawn/src/ledger_sh.rs

Thin subprocess wrapper around the `kei-ledger` binary.

kei-ledger is a bin-only crate (no lib.rs at the time kei-spawn was
introduced). We shell to it rather than replicate SQL — same process
model users expect, same DB file, same env contract (`KEI_LEDGER_DB`).

Every call surfaces stderr on failure so orchestrator sees the real
ledger error (branch too long, duplicate id, etc.), not a wrapped one.

# Security — `$PATH` hijack (CWE-427)

The final fallback in [`ledger_bin`] is the bare name `"kei-ledger"`,
which `std::process::Command` resolves by walking `$PATH`. On a shared
or compromised machine an attacker-controlled directory earlier on
`$PATH` can provide a rogue `kei-ledger` that silently captures ledger
writes. To mitigate:

* Set `KEI_LEDGER_BIN` to an **absolute path** in production / CI
(e.g. `/usr/local/bin/kei-ledger` or the cargo-install path), or
* Run integration tests via `cargo test` which populates the
`CARGO_BIN_EXE_kei-ledger` env var with the workspace-built binary.

The env-override path is checked first in [`ledger_bin`] precisely so
a trusted operator can pin the binary and sidestep `$PATH` resolution.
Regression coverage for [`ledger_bin`] lookup precedence.

Env vars are process-global; serialize with a local mutex so
parallel cargo-test workers don't trample each other.

## Public API

- `pub fn ledger_bin` — Resolve `kei-ledger` executable. Env override → CARGO env (tests) → PATH.
- Test / sandbox escape hatch: when set, every ledger call is a no-op.
- `pub fn fork` — Run `kei-ledger fork` with DNA + worktree metadata.
- `pub fn done` — Run `kei-ledger done <id> --summary <s>`.
- `pub fn fail` — Run `kei-ledger fail <id> --reason <r>`.
- `pub fn list_running` — Run `kei-ledger list --status running`. Returns raw stdout lines.

## Related

- parent: `kei-spawn/Cargo.toml`
- imports: anyhow, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
