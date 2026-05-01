use std::path::PathBuf;
use std::sync::Once;

use kei_gdrive_import::{classify, Verdict};

fn fixture(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("fixtures");
    p.push(name);
    p
}

// `.git/` cannot be tracked inside a parent git repo. The already-repo
// fixture's `.git/HEAD` is materialised at test-runtime instead.
static ENSURE_ALREADY_REPO_FIXTURE: Once = Once::new();
fn ensure_already_repo_fixture() {
    ENSURE_ALREADY_REPO_FIXTURE.call_once(|| {
        let dot_git = fixture("already-repo").join(".git");
        std::fs::create_dir_all(&dot_git).expect("create .git/");
        std::fs::write(dot_git.join("HEAD"), "ref: refs/heads/main\n")
            .expect("write .git/HEAD");
    });
}

#[test]
fn rust_project_is_project() {
    let c = classify(&fixture("rust-project"));
    assert_eq!(c.verdict, Verdict::Project);
    assert_eq!(c.primary_lang, "rust");
    assert!(c.score >= 15, "score was {}", c.score);
    assert!(c.markers.iter().any(|m| m.file == "Cargo.toml"));
}

#[test]
fn node_project_is_project() {
    let c = classify(&fixture("node-project"));
    assert_eq!(c.verdict, Verdict::Project);
    assert_eq!(c.primary_lang, "node");
}

#[test]
fn python_project_is_project() {
    let c = classify(&fixture("python-project"));
    assert_eq!(c.verdict, Verdict::Project);
    assert_eq!(c.primary_lang, "python");
}

#[test]
fn photos_folder_is_not_a_project() {
    let c = classify(&fixture("photos-folder"));
    assert_eq!(c.verdict, Verdict::NotAProject);
    assert_eq!(c.score, 0);
}

#[test]
fn already_repo_short_circuits_regardless_of_score() {
    ensure_already_repo_fixture();
    let c = classify(&fixture("already-repo"));
    assert_eq!(c.verdict, Verdict::AlreadyRepo);
    // Cargo.toml still contributes to score; AlreadyRepo just overrides verdict.
    assert!(c.markers.iter().any(|m| m.file == ".git"));
}

#[test]
fn mixed_ambiguous_falls_in_middle_band() {
    let c = classify(&fixture("mixed-ambiguous"));
    assert_eq!(c.verdict, Verdict::Ambiguous);
    assert!(c.score >= 5 && c.score <= 7, "score was {}", c.score);
}
