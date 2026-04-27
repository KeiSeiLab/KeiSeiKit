//! Integration tests for frustration-matrix.
//!
//! Constructor Pattern: each test = one scenario, one assertion target.
//! We load source modules via `#[path]` so we don't need to expose a
//! library crate surface (matches the pattern used in kei-memory).

#[path = "../src/categories.rs"]
mod categories;
#[path = "../src/markdown.rs"]
mod markdown;
#[path = "../src/report.rs"]
mod report;
#[path = "../src/row.rs"]
mod row;
#[path = "../src/since.rs"]
mod since;

use std::path::PathBuf;

use categories::compile_all;
use markdown::parse as parse_md;
use report::{aggregate, GroupBy};
use row::Row;

// ---------------------------------------------------------------
// 1. categories_compile — every trigger compiles without panic.
// ---------------------------------------------------------------
#[test]
fn categories_compile() {
    let cats = compile_all();
    assert_eq!(cats.len(), 5, "expected 5 seed categories");
    for c in &cats {
        assert!(!c.patterns.is_empty(), "category {} has no patterns", c.id);
    }
}

// ---------------------------------------------------------------
// 2. detect_repeat_signal — "я же уже просил" → matches repeat-signal.
// ---------------------------------------------------------------
#[test]
fn detect_repeat_signal() {
    let cats = compile_all();
    let repeat = cats
        .iter()
        .find(|c| c.id == "repeat-signal")
        .expect("seed must contain repeat-signal");
    let hits = ["я же уже просил", "Again, I told you", "уже говорил"];
    for h in hits {
        assert!(
            repeat.patterns.iter().any(|p| p.is_match(h)),
            "repeat-signal did not match {h:?}"
        );
    }
}

// ---------------------------------------------------------------
// 3. detect_conservative_framing — "это всё что мы смогли" hits.
// ---------------------------------------------------------------
#[test]
fn detect_conservative_framing() {
    let cats = compile_all();
    let cons = cats
        .iter()
        .find(|c| c.id == "conservative-framing")
        .expect("seed must contain conservative-framing");
    let hits = [
        "это всё что мы смогли",
        "Let's accept as limitation",
        "this is a downgrade",
        "refuted finally, move on",
    ];
    for h in hits {
        assert!(
            cons.patterns.iter().any(|p| p.is_match(h)),
            "conservative-framing did not match {h:?}"
        );
    }
}

// ---------------------------------------------------------------
// 4. markdown_parses_user_blocks — 3 `### User` blocks → 3+ UserLines,
//    none coming from an assistant block.
// ---------------------------------------------------------------
#[test]
fn markdown_parses_user_blocks() {
    let md = r#"# Header

### User question 1
почему ты опять полез не туда?

### Assistant
Sorry, let me reconsider.

### User question 2
я же уже просил — не трогай это.

### Assistant
Understood.

### User question 3
стоп. переделай."#;
    let path = PathBuf::from("fixture.md");
    let lines = parse_md(&path, md);
    let texts: Vec<_> = lines.iter().map(|u| u.text.as_str()).collect();
    assert!(
        texts.iter().any(|t| t.contains("почему ты опять")),
        "expected user line 1 in {texts:?}"
    );
    assert!(
        texts.iter().any(|t| t.contains("уже просил")),
        "expected user line 2 in {texts:?}"
    );
    assert!(
        texts.iter().any(|t| t.contains("стоп")),
        "expected user line 3 in {texts:?}"
    );
    assert!(
        !texts.iter().any(|t| t.contains("Sorry, let me reconsider")),
        "assistant line leaked into user output: {texts:?}"
    );
    assert!(
        !texts.iter().any(|t| t.contains("Understood")),
        "assistant line leaked into user output: {texts:?}"
    );
}

// ---------------------------------------------------------------
// 5. report_ranking_orders_by_score — synthetic rows, check top order.
// ---------------------------------------------------------------
#[test]
fn report_ranking_orders_by_score() {
    let rows = vec![
        row("frustration-tone", "a.md", 1.0, "стоп"),
        row("frustration-tone", "a.md", 1.0, "хватит"),
        row("conservative-framing", "b.md", 2.0, "downgrade"),
        row("conservative-framing", "b.md", 2.0, "limitation"),
        row("conservative-framing", "b.md", 2.0, "refuted finally"),
        row("repeat-signal", "c.md", 2.5, "опять"),
        row("repeat-signal", "c.md", 2.5, "уже говорил"),
    ];
    let mut aggs = aggregate(&rows, GroupBy::Category);
    aggs.sort_by(|a, b| {
        b.weighted
            .partial_cmp(&a.weighted)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    // conservative-framing: 3 × 2.0 = 6.0
    // repeat-signal:        2 × 2.5 = 5.0
    // frustration-tone:     2 × 1.0 = 2.0
    assert_eq!(aggs[0].key, "conservative-framing");
    assert_eq!(aggs[1].key, "repeat-signal");
    assert_eq!(aggs[2].key, "frustration-tone");
    assert!((aggs[0].weighted - 6.0).abs() < 1e-9);
    assert!((aggs[1].weighted - 5.0).abs() < 1e-9);
    assert!((aggs[2].weighted - 2.0).abs() < 1e-9);
}

fn row(cat: &str, file: &str, w: f64, quote: &str) -> Row {
    Row {
        category: cat.to_string(),
        chatlog_file: file.to_string(),
        line_no: 1,
        timestamp: "0s".to_string(),
        quote: quote.to_string(),
        weight: w,
    }
}
