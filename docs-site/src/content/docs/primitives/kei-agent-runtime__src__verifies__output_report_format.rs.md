---
title: output_report_format.rs
path: kei-agent-runtime/src/verifies/output_report_format.rs
dna_hash: sha256:6d8af4b8ce65abf9
language: rust
size_loc: 57
generated: by-keidocs
---

# kei-agent-runtime/src/verifies/output_report_format.rs

`output::report-format` verify — reads agent's final report (env var
`AGENT_REPORT_PATH` or `.claude/agents/<id>/review.md`), asserts every
field in `task.output.report-fields-required` is mentioned.

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate, std

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
