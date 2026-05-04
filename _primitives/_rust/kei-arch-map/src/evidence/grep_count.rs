use super::{path_resolve, regex_match};
use std::path::Path;

pub fn check(file: &Path, pattern: &str, expected: u64, root: &Path) -> (bool, String) {
    let resolved = match path_resolve::resolve_confined(file, root) {
        Ok(p) => p,
        Err(e) => return (false, e),
    };
    let contents = match regex_match::read_capped(&resolved) {
        Ok(s) => s,
        Err(e) => return (false, e),
    };
    let re = match regex_match::build(pattern) {
        Ok(r) => r,
        Err(e) => return (false, e),
    };
    let actual = contents.lines().filter(|l| re.is_match(l)).count() as u64;
    if actual == expected {
        (true, String::new())
    } else {
        (
            false,
            format!(
                "grep_count `{}` in {}: actual={} expected={}",
                pattern,
                resolved.display(),
                actual,
                expected
            ),
        )
    }
}
