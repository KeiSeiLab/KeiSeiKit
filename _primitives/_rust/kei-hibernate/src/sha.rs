//! SHA-256 helper — stream file bytes without loading whole payload.
//! Used by exporter (manifest hashes) and importer (post-extract verify).

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const READ_BUF: usize = 64 * 1024;

/// Hex-encode a 32-byte digest. Lowercase, no delimiters.
pub fn hex(digest: &[u8]) -> String {
    let mut s = String::with_capacity(digest.len() * 2);
    for b in digest {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Hash bytes already in memory. Small helper for tests + manifest string.
pub fn hash_bytes(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex(&h.finalize())
}

/// Stream a file through Sha256 in 64KB chunks. Error surfaces raw io;
/// caller wraps with path context.
pub fn hash_file(path: &Path) -> std::io::Result<String> {
    let mut f = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; READ_BUF];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex(&hasher.finalize()))
}
