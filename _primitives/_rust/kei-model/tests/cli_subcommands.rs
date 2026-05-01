//! Smoke-test each clap subcommand parses with representative flags.

use clap::Parser;

use kei_model::cli::{Cli, Cmd};

#[test]
fn parses_list() {
    let cli = Cli::try_parse_from(["kei-model", "list", "--provider", "anthropic"]).unwrap();
    match cli.cmd {
        Cmd::List(a) => assert_eq!(a.provider.as_deref(), Some("anthropic")),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_resolve_with_budget_and_caps() {
    let cli = Cli::try_parse_from([
        "kei-model",
        "resolve",
        "--role",
        "code-implementer",
        "--budget-micro",
        "50000",
        "--cap",
        "code,vision",
    ])
    .unwrap();
    match cli.cmd {
        Cmd::Resolve(a) => {
            assert_eq!(a.role, "code-implementer");
            assert_eq!(a.budget_micro, Some(50_000));
            assert_eq!(a.cap.as_deref(), Some("code,vision"));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_price() {
    let cli = Cli::try_parse_from([
        "kei-model",
        "price",
        "claude-opus-4-7",
        "--input-tokens",
        "1000",
        "--output-tokens",
        "500",
    ])
    .unwrap();
    match cli.cmd {
        Cmd::Price(a) => {
            assert_eq!(a.model_id, "claude-opus-4-7");
            assert_eq!(a.input_tokens, 1000);
            assert_eq!(a.output_tokens, 500);
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_providers() {
    let cli = Cli::try_parse_from(["kei-model", "providers"]).unwrap();
    matches!(cli.cmd, Cmd::Providers(_));
}

#[test]
fn parses_fallback() {
    let cli = Cli::try_parse_from([
        "kei-model",
        "fallback",
        "--primary",
        "claude-opus-4-7",
    ])
    .unwrap();
    match cli.cmd {
        Cmd::Fallback(a) => assert_eq!(a.primary, "claude-opus-4-7"),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn rejects_unknown_subcommand() {
    let r = Cli::try_parse_from(["kei-model", "invalid-verb"]);
    assert!(r.is_err());
}
