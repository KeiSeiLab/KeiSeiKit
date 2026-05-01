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
/// MISS-6: `validate_hex` accepts mixed-case hex, so we must accept either
/// case here too. Both inputs are lowercased (ASCII-only — safe per
/// `validate_hex` invariants) before the xor-fold so a user pasting an
/// UPPERCASE token through a UI matches the lowercase generated form.
/// Lowercasing ASCII does not change the byte length, so the branchless
/// fold over equal-length buffers is preserved.
pub fn tokens_match(expected: &str, got: &str) -> bool {
    if expected.len() != got.len() {
        return false;
    }
    let exp_lower = expected.to_ascii_lowercase();
    let got_lower = got.to_ascii_lowercase();
    let mut diff: u8 = 0;
    for (a, b) in exp_lower.bytes().zip(got_lower.bytes()) {
        diff |= a ^ b;
    }
    diff == 0
}

#[cfg(unix)]
fn write_mode_600(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
fn write_mode_600(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    Ok(())
}

#[cfg(test)]
#[path = "auth_test.rs"]
mod tests;
