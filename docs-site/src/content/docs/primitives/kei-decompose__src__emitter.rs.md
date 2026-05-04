---
title: emitter.rs
path: kei-decompose/src/emitter.rs
dna_hash: sha256:79c1760eff56d138
language: rust
size_loc: 149
generated: by-keidocs
---

# kei-decompose/src/emitter.rs

Action[] → kei-spawn task.toml[] emitter.

One file per Action. Filename pattern:
`<source-stem>-<action-id>-<slug>.toml`

Schema mirrors what kei-spawn consumes: `[task]`, `[scope]`, `[body]`.
Native emitter only — research adapter could shell out to kei-decision
if available, but the unified shape removes that need.

## Public API

- `pub fn emit_all` — Emit a list of Actions, one task.toml per Action, under `out_dir`.
- Heuristic whitelist for research actions: pick the longest run of letters
- Build a 1-8-token kebab slug from the title.
- Use serde_json string escape for safety, then return TOML basic-string.

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
