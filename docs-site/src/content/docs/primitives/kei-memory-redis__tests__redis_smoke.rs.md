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
