//! SQLite-file presence scanner.
//!
//! Constructor Pattern: one cube = "how many `*.sqlite` files live at
//! depth ≤ 2 under this project?". The dashboard uses this to decide
//! whether to expose a project to Datasette. Depth is bounded so we
//! never run away into vendored dependencies.

use std::path::Path;
use walkdir::WalkDir;

/// Returns true if `path` ends in `.sqlite` (case-insensitive). Files
/// ending in `.sqlite-journal`, `.sqlite-shm`, or `.sqlite-wal` are NOT
/// counted — they're transient artefacts of an open connection.
fn is_sqlite_file(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => ext.eq_ignore_ascii_case("sqlite"),
        None => false,
    }
}

/// Count `*.sqlite` files under `project_root` to depth ≤ 2.
/// Depth 0 = the project root itself, depth 1 = `<root>/foo.sqlite`,
/// depth 2 = `<root>/data/foo.sqlite`. Symlinks are not followed.
pub fn count_sqlite_files(project_root: &Path) -> usize {
    if !project_root.exists() {
        return 0;
    }
    WalkDir::new(project_root)
        .max_depth(2)
        .follow_links(false)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file() && is_sqlite_file(e.path()))
        .count()
}
