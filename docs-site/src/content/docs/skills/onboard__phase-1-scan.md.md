---
title: phase-1-scan
path: onboard/phase-1-scan.md
dna_hash: sha256:e47e1df2c0a7fda3
language: markdown
size_loc: 116
generated: by-keidocs
---

# onboard/phase-1-scan.md

## Public API

- `Phase 1 — Intake + Scan` — Free-text intake of target paths, then read-only Bash sweep of declarative
- `1a — Ask for the path(s)` — Emit a regular message (NOT AskUserQuestion):
- `1b — Scope-granularity click (AskUserQuestion, conditional)` — Only emit this call if `len(PATHS) > 1`. For a single project, skip to 1c.
- `1c — Read-only scan (Bash, per project)` — For each path in `PATHS`, run a single Bash sweep. Every command must be
- `Framework / package manifests` — ls -1 "$P"/package.json "$P"/Cargo.toml "$P"/pyproject.toml \
- `CI` — ls -1 "$P"/.github/workflows/*.yml "$P"/.github/workflows/*.yaml \
- `Deploy` — ls -1 "$P"/docker-compose*.yml "$P"/docker-compose*.yaml \
- `Tests` — ls -1d "$P"/tests "$P"/test "$P"/__tests__ "$P"/spec 2>/dev/null
- `README (first 100 lines — purpose extraction)` — head -n 100 "$P"/README.md 2>/dev/null
- `Recent activity (log ONLY; no writes — git is BLOCKED for state change` — 
- `but read-only log is the only inspection primitive available)` — git -C "$P" log --oneline -20 2>/dev/null
- `Env surface — schema files ONLY, never actual secrets` — ls -1 "$P"/.env.example "$P"/.env.template \
- `1d — Structured summary per project` — Produce a markdown summary per project in the conversation (do NOT write
- `Scan: <project-path>` — Stack:        <package.json → Node/<framework> | Cargo.toml → Rust | pyproject → Python | pubspec → Flutter | go.mod → Go | Package.swift → Swift | NONE_DETECTED>
- `Verify-criterion` — - `PATHS` is non-empty and every entry resolves to a directory.

## Related

- parent: `onboard`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
