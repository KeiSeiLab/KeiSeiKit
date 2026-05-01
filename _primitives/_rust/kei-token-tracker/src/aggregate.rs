//! Per-model aggregation queries used by `sleep-report` and CLI.

use serde::{Deserialize, Serialize};

/// Rolled-up usage for a single (model) since a unix epoch lower bound.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelAggregate {
    pub model: String,
    pub events: u32,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub micro_cents: u64,
}

impl ModelAggregate {
    /// Sum of input + output tokens. Convenience for the report renderer.
    pub fn total_tokens(&self) -> u64 {
        self.input_tokens.saturating_add(self.output_tokens)
    }
}

/// 1 cent = 1_000_000 micro-cents — mirrors `kei_ledger::MICRO_CENTS_PER_CENT`.
pub const MICRO_CENTS_PER_CENT: u64 = 1_000_000;

/// Render a micro-cents value as a USD-display string with two decimals.
/// Truncating; callers that need bankers' rounding can convert before
/// formatting. Used by `sleep_report` and the CLI `aggregate` printer.
pub fn format_usd(micro_cents: u64) -> String {
    let total_cents = micro_cents / MICRO_CENTS_PER_CENT;
    let dollars = total_cents / 100;
    let cents = total_cents % 100;
    format!("${dollars}.{cents:02}")
}

/// Sum micro-cents across a slice of aggregates. Pure helper; saves the
/// two consumers (sleep-report header + CLI aggregate footer) from
/// re-implementing the fold.
pub fn total_micro_cents(rows: &[ModelAggregate]) -> u64 {
    rows.iter()
        .fold(0u64, |acc, r| acc.saturating_add(r.micro_cents))
}

/// Sum total token counts (input + output) across a slice.
pub fn total_tokens(rows: &[ModelAggregate]) -> u64 {
    rows.iter().fold(0u64, |acc, r| acc.saturating_add(r.total_tokens()))
}

/// Sum input tokens specifically — sleep-report header line item.
pub fn total_input_tokens(rows: &[ModelAggregate]) -> u64 {
    rows.iter().fold(0u64, |acc, r| acc.saturating_add(r.input_tokens))
}

/// Sum output tokens specifically — sleep-report header line item.
pub fn total_output_tokens(rows: &[ModelAggregate]) -> u64 {
    rows.iter().fold(0u64, |acc, r| acc.saturating_add(r.output_tokens))
}

/// Sum event counts across all aggregates.
pub fn total_events(rows: &[ModelAggregate]) -> u32 {
    rows.iter().fold(0u32, |acc, r| acc.saturating_add(r.events))
}
