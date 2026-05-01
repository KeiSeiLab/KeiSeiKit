//! SHA-256 helpers. Used for WYSIWYD invariant checks
//! (source file must not mutate between lock and verify).

use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub fn hash_file(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn hashes_empty_file() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(b"").unwrap();
        let h = hash_file(f.path()).unwrap();
        // sha256("") — well-known constant
        assert_eq!(
            h,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn hashes_hello_world() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(b"hello world").unwrap();
        let h = hash_file(f.path()).unwrap();
        assert_eq!(
            h,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn returns_err_on_missing_file() {
        let result = hash_file(Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
    }
}
