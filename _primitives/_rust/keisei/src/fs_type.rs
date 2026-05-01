//! Filesystem type detection for brain root.
//!
//! Warns when the brain sits on exFAT / FAT32, where SQLite WAL shared-
//! memory mmap (used by `kei-memory`, `kei-artifact`, `kei-social-store`)
//! is unreliable and `keisei mount` (multi-client) will corrupt DBs.
//! Single-client `keisei attach` stays supported, hence the warning is
//! advisory, never blocking. Platform calls: `statfs(2)` on macOS +
//! Linux; Windows returns `Unknown` until `GetVolumeInformationW` lands.

use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum FsWarning {
    None,
    ExFat,
    Fat32,
    Unknown,
}

/// Print a stderr advisory when the brain root lives on exFAT / FAT32.
/// Advisory only — load succeeds regardless. See [`detect_fs_warning`].
pub fn warn_on_unsafe_fs(root: &Path) {
    match detect_fs_warning(root) {
        FsWarning::ExFat | FsWarning::Fat32 => {
            eprintln!(
                "[keisei] WARNING: brain root '{}' is on an exFAT/FAT32 filesystem. \
                 SQLite WAL mode (used by kei-memory/artifact/social-store) is UNSAFE \
                 on these filesystems. Use 'keisei attach' with ONE client at a time; \
                 'keisei mount' (multi-client fan-out) WILL corrupt memory DBs. \
                 See docs/USB-BRAIN-GUIDE.md for recommended filesystems.",
                root.display()
            );
        }
        _ => {}
    }
}

/// Classify the filesystem at `path`. NEVER returns `Result` — errors
/// collapse to `Unknown` so this stays call-safe inside `Brain::load`.
pub fn detect_fs_warning(path: &Path) -> FsWarning {
    #[cfg(target_os = "macos")]
    {
        return macos_detect(path);
    }
    #[cfg(target_os = "linux")]
    {
        return linux_detect(path);
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        let _ = path;
        FsWarning::Unknown
    }
}

#[cfg(target_os = "macos")]
fn macos_detect(path: &Path) -> FsWarning {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let c_path = match CString::new(path.as_os_str().as_bytes()) {
        Ok(s) => s,
        Err(_) => return FsWarning::Unknown,
    };
    // SAFETY: zero-init POD + valid CString ptr; statfs writes into buf.
    let mut buf: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(c_path.as_ptr(), &mut buf) } != 0 {
        return FsWarning::Unknown;
    }
    let name: String = buf
        .f_fstypename
        .iter()
        .take_while(|b| **b != 0)
        .map(|b| *b as u8 as char)
        .collect();
    match name.as_str() {
        "exfat" => FsWarning::ExFat,
        "msdos" => FsWarning::Fat32,
        _ => FsWarning::None,
    }
}

#[cfg(target_os = "linux")]
fn linux_detect(path: &Path) -> FsWarning {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    // man statfs(2) — magic numbers.
    const MSDOS_SUPER_MAGIC: i64 = 0x4d44;
    const EXFAT_SUPER_MAGIC: i64 = 0x2011_bab0;
    let c_path = match CString::new(path.as_os_str().as_bytes()) {
        Ok(s) => s,
        Err(_) => return FsWarning::Unknown,
    };
    // SAFETY: zero-init POD + valid CString ptr.
    let mut buf: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(c_path.as_ptr(), &mut buf) } != 0 {
        return FsWarning::Unknown;
    }
    match buf.f_type as i64 {
        EXFAT_SUPER_MAGIC => FsWarning::ExFat,
        MSDOS_SUPER_MAGIC => FsWarning::Fat32,
        _ => FsWarning::None,
    }
}
