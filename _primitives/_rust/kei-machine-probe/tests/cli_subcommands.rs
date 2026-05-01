//! clap parser shapes — each subcommand parses with required + optional flags.

use clap::Parser;
use kei_machine_probe::cli::{Cli, Cmd};

#[test]
fn probe_with_no_flags() {
    let cli = Cli::try_parse_from(["kei-machine-probe", "probe"]).expect("probe");
    match cli.cmd {
        Cmd::Probe { mock_dir, no_tooling } => {
            assert!(mock_dir.is_none());
            assert!(!no_tooling);
        }
        _ => panic!("expected Probe"),
    }
}

#[test]
fn probe_with_mock_dir_and_no_tooling() {
    let cli = Cli::try_parse_from([
        "kei-machine-probe",
        "probe",
        "--mock-dir",
        "/tmp/fixtures",
        "--no-tooling",
    ])
    .expect("probe with flags");
    match cli.cmd {
        Cmd::Probe { mock_dir, no_tooling } => {
            assert_eq!(mock_dir.as_deref().unwrap().to_str().unwrap(), "/tmp/fixtures");
            assert!(no_tooling);
        }
        _ => panic!("expected Probe"),
    }
}

#[test]
fn capabilities_parses() {
    let cli =
        Cli::try_parse_from(["kei-machine-probe", "capabilities"]).expect("capabilities");
    matches!(cli.cmd, Cmd::Capabilities { .. });
}

#[test]
fn report_with_markdown_flag() {
    let cli = Cli::try_parse_from(["kei-machine-probe", "report", "--markdown"])
        .expect("report --markdown");
    match cli.cmd {
        Cmd::Report { mock_dir, markdown } => {
            assert!(mock_dir.is_none());
            assert!(markdown);
        }
        _ => panic!("expected Report"),
    }
}

#[test]
fn unknown_subcommand_errors() {
    assert!(Cli::try_parse_from(["kei-machine-probe", "garbage"]).is_err());
}
