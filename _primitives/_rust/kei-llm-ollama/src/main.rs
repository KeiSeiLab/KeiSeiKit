//! kei-llm-ollama CLI entry — clap parse + dispatch + exit-code mapping.

use clap::Parser;

use kei_llm_ollama::cli::{Cli, Cmd};
use kei_llm_ollama::error::ApiError;
use kei_llm_ollama::handlers;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    let result: Result<(), ApiError> = match &cli.command {
        Cmd::Tags(o) => handlers::run_tags(o).await,
        Cmd::Generate(o) => handlers::run_generate(o).await,
        Cmd::Chat(o) => handlers::run_chat(o).await,
        Cmd::Pull(o) => handlers::run_pull(o).await,
        Cmd::Health(o) => handlers::run_health(o).await,
    };
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kei-llm-ollama: {e}");
            std::process::ExitCode::from(e.exit_code() as u8)
        }
    }
}
