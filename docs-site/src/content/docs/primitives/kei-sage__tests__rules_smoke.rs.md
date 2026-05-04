---
title: rules_smoke.rs
path: kei-sage/tests/rules_smoke.rs
dna_hash: sha256:f518605f54dbce7e
language: rust
size_loc: 132
generated: by-keidocs
---

# kei-sage/tests/rules_smoke.rs

Integration smoke test for rule discovery + atom→rule edge persistence.

Creates a temp rules tree with 2 rule files (flat dir), asserts
`discover_rules` extracts slugs + heading names correctly. Then stages
an atom whose `related:` lists one of those rules and asserts
`index_rule_edges` persists a `rule_ref` edge into the store.

## Related

- parent: `kei-sage/tests`
- imports: kei_sage, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
