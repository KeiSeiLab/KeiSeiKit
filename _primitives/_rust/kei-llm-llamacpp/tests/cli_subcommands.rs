//! clap parses each of the 5 subcommands with required + optional flags.
//! Exercises both required-arg presence and flag default values.

use clap::Parser;
use kei_llm_llamacpp::cli::{Cli, Cmd};

#[test]
fn probe_subcommand_parses() {
    let cli = Cli::try_parse_from(["kei-llm-llamacpp", "probe"]).unwrap();
    assert!(matches!(cli.cmd, Cmd::Probe));
}

#[test]
fn models_subcommand_with_dir() {
    let cli = Cli::try_parse_from([
        "kei-llm-llamacpp",
        "models",
        "--dir",
        "/tmp/llama-models",
    ])
    .unwrap();
    match cli.cmd {
        Cmd::Models { dir } => assert_eq!(dir.unwrap().to_str().unwrap(), "/tmp/llama-models"),
        other => panic!("expected Models, got {other:?}"),
    }
}

#[test]
fn models_subcommand_without_dir() {
    let cli = Cli::try_parse_from(["kei-llm-llamacpp", "models"]).unwrap();
    match cli.cmd {
        Cmd::Models { dir } => assert!(dir.is_none()),
        other => panic!("expected Models, got {other:?}"),
    }
}

#[test]
fn generate_subcommand_with_all_flags() {
    let cli = Cli::try_parse_from([
        "kei-llm-llamacpp",
        "generate",
        "--model",
        "/tmp/m.gguf",
        "--prompt",
        "hello",
        "--max-tokens",
        "256",
        "--temperature",
        "0.4",
        "--stream",
    ])
    .unwrap();
    match cli.cmd {
        Cmd::Generate { model, prompt, max_tokens, temperature, stream } => {
            assert_eq!(model.to_str().unwrap(), "/tmp/m.gguf");
            assert_eq!(prompt, "hello");
            assert_eq!(max_tokens, 256);
            assert_eq!(temperature, Some(0.4));
            assert!(stream);
        }
        other => panic!("expected Generate, got {other:?}"),
    }
}

#[test]
fn generate_subcommand_default_max_tokens() {
    let cli = Cli::try_parse_from([
        "kei-llm-llamacpp",
        "generate",
        "--model",
        "/tmp/m.gguf",
        "--prompt",
        "hi",
    ])
    .unwrap();
    match cli.cmd {
        Cmd::Generate { max_tokens, temperature, stream, .. } => {
            assert_eq!(max_tokens, 128);
            assert!(temperature.is_none());
            assert!(!stream);
        }
        other => panic!("expected Generate, got {other:?}"),
    }
}

#[test]
fn server_subcommand_default_host_and_port() {
    let cli = Cli::try_parse_from([
        "kei-llm-llamacpp",
        "server",
        "--model",
        "/tmp/m.gguf",
    ])
    .unwrap();
    match cli.cmd {
        Cmd::Server { host, port, .. } => {
            assert_eq!(host, "127.0.0.1");
            assert_eq!(port, 8080);
        }
        other => panic!("expected Server, got {other:?}"),
    }
}

#[test]
fn version_subcommand_parses() {
    let cli = Cli::try_parse_from(["kei-llm-llamacpp", "version"]).unwrap();
    assert!(matches!(cli.cmd, Cmd::Version));
}

#[test]
fn missing_required_flag_errors() {
    // generate without --model must fail.
    let res = Cli::try_parse_from([
        "kei-llm-llamacpp",
        "generate",
        "--prompt",
        "hi",
    ]);
    assert!(res.is_err(), "generate without --model must error");
}
