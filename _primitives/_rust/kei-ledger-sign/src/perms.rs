use std::fs;
use std::io;
use std::path::Path;

/// Create a new file for writing with mode 0o600 atomically.
///
/// Uses `O_CREAT | O_EXCL` (`create_new`) + `O_WRONLY` + mode 0o600 in a
/// single `open(2)` syscall on Unix. This closes the TOCTOU window where a
/// two-step `write` + `chmod` sequence leaves the file world-readable for
/// an instant.
///
/// Fails if `path` already exists (caller must remove first for overwrite
/// semantics — see `save_keypair` for the atomic rename-into-place pattern).
#[cfg(unix)]
pub fn open_600_write(path: &Path) -> io::Result<fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

#[cfg(not(unix))]
pub fn open_600_write(path: &Path) -> io::Result<fs::File> {
    // Non-unix: mode bits are not applicable. Best effort — create_new
    // still prevents opening a pre-existing attacker-placed file.
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
}

/// Tighten permissions on an existing file to 0o600.
///
/// Retained for backward-compatibility and for callers that need to fix
/// permissions on a file they didn't create. New call sites should prefer
/// `open_600_write` which avoids the race entirely.
#[cfg(unix)]
pub fn chmod_600(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let perms = fs::Permissions::from_mode(0o600);
    fs::set_permissions(path, perms)
}

#[cfg(not(unix))]
pub fn chmod_600(_path: &Path) -> io::Result<()> {
    Ok(())
}
