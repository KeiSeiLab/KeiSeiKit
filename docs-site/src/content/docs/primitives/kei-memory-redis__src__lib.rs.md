---
title: lib.rs
path: kei-memory-redis/src/lib.rs
dna_hash: sha256:34020bbfd45efe1d
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-memory-redis/src/lib.rs

kei-memory-redis — MemoryBackend trait-impl backed by Redis 7+.

Hosted Sleep Wave 6 atomar. Async via the `redis` crate (`aio` +
`tokio-comp`). Single-class-per-file Constructor Pattern:

- [`error`] : crate-local error type, mappable into
`kei_runtime_core::Error`.
- [`store`] : low-level Redis client + key-schema helpers (no trait).
- [`backend`] : [`backend::RedisBackend`] glues `RedisStore` to the
`MemoryBackend` trait + carries a DNA.

Live integration tests live in `tests/redis_smoke.rs` and are gated
behind the `live` cargo feature so a default `cargo test` run does
not require a running Redis.

## Related

- parent: `kei-memory-redis/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
