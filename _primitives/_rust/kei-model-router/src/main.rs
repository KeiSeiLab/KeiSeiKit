//! kei-model-router CLI.
//!
//! Subcommands:
//!   pricing                — print verified pricing table (default)
//!   select <agent> [--prompt P]
//!                          — query router decision for given agent
//!                            spawn. Reads ledger at $KEI_LEDGER_DB.
//!   calibrate              — re-fit kernel weights against ledger
//!                            outcomes. Print baseline vs best MSE.
//!   --help

use kei_model_router::{
    calibrate, select, DecisionInput, KernelWeights, Model, OPUS_47_TOKENIZER_OVERHEAD,
};
use rusqlite::Connection;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("pricing") | None => print_pricing(),
        Some("select") => cmd_select(&args[2..]),
        Some("calibrate") => cmd_calibrate(),
        Some("--help") | Some("-h") => print_help(),
        Some(other) => {
            eprintln!("unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!("kei-model-router — model selection for Claude Code Agent spawns");
    println!();
    println!("Usage:");
    println!("  kei-model-router [pricing]          print verified pricing table");
    println!("  kei-model-router select <agent> [--prompt P]");
    println!("                                      route a synthetic spawn");
    println!("  kei-model-router calibrate          re-fit kernel weights");
    println!("  kei-model-router --help");
    println!();
    println!("Env:");
    println!("  KEI_LEDGER_DB                      override ledger path");
    println!("                                     (default: ~/.claude/agents/ledger.sqlite)");
}

fn print_pricing() {
    println!("kei-model-router — verified Claude API pricing (microcents per 1M tokens)");
    println!("Source: https://platform.claude.com/docs/en/docs/about-claude/pricing");
    println!("Verified: 2026-04-30 (RULE 0.4)");
    println!();
    println!(
        "{:<10} {:>12} {:>12} {:>12} {:>12}",
        "model", "input/M", "output/M", "cache_w_5m", "cache_r"
    );
    for m in Model::all() {
        let p = m.pricing();
        println!(
            "{:<10} {:>12} {:>12} {:>12} {:>12}",
            m.slug(),
            fmt_microcents(p.input_micro_cents_per_mtok),
            fmt_microcents(p.output_micro_cents_per_mtok),
            fmt_microcents(p.cache_write_5m_micro_cents_per_mtok),
            fmt_microcents(p.cache_read_micro_cents_per_mtok),
        );
    }
    println!();
    println!(
        "Note: Opus 4.7 tokenizer may use up to {:.0}% more tokens",
        (OPUS_47_TOKENIZER_OVERHEAD - 1.0) * 100.0
    );
    println!("on identical text vs Sonnet/Haiku; multiply Opus quote accordingly.");
}

fn cmd_select(args: &[String]) {
    let agent = match args.first() {
        Some(a) => a,
        None => {
            eprintln!("usage: kei-model-router select <agent> [--prompt PROMPT]");
            std::process::exit(2);
        }
    };
    let mut prompt = String::new();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--prompt" {
            if let Some(p) = args.get(i + 1) {
                prompt = p.clone();
                i += 2;
                continue;
            }
        }
        i += 1;
    }

    // Synthesize a DNA from agent name. Real spawns get DNA from
    // agent-fork-logger.sh via kei-shared::compose_dna; this CLI uses
    // a stable synthetic so users can probe without a real spawn.
    let synthetic_dna = format!("{agent}::?::00000000::00000000-00000000");

    let conn = match open_ledger() {
        Some(c) => c,
        None => {
            eprintln!("warning: ledger not available; falling back to default");
            print_decision_no_ledger(&synthetic_dna, &prompt);
            return;
        }
    };

    let mut input = DecisionInput::new(synthetic_dna.clone(), prompt);
    input.kernel_weights = KernelWeights::default();
    input.pinned = read_pinned_for_agent(agent);
    let decision = match select(&input, &conn) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("ledger query failed: {e}");
            std::process::exit(1);
        }
    };

    println!("agent:        {agent}");
    println!("dna:          {synthetic_dna}");
    println!("model:        {}", decision.model.slug());
    println!(
        "expected_cost ${:.4} (microcents={})",
        decision.expected_cost_micro_cents as f64 / 100_000_000.0,
        decision.expected_cost_micro_cents
    );
    println!(
        "q_lower_bound {:.3} (posterior n={})",
        decision.quality_lower_bound, decision.posterior_n
    );
    println!(
        "complexity τ={:.2} ({:?} signals)",
        decision.complexity.tau, decision.complexity.features
    );
    println!("reason:       {}", decision.reason);
}

fn print_decision_no_ledger(dna: &str, prompt: &str) {
    let inp = DecisionInput::new(dna.to_string(), prompt.to_string());
    let est = kei_model_router::complexity::estimate(prompt, kei_model_router::dna_class::role(dna));
    println!("model:     {}", inp.fallback.slug());
    println!("τ:         {:.2}", est.tau);
    println!("reason:    no_ledger_fallback");
}

fn cmd_calibrate() {
    let conn = match open_ledger() {
        Some(c) => c,
        None => {
            eprintln!("ledger not found; aborting calibration");
            std::process::exit(1);
        }
    };
    let result = match calibrate::calibrate(&conn) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("calibration query failed: {e}");
            std::process::exit(1);
        }
    };
    println!("rows evaluated: {}", result.rows_evaluated);
    if result.rows_evaluated < 5 {
        println!("(too few rows for calibration; using default weights)");
        return;
    }
    println!("baseline MSE:   {:.4}", result.baseline_mse);
    println!("best MSE:       {:.4}", result.best_mse);
    println!(
        "improvement:    {:.4}",
        result.baseline_mse - result.best_mse
    );
    println!();
    println!("calibrated weights:");
    println!("  alpha_role:  {:.2}", result.best_weights.alpha_role);
    println!("  alpha_caps:  {:.2}", result.best_weights.alpha_caps);
    println!("  alpha_scope: {:.2}", result.best_weights.alpha_scope);
    println!("  alpha_body:  {:.2}", result.best_weights.alpha_body);
}

fn open_ledger() -> Option<Connection> {
    let path = std::env::var("KEI_LEDGER_DB").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{home}/.claude/agents/ledger.sqlite")
    });
    Connection::open(&path).ok()
}

/// Read `~/.claude/settings.json::router.pinned[agent]` if present.
/// Returns Some(Model) for agents the user has pinned to a specific tier.
/// Examples: "Explore" → Haiku, "ml-implementer" → Opus.
fn read_pinned_for_agent(agent: &str) -> Option<Model> {
    let home = std::env::var("HOME").ok()?;
    let path = format!("{home}/.claude/settings.json");
    let raw = std::fs::read_to_string(&path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&raw).ok()?;
    let pinned = json.get("router")?.get("pinned")?;
    let model_slug = pinned.get(agent)?.as_str()?;
    Model::from_slug(model_slug)
}

fn fmt_microcents(uc: u64) -> String {
    let dollars = uc as f64 / 100_000_000.0;
    format!("${:.2}", dollars)
}
