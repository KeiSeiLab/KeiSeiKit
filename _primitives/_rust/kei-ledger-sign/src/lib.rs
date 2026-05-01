pub mod cli;
pub mod error;
pub mod keypair;
pub mod perms;
pub mod sign;

pub use error::{Error, Result};
pub use keypair::{generate_keypair, load_keypair, save_keypair, KeyPair};
pub use sign::{canonical_message, sign_row, verify_row};

pub use ed25519_dalek::{Signature, VerifyingKey};
