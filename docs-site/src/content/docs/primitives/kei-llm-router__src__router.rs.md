---
title: router.rs
path: kei-llm-router/src/router.rs
dna_hash: sha256:dc04ba823eab4ff0
language: rust
size_loc: 155
generated: by-keidocs
---

# kei-llm-router/src/router.rs

Core decision logic for kei-llm-router.

Constructor Pattern: ONE responsibility — turn `(Machine, model_id, opts)`
into a `RouteDecision`. Two layers:

1. **Pure** — `decide()` accepts pre-probed candidates and the machine
snapshot, returns a decision. Deterministic; unit-testable without
network.
2. **Live** — `route()` calls `discovery::discover_models` then `decide()`.
The async surface — used by the CLI binary.

The router does NOT spawn subprocesses; every backend interaction goes
through `health` / `discovery` which delegate to W57/W58/W59 crates.

## Public API

- Caller-supplied options for one route decision.
- Outcome of `route` / `decide`.
- Live route — probe + decide. Used by the CLI binary.
- `pub fn decide` — Pure decision — choose a backend given pre-probed candidates.

## Related

- parent: `kei-llm-router/Cargo.toml`
- imports: crate, kei_machine_probe, kei_model, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
