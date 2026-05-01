//! Phase D nightly markdown sleep-report rendering.
//!
//! Pure function: takes a sorted `Vec<ModelAggregate>` + a date string
//! and emits a deterministic markdown payload. No filesystem side-effects
//! here — the CLI dispatcher writes the bytes.

use crate::aggregate::{
    format_usd, total_events, total_input_tokens, total_micro_cents, total_output_tokens,
    ModelAggregate,
};

/// Render the markdown report. `date` is the report header (e.g.
/// `"2026-05-01"`). The renderer assumes `rows` is already sorted in
/// the order the caller wants displayed.
pub fn render(date: &str, rows: &[ModelAggregate]) -> String {
    let mut out = String::with_capacity(256 + rows.len() * 80);
    out.push_str(&format!("# Token usage report — {date}\n\n"));
    push_summary(&mut out, rows);
    out.push_str("\n## Per model\n");
    push_table(&mut out, rows);
    out
}

fn push_summary(out: &mut String, rows: &[ModelAggregate]) {
    let events = total_events(rows);
    let in_tok = total_input_tokens(rows);
    let out_tok = total_output_tokens(rows);
    let micro = total_micro_cents(rows);
    out.push_str("## Summary\n");
    out.push_str(&format!("- Total events: {events}\n"));
    out.push_str(&format!(
        "- Total tokens: {in_tok} in / {out_tok} out\n"
    ));
    out.push_str(&format!("- Total cost: {}\n", format_usd(micro)));
}

fn push_table(out: &mut String, rows: &[ModelAggregate]) {
    out.push_str("| model | events | tokens in | tokens out | cost |\n");
    out.push_str("|---|---|---|---|---|\n");
    if rows.is_empty() {
        out.push_str("| _(no events in window)_ |  |  |  |  |\n");
        return;
    }
    for r in rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            r.model,
            r.events,
            r.input_tokens,
            r.output_tokens,
            format_usd(r.micro_cents),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_fixed_event_set() {
        let rows = vec![
            ModelAggregate {
                model: "claude-haiku-4-5".into(),
                events: 3,
                input_tokens: 1500,
                output_tokens: 700,
                micro_cents: 12_345_000,
            },
            ModelAggregate {
                model: "gpt-4o".into(),
                events: 1,
                input_tokens: 200,
                output_tokens: 100,
                micro_cents: 5_000_000,
            },
        ];
        let md = render("2026-05-01", &rows);
        assert!(md.starts_with("# Token usage report — 2026-05-01\n"));
        assert!(md.contains("- Total events: 4"));
        assert!(md.contains("- Total tokens: 1700 in / 800 out"));
        assert!(md.contains("- Total cost: $0.17"));
        assert!(md.contains("| claude-haiku-4-5 | 3 | 1500 | 700 | $0.12 |"));
        assert!(md.contains("| gpt-4o | 1 | 200 | 100 | $0.05 |"));
    }

    #[test]
    fn renders_empty_window_placeholder() {
        let md = render("2026-05-01", &[]);
        assert!(md.contains("- Total events: 0"));
        assert!(md.contains("_(no events in window)_"));
    }
}
