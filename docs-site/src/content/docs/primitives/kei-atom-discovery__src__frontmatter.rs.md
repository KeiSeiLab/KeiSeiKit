---
title: frontmatter.rs
path: kei-atom-discovery/src/frontmatter.rs
dna_hash: sha256:6c7fc9eab3752d61
language: rust
size_loc: 191
generated: by-keidocs
---

# kei-atom-discovery/src/frontmatter.rs

Frontmatter schema + YAML parsing.

Locked schema per `docs/SUBSTRATE-SCHEMA.md`. `input`/`output` are
REQUIRED for command/query/stream, OPTIONAL for transform.

YAML parser is `serde_yaml_ng` (maintained fork of the archived
`serde_yaml` crate). A 64 KiB size cap is enforced pre-parse as a
billion-laughs mitigation.

## Public API

- `pub const MAX_FRONTMATTER_BYTES` — Hard cap on frontmatter size. 64 KiB is 100× any realistic atom spec.
- Optional taxonomy facets per `docs/TAXONOMY.md`. All fields optional.
- Optional lineage metadata — wikilink parents + creator DNA + created date.
- Fully-parsed atom metadata — one canonical struct shared across crates.
- Raw deserialisation target — kept private, `AtomMeta` is the public shape.
- `pub fn parse_frontmatter` — Split a markdown file into (frontmatter_yaml, body). Enforces a 64 KiB
- `pub fn parse_side_effects` — Parse the `side_effects:` YAML sequence into typed `{op, domain}` pairs.

## Related

- parent: `kei-atom-discovery/Cargo.toml`
- imports: crate, serde, serde_yaml_ng, std

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
