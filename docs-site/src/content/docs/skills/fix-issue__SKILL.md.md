---
title: SKILL
path: fix-issue/SKILL.md
dna_hash: sha256:941d6c659e21f1ad
language: markdown
size_loc: 52
generated: by-keidocs
---

# fix-issue/SKILL.md

## Public API

- `Fix Issue Workflow` — ---
- `When to use` — - Fixing a tracked GitHub issue: reproduce → trace root cause → fix → add regression test.
- `Step 1: Load Issue Context` — - Fetch issue details: `gh issue view $issue --json title,body,labels,comments`
- `Step 2: Reproduce` — - Set up reproduction environment based on issue description
- `Step 3: Trace Root Cause` — - Use Grep/Glob to find relevant code paths
- `Step 4: Checkpoint` — - `git commit` current state: `checkpoint: before fix-issue #$issue`
- `Step 5: Fix` — - Fix at the root cause level, NOT overlay/patch
- `Step 6: Verify` — - Run the failing test — must pass now
- `Step 7: Commit` — - `fix: <description of what was fixed> (closes #$issue)`

## Related

- parent: `fix-issue`

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
