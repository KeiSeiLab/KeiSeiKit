//! Symlink-safe atomic write — closes the TOCTOU + symlink-escape window
//! that the simpler `tool::write::atomic_write` leaves open for the trusted
//! `/tool/apply` endpoint (F-CRIT-4).
//!
//! Algorithm:
//!   1. `openat(parent_fd, "<basename>.<ts>.tmp", O_CREAT|O_EXCL|O_WRONLY|
//!      O_NOFOLLOW)` — creates a fresh temp inode in the parent dir, refusing
//!      to follow a symlink even at the leaf.
//!   2. `write_all` + `fsync`.
//!   3. `renameat(parent_fd, tmp, parent_fd, basename)` — atomic same-dir
//!      rename; never follows a symlink at the destination because
//!      `renameat` operates on the directory entry, not on a target.
//!   4. After rename, re-canonicalise the destination AND verify it still
//!      lives under `project_root_canon`. Any mismatch (a parent dir was
//!      symlink-swapped between resolve and write) → unlink the just-created
//!      file and return `Forbidden`.
//!
//! Residual race window: an attacker who can write to the parent
//! directory's parent (and thus swap a parent dir for a symlink, then swap
//! it back between rename and canonicalize) can in principle still escape.
//! This is acknowledged in `tool_apply_INTEGRATION.md` — the endpoint is
//! TRUSTED by bearer-auth and the on-disk write is sequenced behind the
//! parent-fd, dramatically narrowing the window vs the prior `tokio::fs::
//! write` + path-only check. A future Wave can close it with `openat2`
//! `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS` (Linux ≥5.6).
//!
//! Local to `tool_apply.rs` for now per Wave 44b INTEGRATION note —
//! orchestrator may relocate to `tool/atomic_io.rs::atomic_write_nofollow`
//! after the wave44a `atomic_write` extraction lands.

use crate::error::AppError;
use tracing::debug;
use nix::fcntl::{openat, renameat, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{fsync, unlinkat, UnlinkatFlags};
use std::ffi::CString;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// Atomic, symlink-refusing write of `content` to `dest`. Caller has already
/// verified `dest`'s deepest-existing ancestor is under `project_root_canon`.
pub(super) async fn atomic_write_nofollow(
    dest: &Path,
    content: &[u8],
    project_root_canon: &Path,
) -> Result<(), AppError> {
    let parent = parent_dir(dest);
    let basename = basename_cstring(dest)?;
    let tmp_name = make_tmp_name(&basename);
    let dest_owned = dest.to_path_buf();
    let root_owned = project_root_canon.to_path_buf();
    let payload = content.to_vec();
    tokio::task::spawn_blocking(move || {
        write_blocking(&parent, &basename, &tmp_name, &payload, &dest_owned, &root_owned)
    })
    .await
    .map_err(|e| AppError::Internal(format!("join: {e}")))?
}

/// All blocking syscalls run inside a single `spawn_blocking` so the runtime
/// only sees one task per write. Returns `AppError` ready to propagate.
fn write_blocking(
    parent: &Path,
    basename: &CString,
    tmp_name: &CString,
    content: &[u8],
    dest: &Path,
    project_root_canon: &Path,
) -> Result<(), AppError> {
    let parent_fd = open_dir(parent)?;
    let dirfd = parent_fd.as_raw_fd();
    let tmp_fd = create_tmp(dirfd, tmp_name)?;
    write_then_fsync(tmp_fd, content)?;
    if let Err(e) = rename_into_place(dirfd, tmp_name, basename) {
        unlink_silent(dirfd, tmp_name);
        return Err(e);
    }
    verify_or_unlink(dirfd, basename, dest, project_root_canon)
}

/// Open the parent directory; the returned `OwnedFd` keeps it open for the
/// duration of all `*at` calls below.
fn open_dir(parent: &Path) -> Result<OwnedFd, AppError> {
    std::fs::File::open(parent)
        .map(OwnedFd::from)
        .map_err(|e| AppError::Internal(format!("open parent {}: {e}", parent.display())))
}

/// Create the staging tempfile with `O_NOFOLLOW | O_EXCL` — refuses any
/// pre-existing symlink at this name.
fn create_tmp(dirfd: RawFd, tmp_name: &CString) -> Result<OwnedFd, AppError> {
    let flags = OFlag::O_CREAT | OFlag::O_EXCL | OFlag::O_WRONLY | OFlag::O_NOFOLLOW;
    let mode = Mode::from_bits_truncate(0o600);
    let raw = openat(Some(dirfd), tmp_name.as_c_str(), flags, mode)
        .map_err(|e| AppError::Internal(format!("openat tmp: {e}")))?;
    Ok(unsafe { OwnedFd::from_raw_fd(raw) })
}

/// Write payload bytes and `fsync(2)` the file before rename.
fn write_then_fsync(tmp_fd: OwnedFd, content: &[u8]) -> Result<(), AppError> {
    use std::io::Write;
    let mut file = std::fs::File::from(tmp_fd);
    file.write_all(content)
        .map_err(|e| AppError::Internal(format!("write tmp: {e}")))?;
    fsync(file.as_raw_fd())
        .map_err(|e| AppError::Internal(format!("fsync tmp: {e}")))
}

/// `renameat` from staging name to final basename, both relative to
/// `parent_fd`. Atomic on POSIX.
fn rename_into_place(dirfd: RawFd, from: &CString, to: &CString) -> Result<(), AppError> {
    renameat(Some(dirfd), from.as_c_str(), Some(dirfd), to.as_c_str())
        .map_err(|e| AppError::Internal(format!("renameat: {e}")))
}

/// After rename, canonicalise the destination and confirm it still lives
/// under `project_root_canon`. If not, unlink and return `Forbidden`.
fn verify_or_unlink(
    dirfd: RawFd,
    basename: &CString,
    dest: &Path,
    project_root_canon: &Path,
) -> Result<(), AppError> {
    let canon = match dest.canonicalize() {
        Ok(c) => c,
        Err(e) => {
            unlink_silent(dirfd, basename);
            return Err(AppError::Internal(format!("post-write canonicalize: {e}")));
        }
    };
    if !canon.starts_with(project_root_canon) {
        unlink_silent(dirfd, basename);
        debug!(
            "tool_apply: symlink escape detected dest={} canon={} root={}",
            dest.display(), canon.display(), project_root_canon.display()
        );
        return Err(AppError::Forbidden);
    }
    Ok(())
}

/// Unlink `name` under `dirfd`, swallowing errors — used in cleanup paths
/// where we already have a primary error to surface.
fn unlink_silent(dirfd: RawFd, name: &CString) {
    let _ = unlinkat(Some(dirfd), name.as_c_str(), UnlinkatFlags::NoRemoveDir);
}

/// Resolve `dest`'s parent directory, defaulting to `.` for bare names.
fn parent_dir(dest: &Path) -> PathBuf {
    dest.parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Convert `dest`'s file name to a `CString` (refusing names with NUL).
fn basename_cstring(dest: &Path) -> Result<CString, AppError> {
    let name = dest.file_name()
        .ok_or_else(|| AppError::BadRequest("path has no filename".into()))?;
    CString::new(name.as_bytes())
        .map_err(|_| AppError::BadRequest("filename contains NUL".into()))
}

/// Build a unique temp basename `<dest>.<nanos>.tmp`.
fn make_tmp_name(basename: &CString) -> CString {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut bytes = basename.as_bytes().to_vec();
    bytes.extend_from_slice(format!(".{ts}.tmp").as_bytes());
    CString::new(bytes).expect("temp name has no NUL")
}
