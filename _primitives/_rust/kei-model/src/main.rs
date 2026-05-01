//! kei-model CLI entry. Dispatches to one handler per subcommand. Each
//! handler stays ≤30 LOC by delegating to library functions.
//!
//! Exit codes:
//!   0 — success
//!   1 — file/IO error
//!   2 — not-found / no-match / unknown id
//!   3 — cycle in fallback chain

use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

use kei_model::cli::{Cli, Cmd, FallbackArgs, ListArgs, PriceArgs, ProvidersArgs, ResolveArgs};
use kei_model::model::{Capability, Model, Provider, Status};
use kei_model::pricing::{estimate, PricingStatus};
use kei_model::registry::Registry;
use kei_model::selector::{resolve, resolve_selectors_path};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match dispatch(cli.cmd) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kei-model: {e}");
            ExitCode::from(map_exit_code(&e))
        }
    }
}

fn dispatch(cmd: Cmd) -> anyhow::Result<()> {
    match cmd {
        Cmd::List(a) => cmd_list(a),
        Cmd::Resolve(a) => cmd_resolve(a),
        Cmd::Price(a) => cmd_price(a),
        Cmd::Providers(a) => cmd_providers(a),
        Cmd::Fallback(a) => cmd_fallback(a),
    }
}

fn cmd_list(a: ListArgs) -> anyhow::Result<()> {
    let path = Registry::resolve_path(a.models_toml.as_deref())?;
    let reg = Registry::load(&path)?;
    let kept = filter_list(reg.list_all(), &a)?;
    let json = serde_json::to_string_pretty(&kept)?;
    println!("{json}");
    Ok(())
}

fn filter_list<'a>(all: &'a [Model], a: &ListArgs) -> anyhow::Result<Vec<&'a Model>> {
    let mut out: Vec<&Model> = all.iter().collect();
    if let Some(p) = &a.provider {
        let pv = parse_provider(p)?;
        out.retain(|m| m.provider == pv);
    }
    if let Some(c) = &a.cap {
        let cv = parse_cap(c)?;
        out.retain(|m| m.capabilities.contains(&cv));
    }
    if let Some(s) = &a.status {
        let sv = parse_status(s)?;
        out.retain(|m| m.status == sv);
    }
    if let Some(r) = &a.role {
        out.retain(|m| m.has_role(r));
    }
    Ok(out)
}

fn cmd_resolve(a: ResolveArgs) -> anyhow::Result<()> {
    let path = Registry::resolve_path(a.models_toml.as_deref())?;
    let reg = Registry::load(&path)?;
    let caps = parse_caps_csv(a.cap.as_deref())?;
    let sel_path: Option<PathBuf> = a.selectors_toml;
    let sel_ref: Option<&std::path::Path> = sel_path.as_deref();
    let r = resolve(&a.role, a.budget_micro, &caps, &reg, sel_ref)?;
    print_resolution(&r);
    Ok(())
}

fn print_resolution(r: &kei_model::selector::Resolution) {
    let body = serde_json::json!({
        "model_id": r.model.id,
        "provider": r.model.provider.as_str(),
        "pricing": r.model.pricing,
        "reason": r.reason,
    });
    println!("{}", serde_json::to_string_pretty(&body).unwrap());
}

fn cmd_price(a: PriceArgs) -> anyhow::Result<()> {
    let path = Registry::resolve_path(a.models_toml.as_deref())?;
    let reg = Registry::load(&path)?;
    let m = reg
        .get(&a.model_id)
        .ok_or_else(|| anyhow::anyhow!("unknown model_id: {}", a.model_id))?;
    let micro = estimate(&m.pricing, a.input_tokens, a.output_tokens);
    print_price(&a.model_id, micro, m.pricing.status);
    Ok(())
}

fn print_price(model_id: &str, micro: u64, status: PricingStatus) {
    let display_cents = (micro as f64) / 1_000_000.0;
    let body = serde_json::json!({
        "model_id": model_id,
        "micro_cents": micro,
        "display_cents": format!("{display_cents:.6}"),
        "pricing_status": status.as_str(),
    });
    println!("{}", serde_json::to_string_pretty(&body).unwrap());
}

fn cmd_providers(a: ProvidersArgs) -> anyhow::Result<()> {
    let path = Registry::resolve_path(a.models_toml.as_deref())?;
    let reg = Registry::load(&path)?;
    let summary = build_provider_summary(&reg);
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
    Ok(())
}

fn build_provider_summary(reg: &Registry) -> serde_json::Value {
    let providers = [
        Provider::Anthropic,
        Provider::Openai,
        Provider::Kimi,
        Provider::Mistral,
        Provider::Deepseek,
        Provider::Local,
    ];
    let mut rows: Vec<serde_json::Value> = Vec::new();
    for p in providers {
        let by_p = reg.by_provider(p);
        if by_p.is_empty() {
            continue;
        }
        rows.push(serde_json::json!({
            "name": p.as_str(),
            "active_count": by_p.iter().filter(|m| m.status == Status::Active).count(),
            "deprecated_count": by_p.iter().filter(|m| m.status == Status::Deprecated).count(),
        }));
    }
    serde_json::json!({ "providers": rows })
}

fn cmd_fallback(a: FallbackArgs) -> anyhow::Result<()> {
    let path = Registry::resolve_path(a.models_toml.as_deref())?;
    let reg = Registry::load(&path)?;
    let chain = kei_model::chain(&a.primary, &reg)?;
    println!("{}", serde_json::to_string_pretty(&chain)?);
    Ok(())
}

fn parse_provider(s: &str) -> anyhow::Result<Provider> {
    Provider::parse(s).ok_or_else(|| anyhow::anyhow!("unknown provider: {s}"))
}

fn parse_cap(s: &str) -> anyhow::Result<Capability> {
    Capability::parse(s).ok_or_else(|| anyhow::anyhow!("unknown capability: {s}"))
}

fn parse_status(s: &str) -> anyhow::Result<Status> {
    Status::parse(s).ok_or_else(|| anyhow::anyhow!("unknown status: {s}"))
}

fn parse_caps_csv(s: Option<&str>) -> anyhow::Result<Vec<Capability>> {
    let raw = match s {
        None => return Ok(Vec::new()),
        Some(x) if x.is_empty() => return Ok(Vec::new()),
        Some(x) => x,
    };
    raw.split(',').map(|t| parse_cap(t.trim())).collect()
}

fn map_exit_code(e: &anyhow::Error) -> u8 {
    let msg = e.to_string();
    if msg.starts_with("cycle in fallback chain") {
        return 3;
    }
    if msg.starts_with("unknown model_id")
        || msg.starts_with("unknown primary model_id")
        || msg.starts_with("no active model matches")
    {
        return 2;
    }
    1
}

// resolve_selectors_path is re-exported for tests; mark it used here so the
// linter knows it's part of the public surface even when main doesn't call
// it directly today.
#[allow(dead_code)]
fn _reexport_anchor() {
    let _ = resolve_selectors_path;
}
