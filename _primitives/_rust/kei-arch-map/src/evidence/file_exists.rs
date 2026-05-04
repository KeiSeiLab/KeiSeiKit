use super::path_resolve;
use std::path::Path;

pub fn check(path: &Path, root: &Path) -> (bool, String) {
    // Existence + root-containment: resolve_confined canonicalizes which
    // both verifies the entry exists AND that it stays inside `root` even
    // through symlinks (security fix D). Missing entries surface as a
    // friendly "not found" via the canonicalize error path.
    match path_resolve::resolve_confined(path, root) {
        Ok(_) => (true, String::new()),
        Err(e) => {
            if e.contains("canonicalize") {
                (false, format!("path not found: {}", path.display()))
            } else {
                (false, e)
            }
        }
    }
}
