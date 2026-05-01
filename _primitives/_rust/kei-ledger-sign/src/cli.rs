use crate::error::{Error, Result};
use crate::keypair::{generate_keypair, load_keypair, save_keypair};
use crate::sign::{sign_row, verify_row};
use ed25519_dalek::{Signature, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use std::path::Path;

pub fn cmd_keygen(out: &Path) -> Result<()> {
    let kp = generate_keypair();
    save_keypair(&kp, out)?;
    println!("{}", hex::encode(kp.verifying().to_bytes()));
    Ok(())
}

pub fn cmd_sign(
    key: &Path,
    dna: &str,
    spec_sha: &str,
    creator_id: &str,
) -> Result<()> {
    let kp = load_keypair(key)?;
    let sig = sign_row(&kp, dna, spec_sha, creator_id)?;
    println!("{}", hex::encode(sig.to_bytes()));
    Ok(())
}

fn parse_pubkey(pubkey_hex: &str) -> Result<VerifyingKey> {
    let bytes = hex::decode(pubkey_hex)?;
    if bytes.len() != PUBLIC_KEY_LENGTH {
        return Err(Error::KeyLength(bytes.len()));
    }
    let mut arr = [0u8; PUBLIC_KEY_LENGTH];
    arr.copy_from_slice(&bytes);
    Ok(VerifyingKey::from_bytes(&arr)?)
}

fn parse_signature(sig_hex: &str) -> Result<Signature> {
    let bytes = hex::decode(sig_hex)?;
    if bytes.len() != SIGNATURE_LENGTH {
        return Err(Error::KeyLength(bytes.len()));
    }
    let mut arr = [0u8; SIGNATURE_LENGTH];
    arr.copy_from_slice(&bytes);
    Ok(Signature::from_bytes(&arr))
}

pub fn cmd_verify(
    pubkey_hex: &str,
    dna: &str,
    spec_sha: &str,
    creator_id: &str,
    sig_hex: &str,
) -> Result<()> {
    let pubkey = parse_pubkey(pubkey_hex)?;
    let sig = parse_signature(sig_hex)?;
    verify_row(&pubkey, dna, spec_sha, creator_id, &sig)
}
