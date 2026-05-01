//! Regenerate the 5 phase-5-migrated agent .md files in-place against
//! the live kit root (parent of `_assembler/`).
//!
//! Run with:
//!   cargo test -p agent-assembler --test regenerate_migrated -- --ignored
//!
//! Marked `#[ignore]` so the normal test suite does not write to the
//! committed tree — it only runs when an operator explicitly asks.

mod common;

use common::assemble_bin;
use std::path::PathBuf;
use std::process::Command;

fn kit_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
#[ignore]
fn regenerate_phase5_agents_in_place() {
    let root = kit_root();
    let manifests = [
        "code-implementer",
        "critic",
        "architect",
        "security-auditor",
        "validator",
    ];
    let args: Vec<String> = std::iter::once("--in-place".to_string())
        .chain(manifests.iter().map(|n| {
            root.join("_manifests")
                .join(format!("{n}.toml"))
                .to_string_lossy()
                .into_owned()
        }))
        .collect();

    let out = Command::new(assemble_bin())
        .env("AGENT_ROOT", &root)
        .env("HOME", &root)
        .args(&args)
        .output()
        .expect("spawn assemble");

    assert!(
        out.status.success(),
        "assemble failed:\n  stdout: {}\n  stderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // Every migrated agent's root-level .md must now exist and contain
    // the substrate section header.
    for name in &manifests {
        let md_path = root.join(format!("{name}.md"));
        let content = std::fs::read_to_string(&md_path)
            .unwrap_or_else(|e| panic!("read {}: {e}", md_path.display()));
        assert!(
            content.contains("# AGENT SUBSTRATE"),
            "{name}.md lacks substrate section after regeneration"
        );
    }
}
