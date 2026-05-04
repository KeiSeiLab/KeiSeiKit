---
title: facet_query.rs
path: kei-sage/src/facet_query.rs
dna_hash: sha256:ce244cf026d79c28
language: rust
size_loc: 188
generated: by-keidocs
---

# kei-sage/src/facet_query.rs

Facet-query over capability.toml + manifest .toml primitives.

TX1 adds `[taxonomy]` + `[lineage]` sections to primitive TOMLs.
This module walks a capabilities root (`<root>/*/*/capability.toml`)
and a manifests root (`<root>/*.toml`), parses the taxonomy section,
and filters by `key=value` AND predicates.

## Public API

- A primitive's identity + its taxonomy facets.
- `pub fn parse_primitive` — Parse a single TOML file into a `PrimitiveFacets`, or `None` if it's
- `pub fn discover_primitives` — Walk capabilities + manifests roots and return all parseable primitives.
- `pub fn discover_primitives_with_roles` — Same as `discover_primitives`, but also walks an optional roles root
- `pub fn parse_filters` — Parse `k=v` filter strings into pairs. Bad entries (no `=`) are dropped.
- `pub fn matches_all` — AND-filter: a primitive matches iff ALL `(k, v)` pairs are present and equal.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, serde, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
