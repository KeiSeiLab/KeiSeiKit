//! Clone a bundled Cubism sample rig into a per-user directory and swap the
//! primary face texture.
//!
//! Pure filesystem utility: no network, no image decoding. The Flux-generated
//! PNG bytes are written verbatim to `<target>/<textures>/texture_00.png`;
//! the hair/secondary texture (`texture_01.png` if present) is copied from
//! the base rig unchanged so the mesh UV layout continues to resolve.
//!
//! Install is atomic-ish: we stage into `<target>.tmp/` and then rename.
//! Same-device rename is a single inode flip (cannot be seen partially by
//! a reader); cross-device falls back to "remove old, rename staged". A
//! concurrent second writer is prevented by the per-user mutex the caller
//! holds (see `handlers/portrait.rs`); this module is TOCTOU-safe only
//! under that contract.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Upper bound on the directory depth we are willing to walk looking for
/// the primary texture. Prevents runaway recursion on a symlink loop or a
/// pathologically deep sample rig.
const LOCATE_DEPTH: usize = 4;

/// Errors from the clone/swap operation. `io::Error` is the common case;
/// `MissingTexture` fires when the base rig lacks a `texture_00.png` anywhere.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("base rig {0:?} is not a directory")]
    NotADir(PathBuf),
    #[error("base rig {0:?} has no texture_00.png under any subdir")]
    MissingTexture(PathBuf),
}

/// Clone `base_dir` to `target_dir`, then overwrite the primary face texture
/// at `<texture_dir>/texture_00.png` with `new_texture_png`.
///
/// Historical API kept for callers that want the non-atomic version; new
/// callers should prefer `install_rig`.
pub fn clone_and_swap(
    base_dir: &Path,
    target_dir: &Path,
    new_texture_png: &[u8],
) -> Result<(), Error> {
    if !base_dir.is_dir() {
        return Err(Error::NotADir(base_dir.to_path_buf()));
    }
    if target_dir.exists() {
        fs::remove_dir_all(target_dir)?;
    }
    fs::create_dir_all(target_dir)?;
    copy_tree(base_dir, target_dir)?;
    let texture_path = locate_texture(target_dir)?;
    fs::write(&texture_path, new_texture_png)?;
    Ok(())
}

/// Atomic install: stage into `<target>.tmp`, then rename onto the final
/// path. Eliminates the TOCTOU window between `remove_dir_all` and `copy`
/// that `clone_and_swap` exposes.
///
/// Caller MUST hold a per-`user_id` mutex so two concurrent installs to the
/// same target do not race on staging either.
pub fn install_rig(
    base_dir: &Path,
    target_dir: &Path,
    new_texture_png: &[u8],
) -> Result<(), Error> {
    if !base_dir.is_dir() {
        return Err(Error::NotADir(base_dir.to_path_buf()));
    }
    let staging = staging_path(target_dir);
    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;
    copy_tree(base_dir, &staging)?;
    let texture_path = locate_texture(&staging)?;
    fs::write(&texture_path, new_texture_png)?;
    finalize(&staging, target_dir)?;
    Ok(())
}

/// Rename `staging` onto `target`, deleting `target` first if it exists.
/// Same-device rename is atomic; cross-device rename falls back to
/// "remove old → rename staged" with a brief window where `target`
/// does not exist. Per-user mutex makes that window invisible.
fn finalize(staging: &Path, target: &Path) -> Result<(), Error> {
    if target.exists() {
        fs::remove_dir_all(target)?;
    }
    match fs::rename(staging, target) {
        Ok(()) => Ok(()),
        Err(e) if is_cross_device(&e) => {
            copy_tree(staging, target)?;
            fs::remove_dir_all(staging)?;
            Ok(())
        }
        Err(e) => Err(Error::Io(e)),
    }
}

/// Detect EXDEV ("cross-device link") on Linux/macOS. `ErrorKind::CrossesDevices`
/// is still nightly-gated on stable Rust, so we compare raw errno. 18 is
/// EXDEV on both Linux and Darwin — the only platforms this crate targets.
fn is_cross_device(e: &io::Error) -> bool {
    e.raw_os_error() == Some(18)
}

/// Derive the staging path for an atomic install. Kept as a sibling of the
/// target so the rename is same-device whenever possible.
fn staging_path(target_dir: &Path) -> PathBuf {
    let mut s = target_dir.as_os_str().to_os_string();
    s.push(".tmp");
    PathBuf::from(s)
}

/// Recursively copy every entry from `src` into `dst`. Regular files are
/// `fs::copy`'d; directories are re-created and recursed into. Symlinks are
/// NOT followed — we want plain file copies so deletes on the cloned side
/// cannot affect the base sample.
fn copy_tree(src: &Path, dst: &Path) -> Result<(), Error> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ftype = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ftype.is_dir() {
            fs::create_dir_all(&to)?;
            copy_tree(&from, &to)?;
        } else if ftype.is_file() {
            fs::copy(&from, &to)?;
        }
        // symlinks, sockets, etc. are skipped silently — not expected in
        // a Cubism sample rig.
    }
    Ok(())
}

