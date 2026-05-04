---
title: duplication
path: architecture-rules/references/duplication.md
dna_hash: sha256:2aded991107965a0
language: markdown
size_loc: 178
generated: by-keidocs
---

# architecture-rules/references/duplication.md

## Public API

- `Duplication Detection Rules` — 
- `Threshold Matrix` — | Duplication Type | Threshold | Action |
- `Detection Methods` — ### Method 1: Structural Grep
- `Find duplicate function bodies (exact)` — grep -rn "function_pattern" src/ | sort | uniq -d
- `Find similar patterns with context` — grep -rn "pattern" src/ --include="*.ts" -A 5
- `Files that always change together (shotgun surgery)` — git log --name-only --pretty=format: | sort | uniq -c | sort -rn
- `Similar commit patterns` — git log --all --oneline | grep -i "fix.*same\|duplicate\|copy"
- `Extraction Decision Tree` — ```
- `What IS and ISN'T Duplication` — ### IS Duplication (extract):
- `Refactoring Patterns for Duplication` — ### Pattern 1: Extract Function
- `Cross-Module Duplication Rules` — ### Where to Put Shared Code

## Related

- parent: `architecture-rules/references`

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
