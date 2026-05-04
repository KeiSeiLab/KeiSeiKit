---
title: generate.rs
path: kei-llm-llamacpp/src/generate.rs
dna_hash: sha256:3fb0c7ad81f30cb8
language: rust
size_loc: 136
generated: by-keidocs
---

# kei-llm-llamacpp/src/generate.rs

Generate — non-streaming `llama-cli` invocation.

Shells out to `llama-cli -m <path> -p <prompt> -n <n>` (plus optional
`--temp`) and parses the trailing timing footer. Output:
`Response { text, eval_tokens, eval_ms, tokens_per_sec }`.

Footer format (llama.cpp >= b3000): line of the form
"llama_perf_context_print: eval time = 1234.56 ms / 12 runs ..."
We tolerate older builds that emit "llama_print_timings" with the
same fields.

## Public API

- User-facing options for a generate call.
- Parsed result from a non-streaming `llama-cli` run.
- `pub fn build_args` — Build the argv passed to `llama-cli` for a non-streaming call.
- Run a non-streaming generate via the supplied Runner.
- `pub fn parse_stdout` — Parse the captured stdout+stderr into a Response. The "answer text"
- Split the model output into (answer_text, footer_block). The footer
- Pull `(eval_tokens, eval_ms)` out of the timing footer.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: crate, regex, serde, std

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
