---
title: drive_smoke.rs
path: kei-spawn/tests/drive_smoke.rs
dna_hash: sha256:8677fe7aa3940be8
language: rust
size_loc: 130
generated: by-keidocs
---

# kei-spawn/tests/drive_smoke.rs

drive_smoke — integration tests for `kei-spawn drive` subcommand.

The drive subcommand shells the full pipeline:
1. spawn_from_task (prepare + ledger fork)
2. SpawnOutput JSON → stdout
3. ManualDriver::invoke → NotImplemented → stderr + exit 64

Because exit-code assertions require invoking the real binary, these
tests use `CARGO_BIN_EXE_kei-spawn` (populated by cargo for integration
tests) with `KEI_SPAWN_LEDGER_NOOP=1` set so the ledger subprocess is
a no-op.

## Related

- parent: `kei-spawn/tests`
- imports: std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
