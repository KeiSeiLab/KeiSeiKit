---
title: report.rs
path: kei-cleanup/src/report.rs
dna_hash: sha256:0057e6249ace79cb
language: rust
size_loc: 149
generated: by-keidocs
---

# kei-cleanup/src/report.rs

Report types per CLEANUP-RUNTIME-SPEC.md §3.3.

`CleanupReport` is the structured output of a kei-cleanup run.
Each scanner contributes zero or more `Finding`s; the runtime
aggregates them, computes counts, and emits via TTY or JSON.

## Public API

- Top-level report — see CLEANUP-RUNTIME-SPEC.md §3.3.
- JSON schema version. v0.1 = 1.
- Workspace root that was scanned.
- Short git SHA of HEAD (or "unknown" if not a git workspace).
- UTC timestamp when the run started.
- Wall-clock runtime in milliseconds.
- All findings, in scanner order.
- Aggregate counts by severity.
- Names of scanners that ran successfully.
- Names of scanners skipped, with reason (e.g. "tool not found").
- Single hygiene finding — see CLEANUP-RUNTIME-SPEC.md §3.3.
- Category of finding.
- Severity bucket.
- File + optional line.
- Human-readable issue description.
- Optional textual fix suggestion.
- Optional cross-reference to a rule or spec section.
- Detector confidence (some scanners are heuristic).
- Finding category — see CLEANUP-RUNTIME-SPEC.md §3.3.
- `pub` item with zero workspace callers (machete / udeps).
- Dependency declared in Cargo.toml, never imported in src/.
- Member dep version differs from `[workspace.dependencies]`.
- File exceeds Constructor Pattern LOC limit.
- Function exceeds Constructor Pattern LOC limit.
- TODO/FIXME/XXX/HACK comment older than configured threshold.
- Derivation marker without matching test marker.
- Test references 2+ sibling crates (dev-dep cycle risk).
- `cargo doc` warning (broken intra-doc link, etc).
- Naming-pair drift: two variants of the same constant in use.
- Severity bucket — controls `--fail-on` and HIGH/MEDIUM/LOW grouping.
- Cosmetic / batch-with-next-refactor.
- Schedule into next cleanup window (≤7 d).
- Block merge to main; needs orchestrator decision.
- Detector confidence — distinguishes deterministic from heuristic scanners.
- Heuristic / may have false positives (e.g. cargo-machete).
- Deterministic (e.g. LOC count, version compare).
- File + optional line location.
- Path relative to workspace root if possible, else absolute.
- 1-based line, or None when not file-line specific.
- Aggregate severity counts.
- HIGH finding count.
- MEDIUM finding count.
- LOW finding count.
- Total = high + medium + low.
- `pub fn from_findings` — Build from a slice of findings — single pass, O(n).
- `pub fn parse_min` — Parse `--fail-on` argument: "high" / "medium" / "low".

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: chrono, serde, std

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
