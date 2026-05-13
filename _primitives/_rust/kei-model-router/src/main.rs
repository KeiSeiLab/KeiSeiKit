//! kei-model-router CLI — model selection for Claude Code Agent spawns.
//! Subcommands: pricing | select <agent> [--prompt P] | calibrate | --help

use kei_model_router::{
    calibrate, pick, select, DecisionInput, KernelWeights, Model, Registry,
    OPUS_47_TOKENIZER_OVERHEAD,
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
    print!(concat!(
        "kei-model-router — model selection for Claude Code Agent spawns\n\n",
        "Usage:\n",
        "  kei-model-router [pricing]              print pricing table from models.toml\n",
        "  kei-model-router select <agent> [--prompt P]\n",
        "  kei-model-router calibrate              re-fit kernel weights\n",
        "  kei-model-router --help\n\n",
        "Env:\n",
        "  KEI_LEDGER_DB          override ledger path\n",
        "  KEI_REGISTRIES_DIR     override registries dir\n",
    ));
}

fn print_pricing() {
    let reg = match Registry::load() {
        Ok(r) => r,
        Err(e) => { eprintln!("registry load error: {e}"); std::process::exit(1); }
    };
    println!("kei-model-router — pricing from models.toml\n");
    println!("{:<30} {:>12} {:>12} {:>12}", "model", "input/M", "output/M", "cache_r");
    for m in &reg.models {
        println!(
            "{:<30} {:>12} {:>12} {:>12}",
            m.id,
            fmt_micro(m.cost_input_per_mtok_micro),
            fmt_micro(m.cost_output_per_mtok_micro),
            fmt_micro(m.cache_read_per_mtok_micro),
        );
    }
    println!("\nNote: Opus 4.7 tokenizer may use up to {:.0}% more tokens vs Sonnet/Haiku.",
        (OPUS_47_TOKENIZER_OVERHEAD - 1.0) * 100.0);
}

fn cmd_select(args: &[String]) {
    let agent = args.first().unwrap_or_else(|| {
        eprintln!("usage: kei-model-router select <agent> [--prompt PROMPT]");
        std::process::exit(2);
    });
    let prompt = parse_prompt_flag(args);
    let dna = format!("{agent}::?::00000000::00000000-00000000");
    let (input, non_claude) = build_select_input(agent, &prompt, &dna);

    if let Some((prov, model_id)) = non_claude {
        print_non_claude(agent, &prov, &model_id);
        return;
    }

    let conn = match open_ledger() {
        Some(c) => c,
        None => {
            eprintln!("warning: ledger not available; falling back to default");
            print_decision_no_ledger(&input, &dna);
            return;
        }
    };

    let d = match select(&input, &conn) {
        Ok(d) => d,
        Err(e) => { eprintln!("ledger query failed: {e}"); std::process::exit(1); }
    };
    print_claude_decision(agent, &d);
}

fn print_non_claude(agent: &str, prov: &str, model_id: &str) {
    println!("agent:        {agent}");
    println!("provider:     {prov}");
    println!("model:        {model_id}");
    println!("reason:       profile_default_non_claude");
}

fn print_claude_decision(agent: &str, d: &kei_model_router::Decision) {
    println!("agent:        {agent}");
    println!("model:        {}", d.model.slug());
    println!("expected_cost ${:.4} (microcents={})",
        d.expected_cost_micro_cents as f64 / 100_000_000.0, d.expected_cost_micro_cents);
    println!("q_lower_bound {:.3} (posterior n={})", d.quality_lower_bound, d.posterior_n);
    println!("reason:       {}", d.reason);
}

/// Build DecisionInput; FIX NEW-1: returns non-Claude (prov, model_id) when
/// profile resolves to a non-Anthropic model, so caller bypasses posterior.
fn build_select_input(
    agent: &str,
    prompt: &str,
    dna: &str,
) -> (DecisionInput, Option<(String, String)>) {
    let mut input = DecisionInput::new(dna.to_string(), prompt.to_string());
    input.kernel_weights = KernelWeights::default();
    input.pinned = read_pinned_for_agent(agent);

    let non_claude = if let Ok(reg) = Registry::load() {
        match pick(agent, &reg) {
            Some((prov, model_id)) => match Model::from_slug(&model_id) {
                Some(m) => { input.fallback = m; None }
                None => Some((prov, model_id)),
            },
            None => None,
        }
    } else {
        None
    };
    (input, non_claude)
}

fn parse_prompt_flag(args: &[String]) -> String {
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--prompt" {
            if let Some(p) = args.get(i + 1) { return p.clone(); }
        }
        i += 1;
    }
    String::new()
}

/// FIX NEW-2: takes &DecisionInput so the profile-resolved fallback survives.
fn print_decision_no_ledger(input: &DecisionInput, dna: &str) {
    let est = kei_model_router::complexity::estimate(
        &input.prompt, kei_model_router::dna_class::role(dna));
    println!("model:     {}\nτ:         {:.2}\nreason:    no_ledger_fallback",
        input.fallback.slug(), est.tau);
}

fn cmd_calibrate() {
    let conn = match open_ledger() {
        Some(c) => c,
        None => { eprintln!("ledger not found; aborting calibration"); std::process::exit(1); }
    };
    let r = match calibrate::calibrate(&conn) {
        Ok(r) => r,
        Err(e) => { eprintln!("calibration query failed: {e}"); std::process::exit(1); }
    };
    println!("rows evaluated: {}", r.rows_evaluated);
    if r.rows_evaluated < 5 {
        println!("(too few rows for calibration; using default weights)");
        return;
    }
    println!("baseline MSE:   {:.4}\nbest MSE:       {:.4}\nimprovement:    {:.4}",
        r.baseline_mse, r.best_mse, r.baseline_mse - r.best_mse);
    println!("calibrated weights:\n  alpha_role:  {:.2}\n  alpha_caps:  {:.2}\n  alpha_scope: {:.2}\n  alpha_body:  {:.2}",
        r.best_weights.alpha_role, r.best_weights.alpha_caps,
        r.best_weights.alpha_scope, r.best_weights.alpha_body);
}

fn open_ledger() -> Option<Connection> {
    let path = if let Ok(p) = std::env::var("KEI_LEDGER_DB") {
        p
    } else {
        let home = std::env::var("HOME").unwrap_or_default();
        if home.is_empty() {
            eprintln!("[kei-model-router] HOME unset; cannot resolve ledger path");
            return None;
        }
        format!("{home}/.claude/agents/ledger.sqlite")
    };
    let conn = Connection::open(&path).ok()?;
    conn.pragma_update(None, "journal_mode", "WAL").ok();
    conn.busy_timeout(std::time::Duration::from_secs(5)).ok();
    Some(conn)
}

fn read_pinned_for_agent(agent: &str) -> Option<Model> {
    let home = std::env::var("HOME").ok()?;
    let raw = std::fs::read_to_string(format!("{home}/.claude/settings.json")).ok()?;
    let json: serde_json::Value = serde_json::from_str(&raw).ok()?;
    let model_slug = json.get("router")?.get("pinned")?.get(agent)?.as_str()?;
    Model::from_slug(model_slug)
}

fn fmt_micro(uc: u64) -> String {
    format!("${:.2}", uc as f64 / 100_000_000.0)
}
