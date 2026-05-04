use super::path_resolve;
use std::path::Path;

pub fn check(path: &Path, range: &[u64; 2], root: &Path) -> (bool, String) {
    let resolved = match path_resolve::resolve_confined(path, root) {
        Ok(p) => p,
        Err(e) => return (false, e),
    };
    let meta = match std::fs::metadata(&resolved) {
        Ok(m) => m,
        Err(e) => return (false, format!("stat {} failed: {}", resolved.display(), e)),
    };
    let size = meta.len();
    let (lo, hi) = (range[0], range[1]);
    if lo > hi {
        return (false, format!("invalid range [{lo}, {hi}]: lo > hi"));
    }
    if size >= lo && size <= hi {
        (true, String::new())
    } else {
        (
            false,
            format!(
                "size({})={} not in [{}..={}]",
                resolved.display(),
                size,
                lo,
                hi
            ),
        )
    }
}
