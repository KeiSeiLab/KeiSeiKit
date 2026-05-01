//! Encyclopedia renderer — public API for markdown and JSON output.
//!
//! Constructor Pattern: this cube owns wire types + orchestration only.
//! Section builders live in `encyclopedia_render`; date formatting in
//! `encyclopedia_time`. No I/O beyond the return value.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::block::{Block, BlockType};
use crate::encyclopedia_render as render;
use crate::encyclopedia_time::utc_now;

// ── public wire types ──────────────────────────────────────────────────────

/// One flattened entry for the JSON surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncyclopediaEntry {
    pub block_type: String,
    pub name: String,
    pub dna: String,
    pub path: String,
    pub body_sha: String,
    pub caps: String,
    pub is_active: bool,
}

/// Top-level JSON envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encyclopedia {
    pub generated_at: String,
    pub total_blocks: u64,
    pub counts: BTreeMap<String, u64>,
    pub blocks: Vec<EncyclopediaEntry>,
}

// ── conversion ─────────────────────────────────────────────────────────────

/// Convert a `Block` slice into sorted `EncyclopediaEntry` values.
pub fn to_entries(blocks: &[Block]) -> Vec<EncyclopediaEntry> {
    let mut out: Vec<EncyclopediaEntry> = blocks
        .iter()
        .map(|b| EncyclopediaEntry {
            block_type: b.block_type.to_string(),
            name: b.name.clone(),
            dna: b.dna.clone(),
            path: b.path.clone(),
            body_sha: b.body_sha.clone(),
            caps: b.caps.clone(),
            is_active: b.is_active(),
        })
        .collect();
    out.sort_by(|a, b| a.block_type.cmp(&b.block_type).then(a.name.cmp(&b.name)));
    out
}

// ── JSON renderer ──────────────────────────────────────────────────────────

/// Render entries as pretty JSON.
pub fn render_json(entries: &[EncyclopediaEntry]) -> Result<String> {
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    for e in entries {
        *counts.entry(e.block_type.clone()).or_insert(0) += 1;
    }
    let enc = Encyclopedia {
        generated_at: utc_now(),
        total_blocks: entries.len() as u64,
        counts,
        blocks: entries.to_vec(),
    };
    Ok(serde_json::to_string_pretty(&enc)?)
}

// ── Markdown renderer ──────────────────────────────────────────────────────

/// Render active entries as a markdown encyclopedia.
/// `all_blocks` must include superseded rows for the chain section.
pub fn render_markdown(active: &[EncyclopediaEntry], all_blocks: &[Block]) -> String {
    let mut out = String::with_capacity(4096);
    let counts = type_counts(active);
    let total: u64 = counts.values().sum();

    render::push_header(&mut out, total, &counts);

    for bt in BlockType::all() {
        let key = bt.as_str();
        let section: Vec<&EncyclopediaEntry> =
            active.iter().filter(|e| e.block_type == key).collect();
        if section.is_empty() {
            continue;
        }
        render::push_section(&mut out, bt, &section);
    }

    render::push_supersede_chains(&mut out, all_blocks);
    render::push_schema_notes(&mut out);
    out
}

// ── private helpers ────────────────────────────────────────────────────────

fn type_counts(entries: &[EncyclopediaEntry]) -> BTreeMap<String, u64> {
    let mut m: BTreeMap<String, u64> = BTreeMap::new();
    for e in entries {
        *m.entry(e.block_type.clone()).or_insert(0) += 1;
    }
    m
}
