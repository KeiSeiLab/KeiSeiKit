//! Atom emit + parseable-frontmatter roundtrip.

use kei_skill_importer::{emit, import, SourceFormat};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn render_atom_starts_with_yaml_delimiter() {
    let skill = import(&fixture("cline-typescript-paths.md"), SourceFormat::Cline)
        .expect("parse");
    let rendered = emit::as_atom::render(&skill).expect("render");
    assert!(rendered.starts_with("---\n"),
        "atom must start with YAML frontmatter delimiter; got first 40 chars: {}",
        &rendered[..rendered.len().min(40)]);
}

#[test]
fn render_atom_includes_provenance_comment() {
    let skill = import(&fixture("cline-typescript-paths.md"), SourceFormat::Cline)
        .expect("parse");
    let rendered = emit::as_atom::render(&skill).expect("render");
    assert!(rendered.contains("<!-- imported from"),
        "expect provenance comment");
    assert!(rendered.contains("format=cline"),
        "expect format=cline in provenance");
}

#[test]
fn render_atom_frontmatter_parses_back_as_yaml() {
    let skill = import(&fixture("cline-typescript-paths.md"), SourceFormat::Cline)
        .expect("parse");
    let rendered = emit::as_atom::render(&skill).expect("render");
    // Extract frontmatter and parse with serde_yaml_ng — must succeed
    let body = rendered.strip_prefix("---\n").expect("starts ---");
    let end = body.find("\n---").expect("close ---");
    let fm = &body[..end];
    let val: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(fm).expect("frontmatter parses as YAML");
    assert_eq!(
        val.get("kind").and_then(|v| v.as_str()),
        Some("transform")
    );
    assert!(val.get("atom").and_then(|v| v.as_str()).unwrap_or("")
        .starts_with("kei-imported::"));
}

#[test]
fn write_atom_creates_atoms_subdir_file() {
    let skill = import(&fixture("cline-typescript-paths.md"), SourceFormat::Cline)
        .expect("parse");
    let tmp = tempfile::tempdir().unwrap();
    let path = emit::as_atom::write(&skill, tmp.path()).expect("write");
    assert!(path.exists(), "file must exist: {}", path.display());
    let parent_name = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap();
    assert_eq!(parent_name, "atoms");
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("# kei-imported::"));
}
