//! Schema-migration logic for the attach marker.
//!
//! Constructor Pattern: single responsibility — own the `WireRecord`
//! enum and its v1/v2/v3 → v4 lift. Extracted from `config.rs` in v0.22
//! so `config.rs` stays under the 200-LOC ceiling.
//!
//! Serde's `untagged` discrimination picks the first variant that
//! deserializes cleanly. Order: v4 first (strictest — carries
//! `schema_version` field), then v1/v2/v3 legacy shapes.

use crate::config::{AttachRecord, Attachment, CURRENT_SCHEMA};
use crate::scope::Scope;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum WireRecord {
    V4(WireV4),
    Legacy(WireLegacy),
}

#[derive(Debug, Deserialize)]
pub(crate) struct WireV4 {
    pub schema_version: u32,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WireLegacy {
    pub brain_path: String,
    pub brain_name: String,
    pub attached_at: String,
    #[serde(default)]
    pub client_type: Option<String>,
    #[serde(default)]
    pub attachments: Vec<LegacyAttachment>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LegacyAttachment {
    pub client_type: String,
    #[serde(default)]
    pub config_path: String,
    #[serde(default)]
    pub scope: Scope,
}

impl WireRecord {
    pub(crate) fn into_current(self) -> (AttachRecord, Option<u32>) {
        match self {
            WireRecord::V4(v4) => (
                AttachRecord {
                    schema_version: v4.schema_version,
                    attachments: v4.attachments,
                },
                None,
            ),
            WireRecord::Legacy(legacy) => {
                let from = legacy_version(&legacy);
                (legacy_to_v4(legacy), Some(from))
            }
        }
    }
}

/// Best-effort classification of the legacy shape. v1 = flat
/// `client_type` string only; otherwise treat as v3 (v2 and v3 are
/// shape-equivalent after deserialization).
fn legacy_version(legacy: &WireLegacy) -> u32 {
    if legacy.attachments.is_empty() && legacy.client_type.is_some() {
        1
    } else {
        3
    }
}

fn legacy_to_v4(legacy: WireLegacy) -> AttachRecord {
    let WireLegacy {
        brain_path,
        brain_name,
        attached_at,
        client_type,
        attachments,
    } = legacy;
    let v4_attachments: Vec<Attachment> = if !attachments.is_empty() {
        attachments
            .into_iter()
            .map(|a| Attachment {
                brain_path: brain_path.clone(),
                brain_name: brain_name.clone(),
                client_type: a.client_type,
                config_path: a.config_path,
                scope: a.scope,
                attached_at: attached_at.clone(),
            })
            .collect()
    } else if let Some(ct) = client_type {
        vec![Attachment {
            brain_path,
            brain_name,
            client_type: ct,
            config_path: String::new(),
            scope: Scope::User,
            attached_at,
        }]
    } else {
        Vec::new()
    };
    AttachRecord {
        schema_version: CURRENT_SCHEMA,
        attachments: v4_attachments,
    }
}