/// Find the first `texture_00.png` anywhere under `root`. Walks via
/// `WalkDir` capped at `LOCATE_DEPTH`, symlinks NOT followed — prevents
/// infinite loops from a pathological base rig.
fn locate_texture(root: &Path) -> Result<PathBuf, Error> {
    for entry in WalkDir::new(root)
        .max_depth(LOCATE_DEPTH)
        .follow_links(false)
        .into_iter()
        .flatten()
    {
        if entry.file_type().is_file() && entry.file_name() == "texture_00.png" {
            return Ok(entry.into_path());
        }
    }
    Err(Error::MissingTexture(root.to_path_buf()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn make_base_rig(root: &Path) {
        let tex_dir = root.join("rig.2048");
        fs::create_dir_all(&tex_dir).unwrap();
        fs::write(tex_dir.join("texture_00.png"), b"OLD00").unwrap();
        fs::write(tex_dir.join("texture_01.png"), b"HAIR01").unwrap();
        fs::write(root.join("rig.moc3"), b"MOC").unwrap();
        fs::write(root.join("rig.model3.json"), b"{}").unwrap();
    }

    #[test]
    fn clone_copies_all_files_and_swaps_texture_00() {
        let tmp = tempdir().unwrap();
        let base = tmp.path().join("base");
        let target = tmp.path().join("custom");
        make_base_rig(&base);
        clone_and_swap(&base, &target, b"NEWFACE").unwrap();
        let tex = target.join("rig.2048").join("texture_00.png");
        assert_eq!(fs::read(&tex).unwrap(), b"NEWFACE");
        let hair = target.join("rig.2048").join("texture_01.png");
        assert_eq!(fs::read(&hair).unwrap(), b"HAIR01");
        assert!(target.join("rig.moc3").is_file());
        assert!(target.join("rig.model3.json").is_file());
    }

    #[test]
    fn clone_is_idempotent_overwrites_existing_target() {
        let tmp = tempdir().unwrap();
        let base = tmp.path().join("base");
        let target = tmp.path().join("custom");
        make_base_rig(&base);
        clone_and_swap(&base, &target, b"FIRST").unwrap();
        clone_and_swap(&base, &target, b"SECOND").unwrap();
        let tex = target.join("rig.2048").join("texture_00.png");
        assert_eq!(fs::read(&tex).unwrap(), b"SECOND");
    }

    #[test]
    fn missing_base_texture_errors() {
        let tmp = tempdir().unwrap();
        let base = tmp.path().join("base");
        let target = tmp.path().join("custom");
        fs::create_dir_all(&base).unwrap();
        fs::write(base.join("rig.moc3"), b"MOC").unwrap();
        let err = clone_and_swap(&base, &target, b"X").unwrap_err();
        assert!(matches!(err, Error::MissingTexture(_)));
    }

    #[test]
    fn install_rig_creates_target_atomically() {
        let tmp = tempdir().unwrap();
        let base = tmp.path().join("base");
        let target = tmp.path().join("custom");
        make_base_rig(&base);
        install_rig(&base, &target, b"ATOMIC").unwrap();
        assert!(target.is_dir());
        let tex = target.join("rig.2048").join("texture_00.png");
        assert_eq!(fs::read(&tex).unwrap(), b"ATOMIC");
        // Staging must be cleaned up.
        assert!(!staging_path(&target).exists());
    }

    #[test]
    fn install_rig_overwrites_existing_target() {
        let tmp = tempdir().unwrap();
        let base = tmp.path().join("base");
        let target = tmp.path().join("custom");
        make_base_rig(&base);
        install_rig(&base, &target, b"FIRST").unwrap();
        install_rig(&base, &target, b"SECOND").unwrap();
        let tex = target.join("rig.2048").join("texture_00.png");
        assert_eq!(fs::read(&tex).unwrap(), b"SECOND");
    }

    #[test]
    fn install_rig_cleans_up_stale_staging() {
        let tmp = tempdir().unwrap();
        let base = tmp.path().join("base");
        let target = tmp.path().join("custom");
        make_base_rig(&base);
        // Simulate a prior crashed run leaving a staging dir behind.
        let staging = staging_path(&target);
        fs::create_dir_all(&staging).unwrap();
        fs::write(staging.join("trash"), b"LEFTOVER").unwrap();
        install_rig(&base, &target, b"RECOVERED").unwrap();
        assert!(!staging.exists());
        let tex = target.join("rig.2048").join("texture_00.png");
        assert_eq!(fs::read(&tex).unwrap(), b"RECOVERED");
    }

    #[test]
    fn locate_texture_walks_nested_subdirs() {
        let tmp = tempdir().unwrap();
        let deep = tmp.path().join("a").join("b").join("c");
        fs::create_dir_all(&deep).unwrap();
        fs::write(deep.join("texture_00.png"), b"DEEP").unwrap();
        let found = locate_texture(tmp.path()).unwrap();
        assert_eq!(fs::read(&found).unwrap(), b"DEEP");
    }
}
