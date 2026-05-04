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
