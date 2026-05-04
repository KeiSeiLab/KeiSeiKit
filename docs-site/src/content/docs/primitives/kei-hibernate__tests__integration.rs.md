---
title: integration.rs
path: kei-hibernate/tests/integration.rs
dna_hash: sha256:ffb54e67f34990a5
language: rust
size_loc: 191
generated: by-keidocs
---

# kei-hibernate/tests/integration.rs

Core integration tests — the 6 cases named in the spec.

1. export_round_trip
2. export_excludes_non_kei_files
3. import_dry_run_makes_no_changes
4. import_refuses_version_mismatch
5. inspect_lists_contents
6. manifest_sha256_verified_on_import

## Public API

- Defence-in-depth: our importer rejects archive entries containing

## Related

- parent: `kei-hibernate/tests`
- imports: common, kei_hibernate, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
