use kei_search_core::budget::Budget;
use kei_search_core::export::{export, Format};
use kei_search_core::fetch::{SourceFetcher, StubFetcher};
use kei_search_core::pipeline::run_research;
use kei_search_core::types::Source;
use kei_search_core::ResearchStore;

fn mk() -> ResearchStore { ResearchStore::open_memory().unwrap() }

struct FakeFetcher;
impl SourceFetcher for FakeFetcher {
    fn fetch(&self, claim: &str) -> (Vec<Source>, i64) {
        (vec![Source {
            url: "https://example.test".into(),
            title: format!("source for: {claim}"),
            content: "body".into(),
            provider: "fake".into(),
            domain: "example.test".into(),
            relevance_score: 0.8,
            ..Default::default()
        }], 10)
    }
}

#[test]
fn budget_enforcement() {
    let mut b = Budget::new(100);
    b.charge(50).unwrap();
    b.charge(40).unwrap();
    assert!(b.charge(20).is_err(), "must reject overspend");
}

#[test]
fn wave_progression_creates_research() {
    let s = mk();
    let id = run_research(&s, &FakeFetcher,
        "Rust is memory-safe. Python is dynamic.", 10_000).unwrap();
    let r = s.get_research(id).unwrap().unwrap();
    assert_eq!(r.status, "completed");
    assert!(r.total_cost_mc > 0);
    assert!(s.claims_for(id).unwrap().len() >= 2);
}

#[test]
fn consensus_scoring_applies_grade() {
    let s = mk();
    let id = run_research(&s, &FakeFetcher, "One claim here.", 10_000).unwrap();
    let claims = s.claims_for(id).unwrap();
    assert!(!claims.is_empty());
    assert!(!claims[0].grade.is_empty());
}

#[test]
fn export_markdown_and_json() {
    let s = mk();
    let id = run_research(&s, &FakeFetcher, "Claim A. Claim B.", 10_000).unwrap();
    let md = export(&s, id, Format::Markdown).unwrap();
    assert!(md.contains("# Research"));
    let js = export(&s, id, Format::Json).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&js).unwrap();
    assert!(parsed.get("claims").is_some());
}

#[test]
fn stop_mid_run_marks_status() {
    let s = mk();
    let id = run_research(&s, &StubFetcher, "x. y.", 10_000).unwrap();
    s.set_status(id, "stopped").unwrap();
    let r = s.get_research(id).unwrap().unwrap();
    assert_eq!(r.status, "stopped");
}

#[test]
fn budget_exhausted_rejects_run() {
    let s = mk();
    // 3 claims × 100mc + 50mc wave2 = 350mc; budget 100 → must overspend.
    let err = run_research(&s, &StubFetcher,
        "alpha claim one. beta claim two. gamma claim three.", 100);
    assert!(err.is_err(), "small budget vs 3 claims must overspend");
}
