---
title: main.rs
path: kei-skill-importer/src/main.rs
dna_hash: sha256:dac6ac8adfe65840
language: rust
size_loc: 190
generated: by-keidocs
---

# kei-skill-importer/src/main.rs

kei-skill-importer CLI — `parse` (JSON), `convert` (write file),
`batch` (walk + convert; JSONL summary). Info logs go to stderr;
stdout is reserved for machine-readable output.

## Public API

- Parse a skill file and print canonical JSON to stdout.
- Parse + decide emit path + write file(s) into <output_dir>.
- Walk <input_dir> and convert every candidate file.

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: anyhow, clap, kei_skill_importer, serde, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
