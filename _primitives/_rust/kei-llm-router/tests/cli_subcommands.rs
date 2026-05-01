//! Test 7 — clap parses all four subcommands.
//!
//! Constructor Pattern: this test exists ONLY to lock the CLI surface;
//! a regression renaming a subcommand or dropping a flag will fail here
//! before it lands in production.

use clap::{CommandFactory, Parser};
use kei_llm_router::cli::{Cli, Command};

#[test]
fn debug_assert_passes() {
    Cli::command().debug_assert();
}

#[test]
fn parses_probe_skip_tooling() {
    let cli = Cli::try_parse_from(["kei-llm-router", "probe", "--skip-tooling"]).unwrap();
    match cli.command {
        Command::Probe(args) => assert!(args.skip_tooling),
        other => panic!("expected Probe, got {other:?}"),
    }
}

#[test]
fn parses_route_with_require_local() {
    let cli = Cli::try_parse_from([
        "kei-llm-router",
        "route",
        "--model",
        "qwen3:4b",
        "--require-local",
    ])
    .unwrap();
    match cli.command {
        Command::Route(args) => {
            assert_eq!(args.model, "qwen3:4b");
            assert!(args.require_local);
            assert!(args.role.is_none());
            assert!(args.budget_micro.is_none());
        }
        other => panic!("expected Route, got {other:?}"),
    }
}

#[test]
fn parses_list_backends() {
    let cli = Cli::try_parse_from(["kei-llm-router", "list-backends"]).unwrap();
    matches!(cli.command, Command::ListBackends);
}

#[test]
fn parses_which_with_model() {
    let cli =
        Cli::try_parse_from(["kei-llm-router", "which", "--model", "qwen3:4b"]).unwrap();
    match cli.command {
        Command::Which(args) => assert_eq!(args.model, "qwen3:4b"),
        other => panic!("expected Which, got {other:?}"),
    }
}

#[test]
fn parses_route_with_role_and_budget() {
    let cli = Cli::try_parse_from([
        "kei-llm-router",
        "route",
        "--model",
        "llama-3-70b-local",
        "--role",
        "edit-local",
        "--budget-micro",
        "500",
    ])
    .unwrap();
    match cli.command {
        Command::Route(args) => {
            assert_eq!(args.role.as_deref(), Some("edit-local"));
            assert_eq!(args.budget_micro, Some(500));
            assert!(!args.require_local);
        }
        other => panic!("expected Route, got {other:?}"),
    }
}
