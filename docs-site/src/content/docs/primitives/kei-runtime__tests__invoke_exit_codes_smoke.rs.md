---
title: invoke_exit_codes_smoke.rs
path: kei-runtime/tests/invoke_exit_codes_smoke.rs
dna_hash: sha256:7ced6f7136caf147
language: rust
size_loc: 136
generated: by-keidocs
---

# kei-runtime/tests/invoke_exit_codes_smoke.rs

Integration test — `kei-runtime invoke` exit codes per §Runtime contract.

- Unknown atom id → exit 2 (atom rejected)
- Known atom whose crate binary is not on PATH → exit 127 (BinaryNotFound)

Real-atom execution (happy path) lives in `invoke_real_atom.rs`, which
points `KEI_RUNTIME_BIN_DIR` at the workspace `target/` to pick up
`kei-task` without polluting the user's PATH.

## Public API

- An atom whose `crate_name` is not in the `kei-*` allowlist should exit 2

## Related

- parent: `kei-runtime/tests`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
