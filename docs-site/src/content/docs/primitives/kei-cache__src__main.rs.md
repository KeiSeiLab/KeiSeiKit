---
title: main.rs
path: kei-cache/src/main.rs
dna_hash: sha256:ea0a71c7fa041e8e
language: rust
size_loc: 137
generated: by-keidocs
---

# kei-cache/src/main.rs

kei-cache CLI dispatcher.

Constructor Pattern: single cube = arg parsing + dispatch + formatting.
Storage: `~/.claude/cache/cache.sqlite` (or `$KEI_CACHE_DB` override).

## Public API

- Override cache DB path (default: $KEI_CACHE_DB or ~/.claude/cache/cache.sqlite)
- Wrap an atom invocation with deterministic caching.
- Atom id (e.g. `kei-router:route`).
- JSON-string input to hash + forward on miss.
- TTL in seconds (default: 3600).
- Atoms-root for discovery (default: $KEI_ATOMS_ROOT or cwd).
- Print hit/miss + live entry counts.
- Evict all expired entries.
- Wipe cache + counters.

## Related

- parent: `kei-cache/Cargo.toml`
- imports: clap, kei_cache, std

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
