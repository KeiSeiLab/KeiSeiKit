//! Tooling detection — ollama present, brew present, llama-server absent.

use kei_machine_probe::{detect_tooling, MockRunner};

#[test]
fn parses_versions_when_binaries_present() {
    let runner = MockRunner::from_dir(".")
        .with_ok("which_ollama", "/opt/homebrew/bin/ollama\n")
        .with_ok("ollama_--version", "ollama version is 0.3.12\n")
        .with_ok("which_brew", "/opt/homebrew/bin/brew\n")
        .with_ok("brew_--version", "Homebrew 4.3.20\n")
        .with_err("which_llama-server", "exit 1");

    let t = detect_tooling(&runner);
    assert_eq!(t.ollama.as_deref(), Some("0.3.12"));
    assert_eq!(t.homebrew.as_deref(), Some("4.3.20"));
    assert!(t.llama_cpp.is_none());
}

#[test]
fn parses_llama_server_version_too() {
    let runner = MockRunner::from_dir(".")
        .with_err("which_ollama", "no")
        .with_err("which_brew", "no")
        .with_ok("which_llama-server", "/usr/local/bin/llama-server\n")
        .with_ok(
            "llama-server_--version",
            "version: 4297 (b46d12f0)\nbuilt with Apple clang version 16.0.0\n",
        );

    let t = detect_tooling(&runner);
    assert!(t.ollama.is_none());
    assert!(t.homebrew.is_none());
    assert_eq!(t.llama_cpp.as_deref(), Some("4297"));
}
