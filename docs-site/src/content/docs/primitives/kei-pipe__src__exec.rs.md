---
title: exec.rs
path: kei-pipe/src/exec.rs
dna_hash: sha256:d59b6c239ee11e96
language: rust
size_loc: 179
generated: by-keidocs
---

# kei-pipe/src/exec.rs

Spawn an atom subprocess and return its JSON output.

Atom IDs are `<crate-name>::<verb>` — e.g. `kei-task::create`. The
crate name resolves to an executable using the same contract as
`kei-runtime`: first `$KEI_RUNTIME_BIN_DIR/<crate>`, then walk `PATH`.

The atom is invoked as `<crate> run-atom <verb>`, JSON on stdin, JSON
on stdout. Exit code 0 = ok; anything else = `AtomFailed`. Tests can
substitute a mock binary by pointing `KEI_RUNTIME_BIN_DIR` at a temp
dir whose `<crate>` is a shell script that echoes its stdin (see
`tests/pipe_smoke.rs`).

## Public API

- `pub fn split_atom` — Parse an atom id into `(crate, verb)`. Rejects empty halves.
- `pub fn run_atom` — Invoke an atom, returning the parsed JSON result (the atom's own
- Outcome label accompanying a cache-aware invocation.
- Returned from the cache; atom was NOT invoked.
- Cache miss; atom was invoked and the result stored.
- `pub fn run_atom_cached` — Cache-aware atom invocation. On hit returns cached JSON; on miss calls
- Resolve `<crate>` as an executable. Mirrors `kei-runtime::invoke`.

## Related

- parent: `kei-pipe/Cargo.toml`
- imports: serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
