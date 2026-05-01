//! Generate — non-streaming stdout parsing.
//!
//! mlx_lm prints the generated text, then `==========` separator, then a
//! footer with token / tokens-per-sec stats. We pin the parser against
//! a representative fixture so version drift in mlx_lm output style
//! is caught here, not in production.

use kei_llm_mlx::generate::{parse_response, build_argv, GenerateOpts};

const STDOUT_SAMPLE: &str = "\
Once upon a time, in a far-off land, there lived a curious cat.
==========
Prompt: 12 tokens, 132.4 tokens-per-sec
Generation: 64 tokens, 78.9 tokens-per-sec
";

#[test]
fn footer_yields_typed_response() {
    let r = parse_response(STDOUT_SAMPLE, "mlx-community/Llama-3.2-3B-Instruct-4bit", "Once upon")
        .expect("parse ok");
    assert!(r.text.starts_with("Once upon a time"));
    assert!(!r.text.contains("=========="));
    assert_eq!(r.prompt_tokens, Some(12));
    assert_eq!(r.generation_tokens, Some(64));
    assert!(r.tokens_per_sec.is_some());
    assert_eq!(r.model_id, "mlx-community/Llama-3.2-3B-Instruct-4bit");
}

#[test]
fn argv_carries_optional_flags() {
    let argv = build_argv(
        "mlx-community/x-4bit",
        "hi",
        &GenerateOpts { max_tokens: Some(64), temperature: Some(0.7) },
    );
    assert!(argv.contains(&"--model".into()));
    assert!(argv.contains(&"mlx-community/x-4bit".to_string()));
    assert!(argv.contains(&"--max-tokens".into()));
    assert!(argv.contains(&"64".to_string()));
    assert!(argv.contains(&"--temp".into()));
}
