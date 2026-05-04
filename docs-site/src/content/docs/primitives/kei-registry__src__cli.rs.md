---
title: cli.rs
path: kei-registry/src/cli.rs
dna_hash: sha256:e5e38d163518bee0
language: rust
size_loc: 198
generated: by-keidocs
---

# kei-registry/src/cli.rs

clap surface — 8 subcommands.

Constructor Pattern: this cube owns argument parsing only. Dispatch
lives in `main.rs`; each handler delegates to `registry`, `related`,
`diff`, or `stats` modules. Default `--db` is resolved at run-time so
the path expansion can react to `$HOME`.

## Public API

- Initialise the SQLite store at `--db`. Idempotent.
- Walk kit + claude dirs, register all detected blocks.
- Comma-separated subset of {primitive,skill,rule,hook,atom}.
- Manually register a single block.
- List blocks (active by default).
- Restrict to one block_type.
- Fetch a single block by integer id OR full DNA.
- Find blocks whose body references the target.
- Facet-by-facet diff between two blocks.
- Aggregate counts.
- Generate a human-readable encyclopedia from registry rows.
- Path to the registry SQLite (default: $KEI_REGISTRY_DB or ~/.claude/registry.sqlite).
- Write output to this path instead of stdout.
- Output format: markdown (default) or json.
- Restrict to one block_type (primitive | skill | rule | hook | atom).
- Shorthand: register a single skill directory (must contain SKILL.md).
- Path to the skill directory (containing SKILL.md). Defaults to `.`.
- Shorthand: register a single .sh hook file.
- Walk all substrate dirs under KIT-ROOT and register each artefact.
- Path to the kit root (defaults to current directory).
- Print counts without writing to the registry.
- Cross-cutting status dashboard: blocks per type, registered path
- Registry SQLite path (default: `$KEI_REGISTRY_DB` or
- Local git repo to scan for branches (default: current dir).
- kei-ledger SQLite path (default: `$KEI_LEDGER_DB` or
- Output format: `ascii` (default) or `json`.
- Audit secret/env-var references across the kit. Reads env-var NAMES
- Env-file paths to scan (default: `~/.claude/secrets/.env` if
- Root to scan for usages (default: current directory).
- Output format: `ascii` (default) or `json`.
- Phase 3 Layer 3 — pipe a RULE 0.16 STATUS-TRUTH MARKER (from

## Related

- parent: `kei-registry/Cargo.toml`
- imports: clap, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
