//! Rule-blocks validation — checks that each name in `manifest.rule_blocks`
//! exists in the kei-registry SQLite store.
//!
//! Constructor Pattern: one cube = one validation concern.
//! Extracted from `validator.rs` to keep all files under 200 LOC.

use crate::manifest::Manifest;
use crate::registry_client::RegistryClient;

/// Validate each name in `m.rule_blocks` exists in kei-registry.
///
/// When the registry DB is absent this is a soft warning only — the
/// assembler can still run on systems where kei-decompose hasn't
/// populated the registry yet (chicken-and-egg). A missing *DB* is
/// therefore not an error; a missing *fragment in an open DB* is.
pub fn check(m: &Manifest) -> Result<(), String> {
    if m.rule_blocks.is_empty() {
        return Ok(());
    }
    let Some(client) = RegistryClient::open() else {
        // DB absent — warn already emitted by RegistryClient::open(); skip.
        return Ok(());
    };
    for name in &m.rule_blocks {
        match client.find_rule(name) {
            Ok(Some(_)) => {}
            Ok(None) => {
                return Err(format!(
                    "rule_blocks: fragment '{name}' not found in kei-registry \
                     (run kei-registry scan to populate, or remove from manifest)"
                ));
            }
            Err(e) => return Err(format!("rule_blocks: {e}")),
        }
    }
    Ok(())
}
