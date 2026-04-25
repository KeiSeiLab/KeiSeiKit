//! Path-traversal-safe base+rel join.
//!
//! `safe_join` is the authoritative base+rel path-join: rejects absolute
//! components and `..`, canonicalises, asserts base containment (including
//! post-canonicalise symlink escapes).

use crate::error::Error;
use std::path::{Component, Path, PathBuf};

/// Safe base+rel path join.
///
/// Rejects absolute paths, parent (`..`) components, non-existent bases,
/// and post-canonicalise escapes from `base` (including symlink escapes).
///
/// Contract:
/// - `base` MUST canonicalize (i.e. must exist as a real directory). A
///   non-existent base means the caller is not in a well-defined sandbox
///   and we refuse to construct a join.
/// - If `joined` canonicalizes, its real path MUST start with `base_canon`.
/// - If `joined` does not exist, we canonicalize `joined.parent()` and
///   require that to start with `base_canon`. This catches symlinked
///   parent directories that redirect outside the sandbox.
/// - If neither `joined` nor `joined.parent()` exist, no symlink can
///   possibly live there — the lexical (absolute + parent-free) check
///   already completed is sufficient.
pub fn safe_join(base: &Path, rel: &str) -> Result<PathBuf, Error> {
    let rel_path = reject_bad_rel(rel)?;
    let joined = base.join(rel_path);
    let base_canon = canonicalize_base(base)?;
    assert_joined_inside_base(&joined, &base_canon, rel)?;
    Ok(joined)
}

fn reject_bad_rel(rel: &str) -> Result<&Path, Error> {
    let rel_path = Path::new(rel);
    if rel_path.is_absolute() {
        return Err(Error::PathAbsolute(rel.to_string()));
    }
    for comp in rel_path.components() {
        if matches!(comp, Component::ParentDir) {
            return Err(Error::PathParent(rel.to_string()));
        }
    }
    Ok(rel_path)
}

fn canonicalize_base(base: &Path) -> Result<PathBuf, Error> {
    base.canonicalize().map_err(|source| Error::Canonicalize {
        path: base.to_path_buf(),
        source,
    })
}

fn assert_joined_inside_base(
    joined: &Path,
    base_canon: &Path,
    rel: &str,
) -> Result<(), Error> {
    if let Ok(jc) = joined.canonicalize() {
        return check_contained(&jc, base_canon, rel);
    }
    let Some(parent) = joined.parent() else {
        return Ok(());
    };
    let Ok(pc) = parent.canonicalize() else {
        // Grand-parent also doesn't exist — no symlink can live here.
        return Ok(());
    };
    check_contained(&pc, base_canon, rel)
}

fn check_contained(candidate: &Path, base_canon: &Path, rel: &str) -> Result<(), Error> {
    if candidate.starts_with(base_canon) {
        Ok(())
    } else {
        Err(Error::PathEscape {
            base: base_canon.to_path_buf(),
            rel: rel.to_string(),
        })
    }
}
