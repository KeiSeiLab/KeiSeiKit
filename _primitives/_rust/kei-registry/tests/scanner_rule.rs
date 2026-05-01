//! Rule scanner walks `<rules-root>/*.md` (flat).

use kei_registry::scanners::rule::RuleScanner;
use kei_registry::scanners::Scanner;
use kei_registry::BlockType;
use std::path::PathBuf;

fn fixture_rules_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("fake-claude")
        .join("rules")
}

#[test]
fn rule_scanner_finds_md_files() {
    let found = RuleScanner.scan(&fixture_rules_root()).unwrap();
    assert_eq!(found.len(), 1, "one fixture .md file");
    assert_eq!(found[0].block_type, BlockType::Rule);
    assert_eq!(found[0].name, "sample-rule", "name is filename stem");
}

#[test]
fn rule_scanner_returns_empty_on_missing_dir() {
    let found = RuleScanner.scan(&PathBuf::from("/no/such/dir")).unwrap();
    assert!(found.is_empty());
}
