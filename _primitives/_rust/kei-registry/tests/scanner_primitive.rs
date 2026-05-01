//! Primitive scanner walks `<kit>/_primitives/_rust/*/Cargo.toml` and
//! emits one Block per crate with `[package].name`.

use kei_registry::scanners::primitive::PrimitiveScanner;
use kei_registry::scanners::Scanner;
use kei_registry::BlockType;
use std::path::PathBuf;

fn fixture_kit_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("fake-kit")
}

#[test]
fn primitive_scanner_finds_one_crate() {
    let found = PrimitiveScanner.scan(&fixture_kit_root()).unwrap();
    assert_eq!(found.len(), 1, "fake-kit has one crate");
    let f = &found[0];
    assert_eq!(f.block_type, BlockType::Primitive);
    assert_eq!(f.name, "foo", "name comes from [package].name");
    assert!(f.path.ends_with("Cargo.toml"), "path is the Cargo.toml file");
}

#[test]
fn primitive_scanner_body_is_cargo_toml() {
    let found = PrimitiveScanner.scan(&fixture_kit_root()).unwrap();
    let body_text = std::str::from_utf8(&found[0].body).unwrap();
    assert!(body_text.contains("name = \"foo\""), "body is the actual TOML");
}

#[test]
fn primitive_scanner_extracts_caps_from_deps() {
    let found = PrimitiveScanner.scan(&fixture_kit_root()).unwrap();
    let caps = &found[0].caps;
    assert!(caps.contains("regex"), "regex dep maps to caps token");
}

#[test]
fn primitive_scanner_missing_root_returns_empty() {
    let bad = PathBuf::from("/this/path/does/not/exist");
    let found = PrimitiveScanner.scan(&bad).unwrap();
    assert!(found.is_empty());
}
