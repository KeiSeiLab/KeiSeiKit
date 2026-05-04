---
title: exec.rs
path: kei-cache/src/exec.rs
dna_hash: sha256:de4ee82ff094c407
language: rust
size_loc: 139
generated: by-keidocs
---

# kei-cache/src/exec.rs

Atom invocation on cache miss.

Constructor Pattern: `AtomExecutor` trait = one-method contract
(atom_id + canonical input → JSON payload string). `SubprocessExecutor`
is the production impl — mirrors the kei-runtime binary-resolution
rules (`KEI_RUNTIME_BIN_DIR` → `$PATH`) and spawns
`<crate> run-atom <verb>` with the input on stdin.

Kind-safety: before invoking we consult `kei-atom-discovery` to obtain
`AtomKind`. `command` and `stream` are refused ("unsafe to cache");
`query` and `transform` pass through.

## Public API

- `pub trait AtomExecutor` — Strategy for invoking an atom after a cache miss.
- `pub struct SubprocessExecutor` — Production executor: resolves atom metadata via kei-atom-discovery,
- `pub fn ensure_cacheable` — Gate: only pure kinds may be cached. Command has side effects; stream is
- Spawn `<crate> run-atom <verb>` with `input_json` on stdin; return stdout.
- `pub fn resolve_binary` — Resolve binary by name:

## Related

- parent: `kei-cache/Cargo.toml`
- imports: anyhow, kei_atom_discovery, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
