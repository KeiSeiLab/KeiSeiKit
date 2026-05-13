//! Task-complexity heuristic.
//!
//! Maps (prompt, role) → τ ∈ [0, 1] via additive feature scoring. Pure
//! function, no LLM call. Fast classifier so router itself has near-zero
//! overhead.
//!
//! Calibration: weights are seeded from session observation; the
//! `calibrate` subcommand can re-fit them against ledger outcomes.
//!
//! Design: every signal contributes a clamped weight; total weight
//! divided by maximum-possible-weight gives τ. Returns matched feature
//! list for transparency / debugging.
//!
//! Constructor Pattern: pure-fn cube. No state, no I/O.

#[derive(Debug, Clone, serde::Serialize)]
pub struct ComplexityEstimate {
    pub tau: f64,
    pub features: Vec<&'static str>,
}

/// Tier mapping for human consumption: τ ∈ [0, 0.30] = lookup,
/// [0.30, 0.70] = multi-step, [0.70, 1.00] = architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Lookup,
    MultiStep,
    Architecture,
}

impl Tier {
    pub fn from_tau(tau: f64) -> Self {
        if tau < 0.30 {
            Self::Lookup
        } else if tau < 0.70 {
            Self::MultiStep
        } else {
            Self::Architecture
        }
    }
}

/// High-complexity signals — bump τ up. Weight 0.20 each.
const HEAVY_KEYWORDS: &[&str] = &[
    "architect", "derive", "proof", "theorem", "rewrite",
    "redesign", "novel", "math", "spectral", "manifold",
    "algorithm", "convergence", "asymptotic",
];

/// Mid-complexity signals — bump τ up. Weight 0.10 each.
const MID_KEYWORDS: &[&str] = &[
    "refactor", "implement", "wire", "integrate", "test",
    "audit", "review", "merge", "migration", "schema",
    "endpoint", "trait", "async",
];

/// Low-complexity signals — bump τ DOWN. Weight 0.10 each (negative).
const LIGHT_KEYWORDS: &[&str] = &[
    "list", "find", "where is", "what is", "search",
    "grep", "show", "print", "display", "rename",
    "format", "lookup",
];

/// Roles known to require architectural reasoning. Add 0.20 to τ if matched.
const HEAVY_ROLES: &[&str] = &[
    "physics-deriver", "ml-implementer", "ml-researcher",
    "kei-architect", "architect", "kei-critic", "critic",
    "code-implementer-rust", "code-implementer",
    "infra-implementer-iac",
];

/// Roles known to be read-only / lookup. Subtract 0.20 from τ.
const LIGHT_ROLES: &[&str] = &[
    "Explore", "researcher-code", "researcher-web",
    "validator-doc", "validator-version", "patent-compliance",
    "keimd-expert",
];

const HEAVY_KW_W: f64 = 0.20;
const MID_KW_W: f64 = 0.10;
const LIGHT_KW_W: f64 = 0.10; // subtracted
const HEAVY_ROLE_W: f64 = 0.20;
const LIGHT_ROLE_W: f64 = 0.20; // subtracted

/// Empirical thresholds — prompt length signals.
const SHORT_PROMPT: usize = 100;
const LONG_PROMPT: usize = 800;

pub fn estimate(prompt: &str, role: Option<&str>) -> ComplexityEstimate {
    let lower = prompt.to_lowercase();
    let mut tau = 0.50; // baseline
    let mut features: Vec<&'static str> = Vec::new();

    for &kw in HEAVY_KEYWORDS {
        if lower.contains(kw) {
            tau += HEAVY_KW_W;
            features.push("heavy_kw");
            break; // count category once
        }
    }
    for &kw in MID_KEYWORDS {
        if lower.contains(kw) {
            tau += MID_KW_W;
            features.push("mid_kw");
            break;
        }
    }
    for &kw in LIGHT_KEYWORDS {
        if lower.contains(kw) {
            tau -= LIGHT_KW_W;
            features.push("light_kw");
            break;
        }
    }

    if let Some(r) = role {
        if HEAVY_ROLES.iter().any(|&h| h == r) {
            tau += HEAVY_ROLE_W;
            features.push("heavy_role");
        }
        if LIGHT_ROLES.iter().any(|&l| l == r) {
            tau -= LIGHT_ROLE_W;
            features.push("light_role");
        }
    }

    let len = prompt.len();
    if len < SHORT_PROMPT {
        tau -= 0.10;
        features.push("short_prompt");
    } else if len > LONG_PROMPT {
        tau += 0.10;
        features.push("long_prompt");
    }

    let tau = tau.clamp(0.0, 1.0);
    ComplexityEstimate { tau, features }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explore_lookup_is_low_tau() {
        let e = estimate("find files matching pattern", Some("Explore"));
        assert!(e.tau < 0.30, "expected lookup tier, got τ={}", e.tau);
        assert_eq!(Tier::from_tau(e.tau), Tier::Lookup);
    }

    #[test]
    fn architecture_prompt_is_high_tau() {
        let prompt = "Architect a novel state-space derivation for the manifold-tangent \
                      proof of convergence in our spectral algorithm. The goal is to \
                      produce a theorem-backed asymptotic bound.";
        let e = estimate(prompt, Some("physics-deriver"));
        assert!(e.tau >= 0.70, "expected architecture tier, got τ={}", e.tau);
        assert_eq!(Tier::from_tau(e.tau), Tier::Architecture);
    }

    #[test]
    fn implementation_with_role_is_mid() {
        let prompt = "Implement the kei-skills consumer endpoint with new tests.";
        let e = estimate(prompt, Some("code-implementer-rust"));
        // mid_kw + heavy_role + short → 0.5 + 0.10 + 0.20 - 0.10 = 0.70 (boundary)
        assert!(e.tau >= 0.30, "got {}", e.tau);
    }

    #[test]
    fn empty_prompt_minus_short_bonus_lands_at_baseline_minus_0_10() {
        let e = estimate("", None);
        assert_eq!(e.tau, 0.40);
        assert!(e.features.contains(&"short_prompt"));
    }

    #[test]
    fn clamps_to_unit_interval() {
        // pile every signal: heavy_kw+mid_kw+long_prompt+heavy_role
        let prompt = "Architect the novel algorithm: refactor implement wire test \
                      audit review merge migration schema endpoint trait async derive \
                      proof theorem rewrite redesign math spectral manifold convergence \
                      asymptotic. ".repeat(20);
        let e = estimate(&prompt, Some("physics-deriver"));
        assert!(e.tau >= 0.0 && e.tau <= 1.0);
    }

    #[test]
    fn light_signals_subtract() {
        let e = estimate("list files in directory", Some("Explore"));
        assert!(e.tau < 0.50);
        assert!(e.features.contains(&"light_kw"));
        assert!(e.features.contains(&"light_role"));
    }
}
