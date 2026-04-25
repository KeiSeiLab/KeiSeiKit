//! Target-path resolution for atom scaffolding.
//!
//! Given the repo root and a `ForgeRequest`, compute the five absolute
//! paths the generator will write, and the five relative template paths
//! it will read from. Decouples path arithmetic from I/O so tests can
//! assert directly on layout.

use super::GenerateError;
use crate::form::ForgeRequest;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TargetPaths {
    pub md: PathBuf,
    pub input_schema: PathBuf,
    pub output_schema: PathBuf,
    pub rust_src: PathBuf,
    pub smoke_test: PathBuf,
}

impl TargetPaths {
    /// Build the five destination paths for `req` under `repo_root`.
    /// Returns `CrateNotFound` if `_primitives/_rust/<crate>/` is absent.
    pub fn resolve(
        repo_root: &Path,
        req: &ForgeRequest,
    ) -> Result<Self, GenerateError> {
        let crate_dir = repo_root
            .join("_primitives/_rust")
            .join(&req.crate_name);
        if !crate_dir.is_dir() {
            return Err(GenerateError::CrateNotFound(crate_dir));
        }
        let verb = &req.verb;
        let verb_snake = req.verb.replace('-', "_");
        Ok(Self {
            md: crate_dir.join("atoms").join(format!("{verb}.md")),
            input_schema: crate_dir
                .join("atoms/schemas")
                .join(format!("{verb}-input.json")),
            output_schema: crate_dir
                .join("atoms/schemas")
                .join(format!("{verb}-output.json")),
            rust_src: crate_dir
                .join("src/atoms")
                .join(format!("{verb_snake}.rs")),
            smoke_test: crate_dir
                .join("tests")
                .join(format!("{verb_snake}_smoke.rs")),
        })
    }

    /// Return `(template-rel-path, absolute-dest-path)` pairs in the same
    /// order new-atom.sh emitted, so any downstream tooling that depends
    /// on file-list ordering sees the same sequence.
    pub fn pairs(&self) -> [(&'static str, &Path); 5] {
        [
            ("atoms/__VERB__.md.template", &self.md),
            (
                "atoms/schemas/__VERB__-input.json.template",
                &self.input_schema,
            ),
            (
                "atoms/schemas/__VERB__-output.json.template",
                &self.output_schema,
            ),
            ("src/atoms/__VERB_SNAKE__.rs.template", &self.rust_src),
            (
                "tests/__VERB_SNAKE___smoke.rs.template",
                &self.smoke_test,
            ),
        ]
    }

    /// Refuse to overwrite: error on the first extant target.
    pub fn assert_none_exist(&self) -> Result<(), GenerateError> {
        for (_, dest) in self.pairs().iter() {
            if dest.exists() {
                return Err(GenerateError::FileExists(dest.to_path_buf()));
            }
        }
        Ok(())
    }

    /// Create `atoms/`, `atoms/schemas/`, `src/atoms/`, `tests/` under
    /// the crate dir. Idempotent.
    pub fn ensure_parent_dirs(&self) -> Result<(), GenerateError> {
        let dirs = [
            self.md.parent(),
            self.input_schema.parent(),
            self.output_schema.parent(),
            self.rust_src.parent(),
            self.smoke_test.parent(),
        ];
        for dir in dirs.into_iter().flatten() {
            fs::create_dir_all(dir)
                .map_err(|e| GenerateError::Io(e, dir.to_path_buf()))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req() -> ForgeRequest {
        ForgeRequest {
            crate_name: "kei-task".into(),
            verb: "add-dep".into(),
            kind: "command".into(),
            description: "x".into(),
        }
    }

    #[test]
    fn resolves_five_paths_under_crate() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("_primitives/_rust/kei-task")).unwrap();

        let t = TargetPaths::resolve(root, &req()).unwrap();
        assert!(t.md.ends_with("kei-task/atoms/add-dep.md"));
        assert!(t
            .input_schema
            .ends_with("kei-task/atoms/schemas/add-dep-input.json"));
        assert!(t
            .output_schema
            .ends_with("kei-task/atoms/schemas/add-dep-output.json"));
        assert!(t.rust_src.ends_with("kei-task/src/atoms/add_dep.rs"));
        assert!(t
            .smoke_test
            .ends_with("kei-task/tests/add_dep_smoke.rs"));
    }

    #[test]
    fn errors_when_crate_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let err = TargetPaths::resolve(tmp.path(), &req()).unwrap_err();
        assert!(matches!(err, GenerateError::CrateNotFound(_)));
    }

    #[test]
    fn assert_none_exist_trips_on_preexisting() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("_primitives/_rust/kei-task/atoms")).unwrap();
        fs::write(
            root.join("_primitives/_rust/kei-task/atoms/add-dep.md"),
            "x",
        )
        .unwrap();

        let t = TargetPaths::resolve(root, &req()).unwrap();
        let err = t.assert_none_exist().unwrap_err();
        assert!(matches!(err, GenerateError::FileExists(_)));
    }

    #[test]
    fn ensure_parent_dirs_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("_primitives/_rust/kei-task")).unwrap();

        let t = TargetPaths::resolve(root, &req()).unwrap();
        t.ensure_parent_dirs().unwrap();
        t.ensure_parent_dirs().unwrap(); // second call — no panic
        assert!(t.md.parent().unwrap().is_dir());
        assert!(t.input_schema.parent().unwrap().is_dir());
        assert!(t.rust_src.parent().unwrap().is_dir());
        assert!(t.smoke_test.parent().unwrap().is_dir());
    }
}
