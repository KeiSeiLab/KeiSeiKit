---
title: run_atom.rs
path: kei-task/src/run_atom.rs
dna_hash: sha256:6a278a2c8f098d7e
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-task/src/run_atom.rs

Machine-facing `run-atom <verb>` dispatcher.

Reads JSON input (stdin or literal), dispatches to `atoms::<verb>::run`,
serializes the typed Output back to stdout. Exit codes mapped by caller.

## Public API

- `pub fn read_input` — Read JSON input from an optional arg. `None` → read from stdin.
- `pub fn dispatch` — Dispatch a verb to its atom. Returns serialized JSON on success.
- `pub fn exit_for_error` — Map a `DispatchError` to the §Runtime exit-code contract.

## Related

- parent: `kei-task/Cargo.toml`
- imports: crate, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
