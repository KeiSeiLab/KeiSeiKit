//! Report — read scan output, aggregate, print plaintext table.
//!
//! Constructor Pattern: one responsibility = turn rows into a ranking.
//! Two group-by modes: `category` (default) and `session` (chatlog file).
//! Sort key = weighted score (count * weight), desc.
//!
//! Output is plaintext with fixed-width columns so the user can grep it.

use crate::row::{parse_csv, Row};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Group-by mode.
#[derive(Copy, Clone, Debug)]
pub enum GroupBy {
    Category,
    Session,
}

#[derive(Debug, Clone)]
pub struct Aggregate {
    pub key: String,
    pub count: usize,
    pub weighted: f64,
    pub top_example: String,
}

/// CLI entry. Reads rows from `input`, prints the top-N table.
pub fn run(input: &Path, top: usize, by: GroupBy) -> Result<()> {
    let rows = read_rows(input)?;
    let mut aggs = aggregate(&rows, by);
    sort_desc(&mut aggs);
    print_table(&aggs, top, by);
    Ok(())
}

fn read_rows(input: &Path) -> Result<Vec<Row>> {
    let body = fs::read_to_string(input)
        .with_context(|| format!("read {}", input.display()))?;
    if looks_like_jsonl(&body) {
        body.lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str::<Row>(l).context("parse jsonl row"))
            .collect()
    } else {
        parse_csv(&body)
    }
}

fn looks_like_jsonl(body: &str) -> bool {
    body.trim_start().starts_with('{')
}

/// Aggregate rows by the chosen key. `top_example` is the first quote
/// encountered in the group; the goal is to show a concrete sample row
/// the reviewer can grep for, not a statistical "representative".
pub fn aggregate(rows: &[Row], by: GroupBy) -> Vec<Aggregate> {
    let mut map: HashMap<String, Aggregate> = HashMap::new();
    for r in rows {
        let key = match by {
            GroupBy::Category => r.category.clone(),
            GroupBy::Session => r.chatlog_file.clone(),
        };
        let slot = map.entry(key.clone()).or_insert(Aggregate {
            key,
            count: 0,
            weighted: 0.0,
            top_example: r.quote.clone(),
        });
        slot.count += 1;
        slot.weighted += r.weight;
    }
    map.into_values().collect()
}

fn sort_desc(aggs: &mut [Aggregate]) {
    aggs.sort_by(|a, b| {
        b.weighted
            .partial_cmp(&a.weighted)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.count.cmp(&a.count))
            .then_with(|| a.key.cmp(&b.key))
    });
}

fn print_table(aggs: &[Aggregate], top: usize, by: GroupBy) {
    let hdr = match by {
        GroupBy::Category => "CATEGORY",
        GroupBy::Session => "SESSION",
    };
    println!("{:<28} {:>6} {:>9}  TOP_EXAMPLE", hdr, "COUNT", "WEIGHTED");
    for a in aggs.iter().take(top) {
        let key = clip(&a.key, 28);
        let ex = clip(&a.top_example, 60);
        println!(
            "{:<28} {:>6} {:>9.1}  {}",
            key, a.count, a.weighted, ex
        );
    }
    if aggs.is_empty() {
        println!("(no rows)");
    }
}

/// Clip long strings for table cells.
fn clip(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        return s.to_string();
    }
    let trimmed: String = s.chars().take(n.saturating_sub(1)).collect();
    format!("{trimmed}…")
}
