//! Rollback accumulator for atom scaffolding writes.
//!
//! Keeps the list of successfully-written paths. On `finish()` the list
//! is returned (success). On `Drop` without `finish()` — i.e. an early
//! return from the caller due to an error — every recorded path is
//! deleted best-effort. Mirrors `trap rollback ERR` in new-atom.sh.
//!
//! Deletion is best-effort: we ignore `std::fs::remove_file` errors
//! because the caller already has a more-specific error to return.

use std::fs;
use std::path::PathBuf;

pub struct Rollback {
    written: Vec<PathBuf>,
    completed: bool,
}

impl Rollback {
    pub fn new() -> Self {
        Self { written: Vec::new(), completed: false }
    }

    /// Register a successful write so the rollback can undo it on drop.
    pub fn record(&mut self, path: PathBuf) {
        self.written.push(path);
    }

    /// Consume the rollback — mark complete and return the recorded
    /// paths. Must be called on the success path; otherwise `Drop`
    /// deletes everything.
    pub fn finish(mut self) -> Vec<PathBuf> {
        self.completed = true;
        std::mem::take(&mut self.written)
    }
}

impl Drop for Rollback {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        for path in &self.written {
            let _ = fs::remove_file(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finish_returns_paths_and_suppresses_rollback() {
        let tmp = tempfile::tempdir().unwrap();
        let a = tmp.path().join("a.txt");
        let b = tmp.path().join("b.txt");
        fs::write(&a, "x").unwrap();
        fs::write(&b, "y").unwrap();

        let mut r = Rollback::new();
        r.record(a.clone());
        r.record(b.clone());
        let files = r.finish();

        assert_eq!(files, vec![a.clone(), b.clone()]);
        assert!(a.exists(), "finish must NOT delete");
        assert!(b.exists());
    }

    #[test]
    fn drop_without_finish_deletes_recorded_files() {
        let tmp = tempfile::tempdir().unwrap();
        let a = tmp.path().join("a.txt");
        let b = tmp.path().join("b.txt");
        fs::write(&a, "x").unwrap();
        fs::write(&b, "y").unwrap();

        {
            let mut r = Rollback::new();
            r.record(a.clone());
            r.record(b.clone());
            // scope ends without finish — Drop fires
        }

        assert!(!a.exists(), "rollback must delete a");
        assert!(!b.exists(), "rollback must delete b");
    }

    #[test]
    fn drop_tolerates_missing_files() {
        let tmp = tempfile::tempdir().unwrap();
        let missing = tmp.path().join("never-existed.txt");
        {
            let mut r = Rollback::new();
            r.record(missing);
            // Drop must not panic on a missing file.
        }
    }
}
