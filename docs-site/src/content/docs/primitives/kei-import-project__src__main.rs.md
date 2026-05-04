---
title: main.rs
path: kei-import-project/src/main.rs
dna_hash: sha256:4b12192889ba41b8
language: rust
size_loc: 347
generated: by-keidocs
---

# kei-import-project/src/main.rs

kei-import-project CLI — decompose foreign codebases into module tables.

Subcommands:
decompose <PATH>              — walk + identify modules, print markdown table
register <PATH> [--registry-db] — walk + identify + write to kei-registry
map <PATH> [--threshold] [--format] — walk + match traits, print summary
extract-skills                — stub (Phase 3)
plan                          — stub (Phase 4)
execute                       — stub (Phase 5)

## Public API

- Walk a path and identify language modules, printing a markdown summary table.
- Path to the repository root.
- Walk a repo, identify modules, and register each in kei-registry.
- Path to the repository root.
- Override registry SQLite path (default: $KEI_REGISTRY_DB or ~/.claude/registry.sqlite).
- Map modules to kei-runtime-core traits via confidence-scored matching.
- Path to the repository root.
- Only show matches at or above this confidence (default 0.3).
- Output format: markdown (default) or json.
- Generate a Rust impl skeleton for a module implementing a specific trait.
- Path to the module directory (or crate root).
- Trait to implement (e.g. compute-provider, memory-backend).
- Output file path. Defaults to stdout.
- Extract skills from foreign repo's README + docs/*.md and register in kei-registry.
- Path to the foreign repo to analyse.
- Project slug (default: basename of repo_path).
- Directory to write canonical SKILL.md fragments into.
- Path to kei-registry SQLite (default: $KEI_REGISTRY_DB or ~/.claude/registry.sqlite).
- Dry run: print plan, write nothing, register nothing.
- Generate a migration plan from a repo's architecture map.
- Path to the repository root.
- Human-readable project name (default: basename of path).
- Confidence threshold; modules below → unmatched (default 0.5).
- Output file path. Defaults to stdout.
- Execute the migration plan: parse plan.md, emit per-phase agent prompts.
- Path to the plan.md file produced by the `plan` subcommand.
- Override kei-ledger SQLite path (default: $KEI_LEDGER_DB or ~/.claude/agents/ledger.sqlite).
- Pre-register each phase as a 'queued' row in kei-ledger.
- Output format: markdown (default) or json.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: clap, kei_import_project, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
