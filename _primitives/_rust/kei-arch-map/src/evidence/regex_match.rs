use super::path_resolve;
use regex::RegexBuilder;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const MAX_BYTES: u64 = 16 * 1024 * 1024;
const RE_SIZE_LIMIT: usize = 1_000_000;

/// Read up to MAX_BYTES into a string, returning a friendly error on failure.
pub(super) fn read_capped(path: &Path) -> Result<String, String> {
    let f = File::open(path).map_err(|e| format!("open {} failed: {}", path.display(), e))?;
    let mut buf = String::new();
    f.take(MAX_BYTES)
        .read_to_string(&mut buf)
        .map_err(|e| format!("read {} failed: {}", path.display(), e))?;
    Ok(buf)
}

/// Build regex with size-limit caps to prevent compilation DoS.
pub(super) fn build(pattern: &str) -> Result<regex::Regex, String> {
    RegexBuilder::new(pattern)
        .size_limit(RE_SIZE_LIMIT)
        .dfa_size_limit(RE_SIZE_LIMIT)
        .build()
        .map_err(|e| format!("invalid regex `{}`: {}", pattern, e))
}

pub fn check(file: &Path, pattern: &str, root: &Path) -> (bool, String) {
    let resolved = match path_resolve::resolve_confined(file, root) {
        Ok(p) => p,
        Err(e) => return (false, e),
    };
    let contents = match read_capped(&resolved) {
        Ok(s) => s,
        Err(e) => return (false, e),
    };
    let re = match build(pattern) {
        Ok(r) => r,
        Err(e) => return (false, e),
    };
    if re.is_match(&contents) {
        (true, String::new())
    } else {
        (false, format!("regex `{}` did not match in {}", pattern, resolved.display()))
    }
}
