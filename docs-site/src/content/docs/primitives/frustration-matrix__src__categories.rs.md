---
title: categories.rs
path: frustration-matrix/src/categories.rs
dna_hash: sha256:65a4734e26b8cf50
language: rust
size_loc: 167
generated: by-keidocs
---

# frustration-matrix/src/categories.rs

Category seed table — one file, one responsibility (the "model").

Extend by editing ONLY this file. Each `Category` is a flat struct of
static metadata + uncompiled regex sources. `compile_all()` is the sole
factory: it walks the seed table once and returns compiled regexes
paired with the metadata. Callers never touch raw strings — they get a
`Vec<CompiledCategory>` and match against it.

Design:
* uncompiled `triggers` live as `&'static str` so they can be verified
at test time without any allocator;
* compiled regexes use `(?i)` prefix so all matching is case-insensitive,
which matters because user pushback appears in both Russian and English;
* every regex is compiled once at startup — never per-line.

Seed rationale (from the 22k-token Explore audit that motivated this CLI):
1. `conservative-framing` and `paradigm-slippage` were the two recurring
classes the audit found.
2. `data-contamination` was a one-off in the audit but is high-priority
for any pre-registered experiment work (see RULE 0.8 pre-registration).
3. `repeat-signal` is the strongest marker per Karpathy "Think Before
Coding" — user literally saying "again".
4. `frustration-tone` is the base-rate surface signal.

## Public API

- `pub struct Category` — Metadata for one frustration class (raw, un-compiled).
- Short slug — machine id, used in CSV/JSONL output.
- Human-readable name — used in report tables.
- Uncompiled regex sources; compiled once at startup.
- Severity multiplier — weighted score = count * weight.
- Free-text context note for the reader (not matched).
- `pub struct CompiledCategory` — Compiled counterpart — regex list + preserved metadata.
- `pub const CATEGORIES` — The seed table — 5 categories. Order defines tie-break order in reports.
- `pub fn compile_all` — Compile every trigger in every category. Called once from `main` / tests.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: regex

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
