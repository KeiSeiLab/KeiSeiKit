---
title: anthropic_invoker.rs
path: kei-cortex/src/anthropic_invoker.rs
dna_hash: sha256:77933792ee604cad
language: rust
size_loc: 168
generated: by-keidocs
---

# kei-cortex/src/anthropic_invoker.rs

Non-streaming Anthropic Messages call with tool-use support.
`tool::run_with_tools` needs a `ModelInvoker` returning a full `ModelTurn`
(content blocks + stop_reason); the streaming `anthropic::open_stream` is
text-only. This module POSTs once per turn and parses Anthropic's
multi-block content (`text` + `tool_use`) into `Vec<ContentBlock>`.

## Public API

- Overall HTTP budget per turn. Anthropic non-streaming responses can be
- `pub fn build_invoker` — Build a `ModelInvoker` that POSTs to Anthropic Messages with tools.
- Single round-trip to Anthropic Messages with tools. Errors stringify so
- Build the JSON body. Local messages map to API messages: User→user,
- Translate the local conversation history into the Anthropic API shape.
- Fire the POST request and surface 4xx/5xx as a string error.
- Parse the response body into a `ModelTurn`. Unknown content-block types
- Pull `usage.input_tokens` + `usage.output_tokens` out of the Anthropic

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, serde_json, std, tokio

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
