//! Category seed table — one file, one responsibility (the "model").
//!
//! Extend by editing ONLY this file. Each `Category` is a flat struct of
//! static metadata + uncompiled regex sources. `compile_all()` is the sole
//! factory: it walks the seed table once and returns compiled regexes
//! paired with the metadata. Callers never touch raw strings — they get a
//! `Vec<CompiledCategory>` and match against it.
//!
//! Design:
//! * uncompiled `triggers` live as `&'static str` so they can be verified
//!   at test time without any allocator;
//! * compiled regexes use `(?i)` prefix so all matching is case-insensitive,
//!   which matters because user pushback appears in both Russian and English;
//! * every regex is compiled once at startup — never per-line.
//!
//! Seed rationale (from the 22k-token Explore audit that motivated this CLI):
//! 1. `conservative-framing` and `paradigm-slippage` were the two recurring
//!    classes the audit found.
//! 2. `data-contamination` was a one-off in the audit but is high-priority
//!    for any pre-registered experiment work (see RULE 0.8 pre-registration).
//! 3. `repeat-signal` is the strongest marker per Karpathy "Think Before
//!    Coding" — user literally saying "again".
//! 4. `frustration-tone` is the base-rate surface signal.

use regex::Regex;

/// Metadata for one frustration class (raw, un-compiled).
pub struct Category {
    /// Short slug — machine id, used in CSV/JSONL output.
    pub id: &'static str,
    /// Human-readable name — used in report tables.
    pub display: &'static str,
    /// Uncompiled regex sources; compiled once at startup.
    pub triggers: &'static [&'static str],
    /// Severity multiplier — weighted score = count * weight.
    pub weight: f64,
    /// Free-text context note for the reader (not matched).
    pub context_hint: &'static str,
}

/// Compiled counterpart — regex list + preserved metadata.
///
/// `display` and `context_hint` are reserved for the richer report format
/// we will add once the 5 seed categories prove their value on real data.
/// They are kept at the type level now to stabilize the struct layout.
pub struct CompiledCategory {
    pub id: &'static str,
    #[allow(dead_code)]
    pub display: &'static str,
    pub weight: f64,
    #[allow(dead_code)]
    pub context_hint: &'static str,
    pub patterns: Vec<Regex>,
}

/// The seed table — 5 categories. Order defines tie-break order in reports.
pub const CATEGORIES: &[Category] = &[
    Category {
        id: "conservative-framing",
        display: "Conservative framing (RULE -1)",
        triggers: &[
            r"не\s+10\s*%",
            r"это\s+(всё|все|всё\s+что)\s+(мы|ты)",
            r"только\s+(\d+\s*%|немного|чуть)",
            r"\blimitation\b",
            r"\bdowngrade\b",
            r"refuted\s+(finally|for\s+good|окончательно)",
            r"\baccept\s+as\b",
            r"провалил(ся|ась|ось)",
            r"не\s+сработал",
        ],
        weight: 2.0,
        context_hint: "Dominant when previous assistant msg used 'failed/refuted'",
    },
    Category {
        id: "paradigm-slippage",
        display: "Paradigm slippage (dark-matter / SM gap)",
        triggers: &[
            r"тёмн(ая|ую|ой)\s+матери",
            r"dark\s+matter.{0,40}explain",
            r"missing\s+(mass|matter)",
            r"standard\s+model.{0,40}gap",
            r"fill\s+(a|the)\s+gap",
        ],
        weight: 1.5,
        context_hint: "User rejects DM/SM-gap framing; as ill-posed",
    },
    Category {
        id: "data-contamination",
        display: "Data contamination (pre-reg / stratification)",
        triggers: &[
            r"грязн(ые|ых)\s+данн",
            r"контролирова(но|нн(ые|ых))",
            r"pooled?.{0,20}(unstratified|without\s+stratif)",
            r"\blumping\b",
            r"коверкать",
            r"\bmassaging\b",
            r"cherry[\s-]?pick",
        ],
        weight: 1.5,
        context_hint: "Violation of RULE 0.8 pre-registration / paradigm-native",
    },
    Category {
        id: "repeat-signal",
        display: "Repeat signal (user explicitly says 'again')",
        triggers: &[
            r"\bопять\b",
            r"\bagain\b",
            r"уже\s+(говорил|спрашивал|просил|сказал)",
            r"\bsecond\s+time\b",
            r"\bthird\s+time\b",
            r"\bповторяю\b",
            r"я\s+же\s+(просил|сказал|говорил)",
        ],
        weight: 2.5,
        context_hint: "Strongest marker — direct RULE 0.10 recurrence-escalate trigger",
    },
    Category {
        id: "frustration-tone",
        display: "Frustration tone (surface anger)",
        triggers: &[
            r"\bстоп\b",
            r"\bstop\b",
            r"\bхватит\b",
            r"\bнахуй\b",
            r"\bнет[- ]?нет\b",
            r"не\s+понял",
            r"\bты\s+что\b",
            r"\bзачем\s+ты\b",
            r"\bпочему\s+ты\b",
            r"\bwhy\s+did\s+you\b",
            r"\bблин\b",
            r"\bкуда\b.{0,10}\b(ты|полез)\b",
        ],
        weight: 1.0,
        context_hint: "Base-rate surface signal; interpret with context",
    },
];

/// Compile every trigger in every category. Called once from `main` / tests.
///
/// Panics at startup if any regex is malformed — this is intentional,
/// because a malformed seed is a developer bug, not a runtime condition.
pub fn compile_all() -> Vec<CompiledCategory> {
    CATEGORIES.iter().map(compile_one).collect()
}

fn compile_one(c: &'static Category) -> CompiledCategory {
    let patterns = c
        .triggers
        .iter()
        .map(|src| compile_ci(c.id, src))
        .collect();
    CompiledCategory {
        id: c.id,
        display: c.display,
        weight: c.weight,
        context_hint: c.context_hint,
        patterns,
    }
}

fn compile_ci(cat_id: &str, src: &str) -> Regex {
    let wrapped = format!("(?i){src}");
    Regex::new(&wrapped)
        .unwrap_or_else(|e| panic!("category {cat_id}: regex {src:?}: {e}"))
}
