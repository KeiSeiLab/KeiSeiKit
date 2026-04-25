//! Discover walks from `cwd` upward, returning every CLAUDE.md / AGENTS.md
//! it finds in nearest-first order.
//!
//! Setup: tempdir -> a/b/c/d/e (5 levels). Plant CLAUDE.md at e (innermost),
//! at c (mid), at a (outermost). Run discover from e. Expect three hits in
//! the order [e, c, a].
//!
//! Symlink-safety: also plant a symlink CLAUDE.md and assert it's skipped.

use crate::context::discover;
use crate::context::types::ContextKind;
use std::fs;

fn fixture() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("tempdir");
    let root = td.path();
    let a = root.join("a");
    let b = a.join("b");
    let c = b.join("c");
    let d = c.join("d");
    let e = d.join("e");
    fs::create_dir_all(&e).unwrap();
    fs::write(a.join("CLAUDE.md"), "outer").unwrap();
    fs::write(c.join("CLAUDE.md"), "middle").unwrap();
    fs::write(e.join("CLAUDE.md"), "inner").unwrap();
    td
}

#[test]
fn returns_nearest_first_three_levels() {
    let td = fixture();
    let start = td.path().join("a/b/c/d/e");
    let hits = discover(&start);
    let claude: Vec<_> = hits
        .iter()
        .filter(|f| f.kind == ContextKind::ClaudeMd)
        .collect();
    assert!(claude.len() >= 3, "got {} CLAUDE.md hits, expected >=3", claude.len());
    assert_eq!(claude[0].content, "inner");
    assert_eq!(claude[1].content, "middle");
    assert_eq!(claude[2].content, "outer");
}

#[test]
fn stops_at_filesystem_root() {
    // Walk from `/`. Should not panic, returns empty (no CLAUDE.md at /).
    let hits = discover(std::path::Path::new("/"));
    let _ = hits.len();
}

#[cfg(unix)]
#[test]
fn skips_symlinked_context_files() {
    let td = tempfile::tempdir().unwrap();
    let real = td.path().join("real.md");
    let link = td.path().join("CLAUDE.md");
    fs::write(&real, "real-body").unwrap();
    std::os::unix::fs::symlink(&real, &link).unwrap();
    let hits = discover(td.path());
    let claude: Vec<_> = hits
        .iter()
        .filter(|f| f.kind == ContextKind::ClaudeMd)
        .collect();
    assert!(
        claude.is_empty(),
        "symlinked CLAUDE.md should be skipped, got {}",
        claude.len()
    );
}
