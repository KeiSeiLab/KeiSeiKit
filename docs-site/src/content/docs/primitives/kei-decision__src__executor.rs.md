---
title: executor.rs
path: kei-decision/src/executor.rs
dna_hash: sha256:0a0dc081d145d6cd
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-decision/src/executor.rs

Action executor — emit task.toml + shell out to `kei-spawn spawn`.

Captures stdout (kei-spawn emits a JSON `SpawnOutput`), parses the fields
we surface in [`ExecuteOutput`]. If the binary cannot be found we look
at a hard-coded fallback path under `~/Projects/KeiSeiKit/_primitives/
_rust/target/release/kei-spawn` before giving up loud.

## Public API

- `pub fn execute_action` — Invoke `kei-spawn spawn <task.toml>` and parse its JSON stdout.
- Parse the JSON `kei-spawn spawn` produces into our [`ExecuteOutput`].
- Search PATH first, fall back to a known release-build location.
- Tiny `which` clone — checks each PATH entry for an executable file.

## Related

- parent: `kei-decision/Cargo.toml`
- imports: anyhow, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
