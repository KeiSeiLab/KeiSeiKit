//! Token lifecycle: generate / save (chmod 600) / load / validate.
//!
//! The bearer token is a 32-byte random value rendered as 64 lowercase hex
//! characters. It is stored at `~/.keisei/cortex.token` with file mode
//! 0600 on unix. Reads trim trailing whitespace so a caller-added newline
//! does not corrupt comparisons.

use rand::RngCore;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Length of the raw token in bytes (32 → 64 hex chars).
pub const TOKEN_BYTES: usize = 32;

/// Length of the hex-rendered token (always `2 * TOKEN_BYTES`).
pub const TOKEN_HEX_LEN: usize = TOKEN_BYTES * 2;

/// Errors surfaced by this module.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("token file I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("token length invalid: expected {TOKEN_HEX_LEN} hex chars, got {0}")]
    BadLength(usize),

    #[error("token contained non-hex byte at index {0}")]
    NotHex(usize),
}

/// Generate a fresh 32-byte token rendered as 64 lowercase hex characters.
pub fn generate_token() -> String {
    let mut buf = [0u8; TOKEN_BYTES];
    rand::thread_rng().fill_bytes(&mut buf);
    to_hex(&buf)
}

/// Lowercase hex encoder; avoids pulling `hex` crate for one function.
fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

/// Write `token` to `path`, creating parent directories and enforcing
/// mode 0600 on unix (atomic: temp file + rename).
pub fn save_token(path: &Path, token: &str) -> Result<(), AuthError> {
    validate_hex(token)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    write_mode_600(path, token.as_bytes())?;
    Ok(())
}

/// Read the token from `path`, trimming trailing whitespace, and validate it.
pub fn load_token(path: &Path) -> Result<String, AuthError> {
    let raw = fs::read_to_string(path)?;
    let token = raw.trim().to_string();
    validate_hex(&token)?;
    Ok(token)
}

/// Validate the token is exactly `TOKEN_HEX_LEN` lowercase-or-uppercase hex.
pub fn validate_hex(token: &str) -> Result<(), AuthError> {
    if token.len() != TOKEN_HEX_LEN {
        return Err(AuthError::BadLength(token.len()));
    }
    for (i, b) in token.bytes().enumerate() {
        let ok = b.is_ascii_digit()
            || (b'a'..=b'f').contains(&b)
            || (b'A'..=b'F').contains(&b);
        if !ok {
            return Err(AuthError::NotHex(i));
        }
    }
    Ok(())
}

/// Constant-time-ish comparison (length + byte-level xor fold).
///
/// Case-insensitivity is a **hex-only** invariant: `validate_hex` accepts
/// mixed case, so a UI paste of an UPPERCASE token must match the
/// lowercase generated form. We gate that lowercasing on `expected`
/// passing `validate_hex` — for non-hex callers (e.g. an operator
/// supplying a base64 `KEI_TOKEN_OVERRIDE`) lowercasing would halve the
/// effective keyspace per character (62→36 for base64), so we fall
/// through to strict byte-level equality instead. Both branches keep the
/// branchless xor-fold over equal-length buffers.
pub fn tokens_match(expected: &str, got: &str) -> bool {
    if expected.len() != got.len() {
        return false;
    }
    if validate_hex(expected).is_ok() {
        let exp_lower = expected.to_ascii_lowercase();
        let got_lower = got.to_ascii_lowercase();
        let mut diff: u8 = 0;
        for (a, b) in exp_lower.bytes().zip(got_lower.bytes()) {
            diff |= a ^ b;
        }
        return diff == 0;
    }
    let mut diff: u8 = 0;
    for (a, b) in expected.bytes().zip(got.bytes()) {
        diff |= a ^ b;
    }
    diff == 0
}

/// Build a unique temp path next to `path`: `<path>.<nanos>.tmp`.
fn tmp_path(path: &Path) -> std::path::PathBuf {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let name = path.file_name().map(|n| {
        let mut s = n.to_os_string();
        s.push(format!(".{ts}.tmp"));
        s
    });
    match (path.parent(), name) {
        (Some(p), Some(n)) => p.join(n),
        _ => path.with_extension("tmp"),
    }
}

#[cfg(unix)]
fn write_mode_600(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    let tmp = tmp_path(path);
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&tmp)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    drop(f);
    fs::rename(&tmp, path)
}

#[cfg(not(unix))]
fn write_mode_600(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let tmp = tmp_path(path);
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tmp)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    drop(f);
    fs::rename(&tmp, path)
}

#[cfg(test)]
#[path = "auth_test.rs"]
mod tests;
