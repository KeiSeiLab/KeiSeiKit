---
title: main.rs
path: kei-artifact/src/main.rs
dna_hash: sha256:4ec95b4fe3657fef
language: rust
size_loc: 143
generated: by-keidocs
---

# kei-artifact/src/main.rs

kei-artifact CLI — register-schema / emit / get / list / validate / chain.

Constructor Pattern: main.rs = dispatch only. Each `cmd_*` fn < 30 LOC.
Artifact-CRUD command bodies live in the `cli_cmds` sibling module.

## Public API

- Initialise the db and register the 5 built-in schemas.
- Register a JSON Schema file under a name. Also refreshes the export
- List all registered schema names, one per line (plain text).
- v0.16: write the current schema-name list as JSON so the assembler's
- Emit an artifact. Content file must be JSON.
- Fetch an artifact by id.
- List artifacts; filter by schema / source / since-seconds.
- Re-validate a stored artifact against its schema.
- Walk the parent-handoff chain.

## Related

- parent: `kei-artifact/Cargo.toml`
- imports: clap, kei_artifact, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
