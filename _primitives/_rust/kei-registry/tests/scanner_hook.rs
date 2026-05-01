//! Hook scanner walks `<hooks-root>/*.sh` (flat).

use kei_registry::scanners::hook::HookScanner;
use kei_registry::scanners::Scanner;
use kei_registry::BlockType;
use std::path::PathBuf;

fn fixture_hooks_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("fake-claude")
        .join("hooks")
}

#[test]
fn hook_scanner_finds_sh_files() {
    let found = HookScanner.scan(&fixture_hooks_root()).unwrap();
    assert_eq!(found.len(), 1, "one fixture .sh file");
    assert_eq!(found[0].block_type, BlockType::Hook);
    assert_eq!(found[0].name, "sample-hook");
    assert_eq!(found[0].caps, "shell");
}

#[test]
fn hook_scanner_path_ends_with_sh() {
    let found = HookScanner.scan(&fixture_hooks_root()).unwrap();
    assert!(found[0].path.ends_with(".sh"));
}
