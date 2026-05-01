//! Ed25519 identity (RFC 8032) — no proprietary crypto, no matrix math.
//!
//! Identity flow:
//!   1. Client generates `Keypair` on first run (`generate_keypair`).
//!   2. `user_id` is the first 16 hex chars of `blake3(public_key_bytes)`.
//!   3. Requests are signed with the private key; the server verifies using
//!      the advertised public key.
//!
//! The public key is safe to publish; the private key is stored locally in
//! `~/.keisei/identity.key` (filesystem permissions `0600`).

use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Signer, Verifier};
use rand_core::OsRng;

#[derive(Debug, Clone)]
pub struct Keypair {
    pub signing: SigningKey,
}

impl Keypair {
    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing.verifying_key()
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.signing.sign(msg)
    }

    pub fn public_hex(&self) -> String {
        hex::encode(self.verifying_key().as_bytes())
    }

    pub fn secret_hex(&self) -> String {
        hex::encode(self.signing.to_bytes())
    }

    pub fn user_id(&self) -> String {
        user_id_from_pubkey(self.verifying_key().as_bytes())
    }

    /// Reconstruct from a 32-byte secret hex string.
    pub fn from_secret_hex(hex_str: &str) -> Result<Self, anyhow::Error> {
        let bytes = hex::decode(hex_str)?;
        if bytes.len() != 32 {
            anyhow::bail!("secret key must be 32 bytes (got {})", bytes.len());
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Keypair { signing: SigningKey::from_bytes(&arr) })
    }
}

/// Derive a stable 16-hex-char user id from a 32-byte Ed25519 public key.
pub fn user_id_from_pubkey(pubkey: &[u8; 32]) -> String {
    let h = blake3::hash(pubkey);
    hex::encode(&h.as_bytes()[..8])
}

/// Generate a fresh Ed25519 keypair using the OS RNG.
pub fn generate_keypair() -> Keypair {
    Keypair { signing: SigningKey::generate(&mut OsRng) }
}

/// Verify a signature against a public key and message.
pub fn verify(pubkey_hex: &str, msg: &[u8], sig_hex: &str) -> Result<(), anyhow::Error> {
    let pub_bytes = hex::decode(pubkey_hex)?;
    if pub_bytes.len() != 32 {
        anyhow::bail!("public key must be 32 bytes (got {})", pub_bytes.len());
    }
    let mut pub_arr = [0u8; 32];
    pub_arr.copy_from_slice(&pub_bytes);
    let verifying = VerifyingKey::from_bytes(&pub_arr)?;

    let sig_bytes = hex::decode(sig_hex)?;
    if sig_bytes.len() != 64 {
        anyhow::bail!("signature must be 64 bytes (got {})", sig_bytes.len());
    }
    let mut sig_arr = [0u8; 64];
    sig_arr.copy_from_slice(&sig_bytes);
    let sig = Signature::from_bytes(&sig_arr);

    verifying.verify(msg, &sig).map_err(|e| anyhow::anyhow!("signature verify failed: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_sign_verify() {
        let kp = generate_keypair();
        let msg = b"hello pet";
        let sig = kp.sign(msg);
        assert!(kp.verifying_key().verify(msg, &sig).is_ok());
    }

    #[test]
    fn user_id_is_16_hex() {
        let kp = generate_keypair();
        let id = kp.user_id();
        assert_eq!(id.len(), 16);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn user_id_is_deterministic() {
        let kp = generate_keypair();
        let id1 = kp.user_id();
        let id2 = user_id_from_pubkey(kp.verifying_key().as_bytes());
        assert_eq!(id1, id2);
    }

    #[test]
    fn secret_roundtrip() {
        let kp1 = generate_keypair();
        let hex = kp1.secret_hex();
        let kp2 = Keypair::from_secret_hex(&hex).unwrap();
        assert_eq!(kp1.public_hex(), kp2.public_hex());
        assert_eq!(kp1.user_id(), kp2.user_id());
    }

    #[test]
    fn verify_via_hex_api() {
        let kp = generate_keypair();
        let msg = b"cross-boundary verify";
        let sig = kp.sign(msg);
        let sig_hex = hex::encode(sig.to_bytes());
        assert!(verify(&kp.public_hex(), msg, &sig_hex).is_ok());
    }
}
