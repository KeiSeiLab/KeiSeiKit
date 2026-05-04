---
title: provider.rs
path: kei-router/src/provider.rs
dna_hash: sha256:1058dc4770dfbc15
language: rust
size_loc: 89
generated: by-keidocs
---

# kei-router/src/provider.rs

LLM provider trait — minimal abstraction across Anthropic, OpenAI, Kimi.

Constructor Pattern: types here are wire-format-agnostic. Each provider impl
translates these into its own request shape and back into `StreamEvent`s.

Wave 32 v0.40 design notes:
- `Message` is provider-agnostic role+content. Tool-use blocks not modelled
in v0.40 (text-only streaming first; tool-call streaming added in v0.41).
- `StreamEvent::Token` is the only event a v0.40 caller needs to render.
Other variants exist so v0.41 tool-call streaming doesn't break the trait.
- Cost cents are integer per-million-tokens. Sub-cent pricing rounds up.

## Public API

- One conversation turn. Role is "user" | "assistant" | "system" depending
- Tool definition exposed to the model. Schema is a JSON Schema object;
- One event from the streaming response. v0.40 implementations emit
- Incremental text token.
- Tool-call name signalled (v0.41).
- Tool-call argument fragment (v0.41).
- Stream completed cleanly.
- Errors a provider may surface. `Upstream` carries a status + truncated body.
- Provider trait. Each implementation is one cube (one file).
- Stable wire name, e.g. "anthropic" / "openai" / "kimi".
- Cents per 1M input tokens. Used by `LlmRouter::cheapest_for_*`.
- Cents per 1M output tokens.
- Open a streaming Messages request. Returns a stream after the

## Related

- parent: `kei-router/Cargo.toml`
- imports: async_trait, futures, serde

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
