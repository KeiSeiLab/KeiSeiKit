//! `rescue(agent_id, kit_root, out_dir)` — copy a fork's files out of
//! band.
//!
//! Resolution order:
//!   1. `_forks/<id>/` (live) → copy to `out_dir`
//!   2. `_archive/forks/<date>/<id>/` (archived) → copy to `out_dir`
//!   3. Neither → `Error::Gone`
//!
//! Copy is recursive; the destination may pre-exist (we merge on top).
//! Returns the number of regular files copied.

use crate::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub fn rescue(agent_id: &str, kit_root: &Path, out_dir: &Path) -> Result<usize, Error> {
    let src = locate(agent_id, kit_root).ok_or_else(|| Error::Gone(agent_id.to_string()))?;
    fs::create_dir_all(out_dir)?;
    Ok(copy_tree(&src, out_dir)?)
}

fn locate(agent_id: &str, kit_root: &Path) -> Option<PathBuf> {
    let live = kit_root.join("_forks").join(agent_id);
    if live.is_dir() {
        return Some(live);
    }
    let archive_root = kit_root.join("_archive/forks");
    let dates = fs::read_dir(&archive_root).ok()?;
    for e in dates.flatten() {
        let candidate = e.path().join(agent_id);
        if candidate.is_dir() {
            return Some(candidate);
        }
    }
    None
}

fn copy_tree(src: &Path, dst: &Path) -> std::io::Result<usize> {
    let mut n = 0;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        let from = entry.path();
        let to = dst.join(&name);
        if from.is_dir() {
            fs::create_dir_all(&to)?;
            n += copy_tree(&from, &to)?;
        } else if from.is_file() {
            fs::copy(&from, &to)?;
            n += 1;
        }
    }
    Ok(n)
}
