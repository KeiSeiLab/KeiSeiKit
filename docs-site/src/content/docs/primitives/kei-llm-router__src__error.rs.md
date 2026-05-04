---
title: error.rs
path: kei-llm-router/src/error.rs
dna_hash: sha256:5dd7e7c976883b97
language: rust
size_loc: 89
generated: by-keidocs
---

# kei-llm-router/src/error.rs

Error enum for kei-llm-router.

Constructor Pattern: ONE responsibility — name failure modes and map
each to a stable exit code. The CLI surfaces the code; the lib surface
returns `Error` so callers can handle programmatically.

Exit code map (per task spec):
0 success
1 IO / probe error           → ProbeFailed | IoError
2 no backend available       → NoBackendAvailable | NoCompatibleBackend
3 model not in registry      → ModelNotInRegistry

## Public API

- All failure modes the router surfaces.
- `kei_machine_probe::probe()` failed before we could decide.
- No viable backend reached an "available" state for `model_id`.
- Machine reports `Capability::NoLocalInferenceViable` — no point
- `kei_model::Registry::get(model_id)` returned None and the
- Generic IO / file / serde failure.
- `pub fn exit_code` — Stable exit code for the CLI to surface.
- `pub fn kind` — Human-readable kind tag for JSON serialisation.
- `pub type Result` — Convenient `Result` alias used throughout the crate.

## Related

- parent: `kei-llm-router/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
