//! Clap parses every subcommand with required + optional flags.

use clap::Parser;
use kei_llm_ollama::cli::{Cli, Cmd};

fn parse(argv: &[&str]) -> Cli {
    Cli::try_parse_from(argv).unwrap_or_else(|e| panic!("clap parse failed for {argv:?}: {e}"))
}

#[test]
fn tags_with_default_base_url() {
    let cli = parse(&["kei-llm-ollama", "tags"]);
    match cli.command {
        Cmd::Tags(o) => assert!(o.base_url.starts_with("http://127.0.0.1")),
        other => panic!("expected Tags, got {other:?}"),
    }
}

#[test]
fn tags_with_explicit_base_url() {
    let cli = parse(&[
        "kei-llm-ollama",
        "tags",
        "--base-url",
        "http://127.0.0.1:9999",
    ]);
    match cli.command {
        Cmd::Tags(o) => assert_eq!(o.base_url, "http://127.0.0.1:9999"),
        other => panic!("expected Tags, got {other:?}"),
    }
}

#[test]
fn generate_with_required_flags() {
    let cli = parse(&[
        "kei-llm-ollama",
        "generate",
        "--model",
        "qwen3:4b",
        "--prompt",
        "hi",
    ]);
    match cli.command {
        Cmd::Generate(o) => {
            assert_eq!(o.model, "qwen3:4b");
            assert_eq!(o.prompt, "hi");
            assert!(!o.stream);
        }
        other => panic!("expected Generate, got {other:?}"),
    }
}

#[test]
fn generate_with_stream_and_temperature() {
    let cli = parse(&[
        "kei-llm-ollama",
        "generate",
        "--model",
        "qwen3:4b",
        "--prompt",
        "hi",
        "--stream",
        "--temperature",
        "0.7",
        "--max-tokens",
        "32",
    ]);
    match cli.command {
        Cmd::Generate(o) => {
            assert!(o.stream);
            assert_eq!(o.temperature, Some(0.7));
            assert_eq!(o.max_tokens, Some(32));
        }
        other => panic!("expected Generate, got {other:?}"),
    }
}

#[test]
fn chat_with_inline_messages() {
    let cli = parse(&[
        "kei-llm-ollama",
        "chat",
        "--model",
        "qwen3:4b",
        "--messages",
        r#"[{"role":"user","content":"hi"}]"#,
    ]);
    match cli.command {
        Cmd::Chat(o) => {
            assert_eq!(o.model, "qwen3:4b");
            assert!(o.messages.starts_with('['));
        }
        other => panic!("expected Chat, got {other:?}"),
    }
}

#[test]
fn chat_with_at_path() {
    let cli = parse(&[
        "kei-llm-ollama",
        "chat",
        "--model",
        "qwen3:4b",
        "--messages",
        "@/tmp/messages.json",
    ]);
    match cli.command {
        Cmd::Chat(o) => assert!(o.messages.starts_with('@')),
        other => panic!("expected Chat, got {other:?}"),
    }
}

#[test]
fn pull_with_model() {
    let cli = parse(&["kei-llm-ollama", "pull", "--model", "qwen3:4b"]);
    match cli.command {
        Cmd::Pull(o) => assert_eq!(o.model, "qwen3:4b"),
        other => panic!("expected Pull, got {other:?}"),
    }
}

#[test]
fn health_with_default_base_url() {
    let cli = parse(&["kei-llm-ollama", "health"]);
    match cli.command {
        Cmd::Health(o) => assert!(o.base_url.contains("127.0.0.1")),
        other => panic!("expected Health, got {other:?}"),
    }
}

#[test]
fn health_with_timeout_ms() {
    let cli = parse(&[
        "kei-llm-ollama",
        "health",
        "--timeout-ms",
        "500",
    ]);
    match cli.command {
        Cmd::Health(o) => assert_eq!(o.timeout_ms, Some(500)),
        other => panic!("expected Health, got {other:?}"),
    }
}

#[test]
fn missing_required_flag_errors() {
    // `generate` requires --model and --prompt
    let r = Cli::try_parse_from(["kei-llm-ollama", "generate", "--prompt", "hi"]);
    assert!(r.is_err(), "generate without --model must error");
}
