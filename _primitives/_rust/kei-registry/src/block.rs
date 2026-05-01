//! Block — registry record for one kit artefact.
//!
//! Constructor Pattern: this cube owns the data shape only. SQL persistence
//! lives in `store.rs`; query helpers live in `registry.rs`. Block is what
//! flows over the JSON CLI surface.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Five recognised block types. Order is the canonical scan order and the
/// wire-format `<role>` segment of the block DNA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockType {
    Primitive,
    Skill,
    Rule,
    Hook,
    Atom,
}

impl BlockType {
    /// Stable wire-format string. Used as the DNA `role` segment.
    pub fn as_str(self) -> &'static str {
        match self {
            BlockType::Primitive => "primitive",
            BlockType::Skill => "skill",
            BlockType::Rule => "rule",
            BlockType::Hook => "hook",
            BlockType::Atom => "atom",
        }
    }

    /// All five recognised types in canonical scan order.
    pub fn all() -> &'static [BlockType] {
        &[
            BlockType::Primitive,
            BlockType::Skill,
            BlockType::Rule,
            BlockType::Hook,
            BlockType::Atom,
        ]
    }
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for BlockType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "primitive" => Ok(BlockType::Primitive),
            "skill" => Ok(BlockType::Skill),
            "rule" => Ok(BlockType::Rule),
            "hook" => Ok(BlockType::Hook),
            "atom" => Ok(BlockType::Atom),
            other => Err(format!("unknown block_type: {other}")),
        }
    }
}

/// Block — single registry record. Mirrors the SQLite `blocks` row plus
/// the synthetic DNA composed from the other facets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub id: i64,
    pub dna: String,
    pub block_type: BlockType,
    pub name: String,
    pub path: String,
    pub caps: String,
    pub scope_sha: String,
    pub body_sha: String,
    pub nonce: String,
    pub created: i64,
    pub modified: i64,
    pub superseded_by: Option<String>,
}

impl Block {
    /// True if no successor row points at this block.
    pub fn is_active(&self) -> bool {
        self.superseded_by.is_none()
    }
}
