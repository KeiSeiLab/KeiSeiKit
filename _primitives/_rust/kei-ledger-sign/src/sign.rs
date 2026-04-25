use crate::error::{Error, Result};
use crate::keypair::KeyPair;
use ed25519_dalek::{Signature, Signer, Verifier, VerifyingKey};

fn reject_pipe(name: &str, value: &str) -> Result<()> {
    if value.contains('|') {
        return Err(Error::MessageSeparator(format!(
            "{}={}",
            name, value
        )));
    }
    Ok(())
}

pub fn canonical_message(
    dna: &str,
    spec_sha: &str,
    creator_id: &str,
) -> Result<Vec<u8>> {
    reject_pipe("dna", dna)?;
    reject_pipe("spec_sha", spec_sha)?;
    reject_pipe("creator_id", creator_id)?;
    Ok(format!("{}|{}|{}", dna, spec_sha, creator_id).into_bytes())
}

pub fn sign_row(
    kp: &KeyPair,
    dna: &str,
    spec_sha: &str,
    creator_id: &str,
) -> Result<Signature> {
    let msg = canonical_message(dna, spec_sha, creator_id)?;
    Ok(kp.signing.sign(&msg))
}

pub fn verify_row(
    pubkey: &VerifyingKey,
    dna: &str,
    spec_sha: &str,
    creator_id: &str,
    sig: &Signature,
) -> Result<()> {
    let msg = canonical_message(dna, spec_sha, creator_id)?;
    pubkey.verify(&msg, sig)?;
    Ok(())
}
