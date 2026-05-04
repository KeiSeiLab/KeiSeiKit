---
title: error.rs
path: kei-llm-mlx/src/error.rs
dna_hash: sha256:7be953a5f2824710
language: rust
size_loc: 71
generated: by-keidocs
---

# kei-llm-mlx/src/error.rs

Error enum — exit-code SSoT.

Each variant maps to a stable exit code in `cli.rs`:
1 → IO / parse / serialize error
2 → NotSupported (platform gate)
3 → BinaryNotFound / ModelNotFound
4 → NonZeroExit (subprocess returned non-zero)
5 → Timeout

Constructor Pattern: this cube holds the enum + impls only. Conversion
to `ExitCode` lives in `cli.rs` so the error surface stays
interpretation-agnostic (a library consumer can map differently).

## Public API

- Platform gate refused. Reason is the stable `is_supported()` string.
- `which mlx_lm.generate` / `mlx_lm.server` returned nothing.
- Model id not present in the cache (or cache directory missing).
- `Command::output` failed (binary not executable, permission, etc.).
- Child exited with non-zero. `code = None` only on signal kill.
- stdout / footer / NDJSON could not be parsed.
- Reserved for future timeout-bounded calls. Currently unused; kept
- Security gate refused (e.g. `--host` not localhost).
- IO / serialize error not covered by the above.
- `pub fn exit_code_for` — Exit-code table. SSoT for `cli.rs`.

## Related

- parent: `kei-llm-mlx/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
