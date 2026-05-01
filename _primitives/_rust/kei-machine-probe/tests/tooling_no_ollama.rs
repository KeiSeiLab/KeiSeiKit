//! Tooling detection — every binary missing.

use kei_machine_probe::{detect_tooling, MockRunner};

#[test]
fn all_missing_yields_all_none() {
    let runner = MockRunner::from_dir(".")
        .with_err("which_ollama", "not installed")
        .with_err("which_brew", "not installed")
        .with_err("which_llama-server", "not installed");

    let t = detect_tooling(&runner);
    assert!(t.ollama.is_none());
    assert!(t.homebrew.is_none());
    assert!(t.llama_cpp.is_none());
}

#[test]
fn empty_which_output_treated_as_absent() {
    // `which` sometimes returns empty stdout with exit 0 on some shells.
    let runner = MockRunner::from_dir(".")
        .with_ok("which_ollama", "\n")
        .with_err("which_brew", "no")
        .with_err("which_llama-server", "no");

    let t = detect_tooling(&runner);
    assert!(t.ollama.is_none());
}
