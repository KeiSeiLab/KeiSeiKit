// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

//! `SecretString` — a zeroising, redacting string newtype.
//!
//! Stores a sensitive value (password, API key, JWT secret) and ensures:
//! - `Debug` prints `"<redacted>"` — never the value.
//! - `Drop` zeroes the heap bytes before deallocation.
//!
//! The value is exposed only via [`SecretString::expose`], forcing callers
//! to be explicit about accessing the secret.

use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

/// A string whose `Debug` impl is redacted and whose `Drop` zeroes memory.
///
/// `Serialize` emits the literal `"<redacted>"` so that parent structs
/// that derive `Serialize` never accidentally leak the secret value.
/// Use [`SecretString::expose`] to access the real value explicitly.
#[derive(Clone, Deserialize)]
pub struct SecretString(String);

impl SecretString {
    /// Wrap a value. The caller is responsible for not logging the input.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Expose the raw value. Name is intentionally verbose.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

/// Serializes as the literal `"<redacted>"` — never the secret value.
///
/// This prevents accidental secret leaks when a parent struct that holds a
/// `SecretString` field also derives `Serialize`.
impl Serialize for SecretString {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str("<redacted>")
    }
}

/// Always prints `<redacted>` — never the secret value.
impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

/// Not `Display` — secrets should not be formatted accidentally.

impl Drop for SecretString {
    fn drop(&mut self) {
        // Safety: we are zeroing the heap bytes of the String before it is
        // freed. This is safe because:
        // 1. We have exclusive ownership (we are in Drop).
        // 2. The bytes are valid UTF-8 (all zeros is not, but the memory is
        //    about to be freed so UTF-8 invariant need not hold after).
        // SAFETY: we're in Drop; the Vec<u8> backing is about to be freed.
        unsafe {
            let buf = self.0.as_bytes_mut();
            for b in buf.iter_mut() {
                std::ptr::write_volatile(b, 0u8);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_is_redacted() {
        let s = SecretString::new("super-secret");
        assert_eq!(format!("{:?}", s), "<redacted>");
    }

    #[test]
    fn expose_returns_value() {
        let s = SecretString::new("my-password");
        assert_eq!(s.expose(), "my-password");
    }

    #[test]
    fn clone_is_independent() {
        let a = SecretString::new("abc");
        let b = a.clone();
        assert_eq!(a.expose(), b.expose());
    }

    #[test]
    fn serialize_emits_redacted_literal() {
        let s = SecretString::new("secret123");
        let json = serde_json::to_string(&s).expect("serialize must not fail");
        assert_eq!(json, "\"<redacted>\"");
    }
}
