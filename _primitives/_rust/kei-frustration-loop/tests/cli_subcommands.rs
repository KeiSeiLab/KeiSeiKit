//! Clap parser smoke tests for the 5 v0.40 loop subcommands. The new crate
//! exposes the `cli` module via `kei_frustration_loop`, so we can import
//! the parser directly — no `#[path]` mounting needed.
//!
//! We only assert that clap accepts the flag combinations we documented;
//! actual side-effects are covered by the per-cube tests.

use clap::Parser;
use kei_frustration_loop::cli::{Cli, Cmd};

fn parse(argv: &[&str]) -> Cli {
    let mut full = vec!["kei-frustration-loop"];
    full.extend_from_slice(argv);
    Cli::try_parse_from(&full).unwrap_or_else(|e| panic!("parse failed for {argv:?}: {e}"))
}

#[test]
fn parse_bootstrap_minimal() {
    let cli = parse(&["bootstrap"]);
    let Cmd::Bootstrap { user, home } = cli.cmd else {
        panic!("wrong variant");
    };
    assert_eq!(user, "default");
    assert!(home.is_none());
}

#[test]
fn parse_bootstrap_full() {
    let cli = parse(&["bootstrap", "--user", "alice", "--home", "/tmp/h"]);
    let Cmd::Bootstrap { user, home } = cli.cmd else {
        panic!("wrong variant");
    };
    assert_eq!(user, "alice");
    assert_eq!(home.unwrap(), std::path::PathBuf::from("/tmp/h"));
}

#[test]
fn parse_nightly_scan_with_since() {
    let cli = parse(&[
        "nightly-scan", "--user", "bob", "--since", "1700000000", "--home", "/h",
    ]);
    let Cmd::NightlyScan { user, since, home } = cli.cmd else {
        panic!("wrong variant");
    };
    assert_eq!(user, "bob");
    assert_eq!(since, Some(1_700_000_000));
    assert!(home.is_some());
}

#[test]
fn parse_feedback_correct_label() {
    let cli = parse(&[
        "feedback", "hit-42", "--label", "correct", "--user", "u", "--message", "x",
    ]);
    let Cmd::Feedback {
        hit_id, label, user, ..
    } = cli.cmd else {
        panic!("wrong variant");
    };
    assert_eq!(hit_id, "hit-42");
    assert_eq!(label, "correct");
    assert_eq!(user, "u");
}

#[test]
fn parse_feedback_new_category_label() {
    let cli = parse(&[
        "feedback", "h-1", "--label", "new:bias",
    ]);
    let Cmd::Feedback { label, .. } = cli.cmd else {
        panic!("wrong variant");
    };
    assert_eq!(label, "new:bias");
}

#[test]
fn parse_auto_train_with_threshold_override() {
    let cli = parse(&[
        "auto-train", "--user", "u", "--threshold", "33",
    ]);
    let Cmd::AutoTrain {
        user, threshold, ..
    } = cli.cmd else {
        panic!("wrong variant");
    };
    assert_eq!(user, "u");
    assert_eq!(threshold, Some(33));
}

#[test]
fn parse_personalize_minimal() {
    let cli = parse(&["personalize"]);
    let Cmd::Personalize { user, home } = cli.cmd else {
        panic!("wrong variant");
    };
    assert_eq!(user, "default");
    assert!(home.is_none());
}

#[test]
fn parse_auto_train_traces_dir_override() {
    let cli = parse(&[
        "auto-train", "--traces-dir", "/tmp/t",
    ]);
    let Cmd::AutoTrain { traces_dir, .. } = cli.cmd else {
        panic!("wrong variant");
    };
    assert_eq!(traces_dir.unwrap(), std::path::PathBuf::from("/tmp/t"));
}
