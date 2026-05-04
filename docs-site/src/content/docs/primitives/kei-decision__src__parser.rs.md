---
title: parser.rs
path: kei-decision/src/parser.rs
dna_hash: sha256:6b0099b8138b08b5
language: rust
size_loc: 192
generated: by-keidocs
---

# kei-decision/src/parser.rs

Markdown action-table parser.

Looks for a section heading whose text matches one of:
- "Actionable plan"
- "Backlog"
- "Action items"
and extracts the markdown table that follows. Each table row becomes one
[`RawAction`]. Effort and severity are inferred from the row cells; deps
are parsed from a free-text "deps:" hint inside the action cell when
present.

No md crate — table format is well-defined: `| col | col | ... |`.

## Public API

- Local error enum — avoids pulling thiserror as new dep (RULE: no new deps).
- `pub fn parse_master_report` — Read MASTER-REPORT.md, locate first action-style section, return rows.
- Walk the doc, find a heading line that names an action section, then the
- Inspect the table-header pipe row and locate the columns we care about.
- Walk the body rows below the separator, build [`RawAction`] per row.
- Split a pipe-row into its inner cells (drop empty leading/trailing).
- `deps: 1, 2` or `(after #3)` → vec of id strings.

## Related

- parent: `kei-decision/Cargo.toml`
- imports: anyhow, regex, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
