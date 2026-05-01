//! Cosine similarity over sparse term-weight maps.
//!
//! Constructor Pattern: one cube, one pure-math responsibility.
//! Classical numerator = Σ a·b over shared keys;
//! classical denominator = ‖a‖₂ · ‖b‖₂. No normalize-to-Frobenius, no rank
//! projection — just textbook cosine on HashMap<String, f64>.

use std::collections::HashMap;

/// Cosine similarity between two sparse vectors keyed by token.
pub fn cosine_tfidf(a: &HashMap<String, f64>, b: &HashMap<String, f64>) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0f64;
    for (k, va) in a {
        if let Some(vb) = b.get(k) {
            dot += va * vb;
        }
    }
    let norm_a: f64 = a.values().map(|v| v * v).sum::<f64>().sqrt();
    let norm_b: f64 = b.values().map(|v| v * v).sum::<f64>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}
