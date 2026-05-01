//! clap parses all 5 subcommands.
//!
//! Pure parser test: we just verify the `Cli` struct accepts the canonical
//! invocations for `probe`, `models`, `generate`, `server`, `version`. No
//! dispatch — that path is exercised by the integration smoke during
//! cargo build.

use clap::Parser;
use kei_llm_mlx::cli::{Cli, Cmd};

#[test]
fn parses_all_five() {
    let probe = Cli::parse_from(["kei-llm-mlx", "probe"]);
    assert!(matches!(probe.cmd, Cmd::Probe));

    let models = Cli::parse_from(["kei-llm-mlx", "models"]);
    assert!(matches!(models.cmd, Cmd::Models { .. }));

    let gen = Cli::parse_from([
        "kei-llm-mlx",
        "generate",
        "--model",
        "mlx-community/x-4bit",
        "--prompt",
        "hi",
        "--max-tokens",
        "32",
    ]);
    if let Cmd::Generate { model, prompt, max_tokens, temperature, stream } = gen.cmd {
        assert_eq!(model, "mlx-community/x-4bit");
        assert_eq!(prompt, "hi");
        assert_eq!(max_tokens, Some(32));
        assert!(temperature.is_none());
        assert!(!stream);
    } else {
        panic!("expected Generate");
    }

    let srv = Cli::parse_from(["kei-llm-mlx", "server", "--model", "x", "--port", "9090"]);
    if let Cmd::Server { model, port, host } = srv.cmd {
        assert_eq!(model, "x");
        assert_eq!(port, 9090);
        assert_eq!(host, "127.0.0.1", "host default must be loopback");
    } else {
        panic!("expected Server");
    }

    let ver = Cli::parse_from(["kei-llm-mlx", "version"]);
    assert!(matches!(ver.cmd, Cmd::Version));
}
