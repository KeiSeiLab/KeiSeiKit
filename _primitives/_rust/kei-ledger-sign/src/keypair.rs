use crate::error::{Error, Result};
use crate::perms::open_600_write;
use ed25519_dalek::{SigningKey, VerifyingKey, SECRET_KEY_LENGTH};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct KeyFile {
    version: u32,
    private_hex: String,
    public_hex: String,
}

pub struct KeyPair {
    pub signing: SigningKey,
}

impl KeyPair {
    pub fn verifying(&self) -> VerifyingKey {
        self.signing.verifying_key()
    }
}

pub fn generate_keypair() -> KeyPair {
    let mut seed = [0u8; SECRET_KEY_LENGTH];
    OsRng.fill_bytes(&mut seed);
    let signing = SigningKey::from_bytes(&seed);
    KeyPair { signing }
}

pub fn save_keypair(kp: &KeyPair, path: &Path) -> Result<()> {
    let file = KeyFile {
        version: 1,
        private_hex: hex::encode(kp.signing.to_bytes()),
        public_hex: hex::encode(kp.verifying().to_bytes()),
    };
    let text = serde_json::to_string_pretty(&file)?;
    // Atomic save: write to <path>.tmp with mode 0o600 from the first byte,
    // then rename over the destination. rename(2) preserves the 0o600 mode
    // and is atomic on POSIX, so no other process can ever observe the
    // private key at mode 0o644.
    let tmp = tmp_path(path);
    let _ = std::fs::remove_file(&tmp);
    let mut f = open_600_write(&tmp)?;
    f.write_all(text.as_bytes())?;
    f.sync_all()?;
    drop(f);
    std::fs::rename(&tmp, path)?;
    Ok(())
}

fn tmp_path(path: &Path) -> std::path::PathBuf {
    let mut name = path
        .file_name()
        .map(|s| s.to_os_string())
        .unwrap_or_default();
    name.push(".tmp");
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.join(name),
        _ => std::path::PathBuf::from(name),
    }
}

pub fn load_keypair(path: &Path) -> Result<KeyPair> {
    let text = std::fs::read_to_string(path)?;
    let file: KeyFile = serde_json::from_str(&text)?;
    let priv_bytes = hex::decode(&file.private_hex)?;
    if priv_bytes.len() != SECRET_KEY_LENGTH {
        return Err(Error::KeyLength(priv_bytes.len()));
    }
    let mut seed = [0u8; SECRET_KEY_LENGTH];
    seed.copy_from_slice(&priv_bytes);
    let signing = SigningKey::from_bytes(&seed);
    let derived_pub = hex::encode(signing.verifying_key().to_bytes());
    if derived_pub != file.public_hex {
        return Err(Error::Signature(
            "stored public key does not match derived public key (tamper)".into(),
        ));
    }
    Ok(KeyPair { signing })
}
