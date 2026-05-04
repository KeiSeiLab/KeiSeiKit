---
title: emitter.rs
path: kei-decision/src/emitter.rs
dna_hash: sha256:ecb9e56d5cea6b73
language: rust
size_loc: 143
generated: by-keidocs
---

# kei-decision/src/emitter.rs

`RankedAction → task.toml` emitter (kei-spawn-compatible).

Each emitted file is a minimal kei-spawn task with three sections:

[task]     role + description + body-from-master-line
[scope]    files-whitelist guessed from kind
[body]     long-form text (the action title + source-line ref)

The orchestrator can edit the file before passing to `kei-spawn spawn`.

## Public API

- `pub fn emit_task_toml` — Emit one task.toml under `out_dir`. Returns its path + size.
- Best-effort whitelist guess. Conservative — orchestrator should verify.
- Heuristic: pick the longest run of letters after "kei-" or after "primitive ".
- `make_slug("Refactor 4 hooks to call kei-leak-matrix") → "refactor-4-hooks-to-call-kei-leak"`.
- Use serde_json string escape for safety, then wrap in TOML basic-string quotes.

## Related

- parent: `kei-decision/Cargo.toml`
- imports: anyhow, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
