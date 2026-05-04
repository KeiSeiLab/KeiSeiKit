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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
