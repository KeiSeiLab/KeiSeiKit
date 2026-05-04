---
title: lib.rs
path: kei-pipe/src/lib.rs
dna_hash: sha256:1f704c215dce3e55
language: rust
size_loc: 164
generated: by-keidocs
---

# kei-pipe/src/lib.rs

`kei-pipe` — atom DAG runtime.

Reads a TOML DAG spec, topologically sorts the steps, then runs each
atom sequentially. JSON output of a step can be referenced by a later
step via `$step-id.path.to.field` in its input block.

Atom invocation: spawn `<crate-name> run-atom <verb>` with JSON on
stdin, parse stdout as JSON. Binary resolution honours
`KEI_RUNTIME_BIN_DIR` first, then `PATH` (same contract as
`kei-runtime`).

Public surface:
- [`dag::DagSpec`] / [`dag::Step`] — parsed DAG structures
- [`dag::parse_dag`] / [`dag::topo_sort`] — DAG pipeline
- [`resolve::resolve_input`] — substitute `$step.path` in input values
- [`exec::run_atom`] — invoke one atom via subprocess
- [`report::DagReport`] / [`report::StepReport`] — run outcome
- [`run_dag`] / [`validate_dag`] — top-level entry points

## Public API

- Top-level errors from running a DAG.
- `pub fn validate_dag` — Parse + topo-sort a DAG file without running any atoms. Returns Ok
- `pub fn run_dag` — Parse + execute a DAG file. On success returns a full report; on the
- Open the SQLite cache Connection only if the DAG declares a path AND
- Resolve the effective cache config for a step: per-step wins over

## Related

- parent: `kei-pipe/Cargo.toml`
- imports: crate, std

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
