---
title: cli_subcommands.rs
path: kei-llm-router/tests/cli_subcommands.rs
dna_hash: sha256:25f9de79912735ee
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-llm-router/tests/cli_subcommands.rs

Test 7 — clap parses all four subcommands.

Constructor Pattern: this test exists ONLY to lock the CLI surface;
a regression renaming a subcommand or dropping a flag will fail here
before it lands in production.

## Related

- parent: `kei-llm-router/tests`
- imports: clap, kei_llm_router

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
