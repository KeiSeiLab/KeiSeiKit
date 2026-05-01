//! matcher — heuristic trait-pattern matcher over a ModuleSource.
//!
//! Uses regex-based extraction of impl blocks and method names for
//! improved precision over raw substring search. Avoids false positives
//! from comments and string literals. No syn/AST dependency.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use crate::module_source::ModuleSource;
use crate::trait_patterns::{all_patterns, TraitKind};
use std::sync::OnceLock;

/// Confidence threshold below which a match is omitted.
const MIN_CONFIDENCE: f64 = 0.3;

/// A single trait-match result for one pattern.
#[derive(Debug, Clone, serde::Serialize)]
pub struct MatchScore {
    pub kind: TraitKind,
    /// Normalised confidence in [0.0, 1.0].
    pub confidence: f64,
    /// Required methods that were found in the source.
    pub matched_methods: Vec<String>,
    /// Indicator keywords that were found in the source.
    pub matched_keywords: Vec<String>,
}

struct Fingerprint {
    method_names: Vec<String>,
    trait_impl_names: Vec<String>,
    use_segments: Vec<String>,
}

/// Analyse all source files in `source` and return confident trait matches.
pub fn match_module(source: &ModuleSource) -> Vec<MatchScore> {
    let fp = extract_fingerprint(source);
    let mut results: Vec<MatchScore> = all_patterns()
        .iter()
        .filter_map(|p| score_pattern(p, &fp))
        .collect();
    results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    results
}

fn impl_trait_re() -> &'static regex::Regex {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r"\bimpl\s+(\w+)\s+for\s+\w+").unwrap())
}

fn fn_name_re() -> &'static regex::Regex {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r"(?:async\s+)?fn\s+(\w+)\s*[<(]").unwrap())
}

fn use_segment_re() -> &'static regex::Regex {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r"\buse\s+((?:\w+::)*\w+)").unwrap())
}

fn extract_fingerprint(source: &ModuleSource) -> Fingerprint {
    let mut method_names = Vec::new();
    let mut trait_impl_names = Vec::new();
    let mut use_segments = Vec::new();
    for (_path, content) in &source.source_files {
        let stripped = strip_string_literals(content);
        for cap in impl_trait_re().captures_iter(&stripped) {
            trait_impl_names.push(cap[1].to_owned());
        }
        for cap in fn_name_re().captures_iter(&stripped) {
            method_names.push(cap[1].to_owned());
        }
        for cap in use_segment_re().captures_iter(&stripped) {
            for seg in cap[1].split("::") {
                use_segments.push(seg.to_owned());
            }
        }
    }
    Fingerprint { method_names, trait_impl_names, use_segments }
}

/// Replace string literal contents + line comments with spaces.
fn strip_string_literals(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut in_string = false;
    let mut escape = false;
    let mut in_comment = false;
    let mut prev_slash = false;
    for ch in src.chars() {
        if in_comment {
            out.push(if ch == '\n' { in_comment = false; '\n' } else { ' ' });
            prev_slash = false;
            continue;
        }
        if !in_string && prev_slash && ch == '/' {
            in_comment = true;
            out.push(' ');
            prev_slash = false;
            continue;
        }
        prev_slash = ch == '/' && !in_string;
        if escape { escape = false; out.push(' '); continue; }
        if ch == '\\' && in_string { escape = true; out.push(' '); continue; }
        if ch == '"' { in_string = !in_string; out.push(ch); continue; }
        if in_string { out.push(' '); } else { out.push(ch); }
    }
    out
}

fn score_pattern(p: &crate::trait_patterns::TraitPattern, fp: &Fingerprint) -> Option<MatchScore> {
    for forbidden in p.forbidden_deps {
        if fp.use_segments.iter().any(|s| s == forbidden) { return None; }
    }
    let (method_score, matched_methods) = score_methods(p.required_methods, &fp.method_names);
    let (kw_score, matched_keywords) = score_keywords(p.indicator_keywords, fp);
    let confidence = method_score * 0.6 + kw_score * 0.4;
    if confidence < MIN_CONFIDENCE { return None; }
    Some(MatchScore { kind: p.kind, confidence, matched_methods, matched_keywords })
}

fn score_methods(required: &[&str], method_names: &[String]) -> (f64, Vec<String>) {
    let mut matched = Vec::new();
    for &m in required {
        if method_names.iter().any(|n| n == m) { matched.push(m.to_owned()); }
    }
    let score = if required.is_empty() { 0.0 } else { matched.len() as f64 / required.len() as f64 };
    (score, matched)
}

fn score_keywords(keywords: &[&str], fp: &Fingerprint) -> (f64, Vec<String>) {
    let corpus = [fp.method_names.join(" "), fp.trait_impl_names.join(" "), fp.use_segments.join(" ")].join(" ");
    let mut matched = Vec::new();
    for &kw in keywords {
        if corpus.contains(kw) { matched.push(kw.to_owned()); }
    }
    let score = if keywords.is_empty() { 0.0 } else { matched.len() as f64 / keywords.len() as f64 };
    (score, matched)
}

// Tests live in tests/matcher_tests.rs to keep this file ≤200 LOC.
