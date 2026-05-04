---
title: pipe_smoke.rs
path: kei-pipe/tests/pipe_smoke.rs
dna_hash: sha256:2c5218372e94d185
language: rust
size_loc: 315
generated: by-keidocs
---

# kei-pipe/tests/pipe_smoke.rs

Smoke tests for the `kei-pipe` DAG runtime.

Four scenarios:
1. Happy path — 2 steps with `$step.result.id` substitution.
2. Cycle detection — errors clearly.
3. Unknown dependency — errors clearly.
4. Resolver walks `$step.nested.sub.field` into deep paths.

Mock atom: a shell script `mock-atom` that echoes stdin as `result`.
The crate name is `mockcrate`, so atom ids look like `mockcrate::echo`.

## Public API

- Serialize every test that reads/writes `KEI_RUNTIME_BIN_DIR`. Without
- Create a temp dir holding a POSIX shell script `mock-atom` that, for
- Counting mock: the mock script increments a counter file every run so

## Related

- parent: `kei-pipe/tests`
- imports: kei_pipe, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
