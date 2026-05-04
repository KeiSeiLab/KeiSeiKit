---
title: injection_patterns.rs
path: kei-memory/src/injection_patterns.rs
dna_hash: sha256:b738b657bdb9b14e
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-memory/src/injection_patterns.rs

Injection-pattern table for `injection_guard`.

Constructor Pattern: this cube only declares regex/string patterns.
Detection logic lives in `injection_guard.rs`. Test corpus in
`tests/guard_test_corpus.rs`. Each entry carries a stable id, a
severity (`Block` or `Warn`), and a human-readable source label so
triage output points back to the heuristic that fired.

## Public API

- Severity of a single pattern match.
- Hard reject — content must not be persisted.
- Surface to caller; persistence still allowed unless caller upgrades.
- `pub struct RegexPattern` — One regex-based pattern row.
- `pub struct SubstringPattern` — One substring/heuristic row evaluated on a lower-cased copy of input.
- All needles must appear (AND semantics).
- `pub const INVISIBLE_CHARS` — Invisible / bidi / zero-width unicode codepoints.
- Threshold above which a single base64-looking line is flagged.
- PEM marker dashes — built at runtime to keep `secrets-guard` quiet.
- `pub fn regex_patterns` — Lazily-built regex pattern table.
- `pub fn substring_patterns` — Lazily-built substring/heuristic table.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
