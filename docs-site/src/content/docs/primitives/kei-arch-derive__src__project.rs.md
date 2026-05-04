---
title: project.rs
path: kei-arch-derive/src/project.rs
dna_hash: sha256:bfc36925010a9e38
language: rust
size_loc: 164
generated: by-keidocs
---

# kei-arch-derive/src/project.rs

Predicate → Evidence projection (the §2 mapping table of MATH-DNA-DESIGN).

Each kei-registry `Predicate` variant projects DOWN to one of the seven
evidence kinds already shipped in `kei_arch_map::schema::Evidence`:
`FileExists`, `RegexMatch`, `GrepCount`, `FileSize`, `JsonField`,
`CargoCheckClean`, `HttpStatus`. The 6 "new derivable kinds" (per
Wave 5 Option B verdict) project onto existing ones via synthesized
patterns — no schema bump required.

Constructor Pattern: this cube ONLY converts. It owns no I/O and no
ordering. Output is deterministic for a given predicate (pure fn).

## Public API

- Mirror of `kei_arch_map::schema::Evidence` — the seven evidence kinds
- `pub fn predicate_to_evidence` — Project a single registry predicate onto an `EvidenceClaim`.
- Minimal regex-meta escape. Symbol names rarely contain regex meta but

## Related

- parent: `kei-arch-derive/Cargo.toml`
- imports: kei_registry, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
