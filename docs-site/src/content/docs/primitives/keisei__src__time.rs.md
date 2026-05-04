---
title: time.rs
path: keisei/src/time.rs
dna_hash: sha256:afb059e2cfda7375
language: rust
size_loc: 90
generated: by-keidocs
---

# keisei/src/time.rs

RFC-3339-ish UTC timestamp helpers.

Constructor Pattern: single responsibility — compute the current UTC
wall-clock as a `"YYYY-MM-DDThh:mm:ssZ"` string without pulling in
`chrono` for this one job. Extracted from `config.rs` in v0.22 so the
config module stays under the 200-LOC ceiling.

Uses Howard Hinnant's civil-from-days algorithm (public domain,
<http://howardhinnant.github.io/date_algorithms.html>). Tested against
four anchor dates including the century non-leap edge case
(`2100-03-01`, NOT Feb 29).

## Public API

- `pub fn now_utc_string` — Current UTC time as `"YYYY-MM-DDThh:mm:ssZ"`. Falls back to epoch on
- `pub fn format_epoch_utc` — Format a Unix epoch (seconds) as UTC `"YYYY-MM-DDThh:mm:ssZ"`.
- `pub fn civil_from_days` — Civil-from-days (Howard Hinnant). `z` is days since 1970-01-01 (may be

## Related

- parent: `keisei/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
