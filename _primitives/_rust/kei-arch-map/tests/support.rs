//! Thin wrappers around the library's evidence::* checkers, kept here so
//! `tests/evidence.rs` reads cleanly. Each wrapper accepts string-ish args
//! and forwards to the matching `kei_arch_map::evidence::*::check`.

use std::path::Path;

pub fn file_exists(path: &str, root: &Path) -> (bool, String) {
    kei_arch_map::evidence::file_exists::check(Path::new(path), root)
}

pub fn regex_match(file: &str, pattern: &str, root: &Path) -> (bool, String) {
    kei_arch_map::evidence::regex_match::check(Path::new(file), pattern, root)
}

pub fn grep_count(file: &str, pattern: &str, expected: u64, root: &Path) -> (bool, String) {
    kei_arch_map::evidence::grep_count::check(Path::new(file), pattern, expected, root)
}

pub fn file_size(path: &str, range: &[u64; 2], root: &Path) -> (bool, String) {
    kei_arch_map::evidence::file_size::check(Path::new(path), range, root)
}

pub fn json_field(file: &str, dotted: &str, expected: &str, root: &Path) -> (bool, String) {
    kei_arch_map::evidence::json_field::check(Path::new(file), dotted, expected, root)
}

pub fn cargo_check(manifest_dir: &str, root: &Path) -> (bool, String) {
    kei_arch_map::evidence::cargo_check::check(Path::new(manifest_dir), root)
}

pub fn cargo_check_safe(
    manifest_dir: &str,
    allowed: &[&str],
    root: &Path,
) -> (bool, String) {
    let allowed_bufs: Vec<std::path::PathBuf> =
        allowed.iter().map(std::path::PathBuf::from).collect();
    kei_arch_map::evidence::cargo_check_safe::check(
        Path::new(manifest_dir),
        &allowed_bufs,
        root,
    )
}

pub fn http_status(url: &str, expected: &[u16]) -> (bool, String) {
    kei_arch_map::evidence::http_status::check(url, expected)
}
