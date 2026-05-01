//! kei-router CLI — print routed tool-call as JSON.

use clap::Parser;
use kei_router::Router;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-router", version, about = "Route NL query → tool-call JSON")]
struct Cli {
    /// The natural-language query.
    query: String,
    /// Hint remote-MCP forwarding on fallback (adds _forward=true).
    #[arg(long)]
    forward: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let router = Router::new();
    let result = if cli.forward {
        router.route_with_hint(&cli.query)
    } else {
        router.route(&cli.query)
    };
    match serde_json::to_string_pretty(&result) {
        Ok(s) => {
            println!("{}", s);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kei-router: json encode failed: {e}");
            ExitCode::from(1)
        }
    }
}
