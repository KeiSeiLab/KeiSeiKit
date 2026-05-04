---
title: plan_parser.rs
path: kei-import-project/src/plan_parser.rs
dna_hash: sha256:81152dd9508de44e
language: rust
size_loc: 151
generated: by-keidocs
---

# kei-import-project/src/plan_parser.rs

plan_parser — reverse of plan_render: parse plan.md → structured form.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- One module entry inside a parsed phase.
- One migration phase parsed from the plan.md per-phase detail section.
- The full parsed plan.
- `pub fn parse_plan` — Parse a plan.md string into a `ParsedPlan`.
- `pub fn parse_plan_file` — Read a plan.md file and parse it.
- `pub fn extract_phases` — Parse all `### Px.y — TraitFamily` blocks from the Per-phase detail section.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
