---
title: redis_smoke.rs
path: kei-memory-redis/tests/redis_smoke.rs
dna_hash: sha256:91aa710100e63fd6
language: rust
size_loc: 101
generated: by-keidocs
---

# kei-memory-redis/tests/redis_smoke.rs

Live smoke tests. Compile-only by default. Enable with:

```text
cargo test -p kei-memory-redis --features live
```

The tests assume a reachable Redis 7+ on `REDIS_URL`
(default `redis://127.0.0.1:6379`). They isolate themselves under a
per-test prefix and clean up at the end.

## Related

- parent: `kei-memory-redis/tests`
- imports: kei_memory_redis, kei_runtime_core

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
