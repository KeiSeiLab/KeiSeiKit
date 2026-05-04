---
title: cargo_check.rs
path: kei-arch-map/src/evidence/cargo_check.rs
dna_hash: sha256:0af6fb51bbbdd108
language: rust
size_loc: 167
generated: by-keidocs
---

# kei-arch-map/src/evidence/cargo_check.rs

## Public API

- Cached cargo binary path. `which` is invoked once per process; subsequent
- `pub fn resolve_cargo` — Resolve the `cargo` binary to an ABSOLUTE path via the `which` crate.
- Captured cargo output. Mirrors `std::process::Output` but built from
- Spawn cargo check with fixed argv (NOT through sh).
- Spawn a thread that reads a pipe to EOF into a Vec<u8>.
- Wait with timeout while concurrently draining stdout+stderr in
- Count compiler-error diagnostics in cargo JSON stream.

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: serde, std, wait_timeout

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
