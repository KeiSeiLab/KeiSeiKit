---
title: coverage_map.rs
path: kei-cleanup/src/scanners/coverage_map.rs
dna_hash: sha256:45a65c28f3e76daa
language: rust
size_loc: 150
generated: by-keidocs
---

# kei-cleanup/src/scanners/coverage_map.rs

Coverage-map scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 + Appendix A.

Walks workspace; collects two sets of marker IDs:
* Derivations:  lines matching cfg.coverage_map.derivation_marker
(default `// derive:`) in src/*.rs and theory/*.md.
* Tests:        lines matching cfg.coverage_map.test_id_marker
(default `// covers:`) in tests/*.rs.

Emits CoverageGap (MEDIUM, Confidence::Medium) for every derivation
ID with no matching test marker.
Emits LOW (Confidence::Low) for tests covering ID with no derivation
marker — likely typo or stale fixture.

## Public API

- `pub fn scan` — Public scanner entry — see Appendix A row "coverage_map".

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: crate, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
