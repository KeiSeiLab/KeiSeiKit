use clap::{Parser, Subcommand};
use kei_content_store::assets::{register_asset, Asset};
use kei_content_store::campaigns::{attach_asset, create_campaign};
use kei_content_store::prompts::{history, register_prompt, Prompt};
use kei_content_store::Store;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-content-store", version)]
struct Cli {
    #[arg(long)] db: Option<PathBuf>,
    #[command(subcommand)] cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    RegisterAsset { title: String,
                    #[arg(long, default_value = "")] file_path: String,
                    #[arg(long, default_value = "")] media_type: String,
                    #[arg(long, default_value = "")] provider: String },
    RegisterPrompt { prompt_text: String,
                     #[arg(long, default_value = "")] model: String,
                     #[arg(long, default_value = "")] prompt_type: String },
    CreateCampaign { name: String, #[arg(long, default_value = "")] description: String },
    AttachAsset { campaign_id: i64, asset_id: i64 },
    PromptHistory { prompt_id: i64 },
}

fn db_path(o: Option<PathBuf>) -> PathBuf {
    if let Some(p) = o { return p; }
    if let Ok(e) = std::env::var("KEI_CONTENT_DB") { return PathBuf::from(e); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/content/content.sqlite")
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let s = Store::open(&db_path(cli.db))?;
    dispatch(&s, cli.cmd)
}

fn dispatch(s: &Store, cmd: Cmd) -> anyhow::Result<()> {
    match cmd {
        Cmd::RegisterAsset { title, file_path, media_type, provider } =>
            cmd_asset(s, title, file_path, media_type, provider),
        Cmd::RegisterPrompt { prompt_text, model, prompt_type } =>
            cmd_prompt(s, prompt_text, model, prompt_type),
        Cmd::CreateCampaign { name, description } => cmd_campaign(s, &name, &description),
        Cmd::AttachAsset { campaign_id, asset_id } =>
            cmd_attach(s, campaign_id, asset_id),
        Cmd::PromptHistory { prompt_id } => cmd_history(s, prompt_id),
    }
}

fn cmd_asset(s: &Store, title: String, file_path: String,
             media_type: String, provider: String) -> anyhow::Result<()> {
    let id = register_asset(s, &Asset {
        title, file_path, media_type, provider,
        unit_type: "asset".into(), ..Default::default()
    })?;
    println!("{}", id);
    Ok(())
}

fn cmd_prompt(s: &Store, prompt_text: String, model: String,
              prompt_type: String) -> anyhow::Result<()> {
    let id = register_prompt(s, &Prompt {
        prompt_text, model, prompt_type, ..Default::default()
    })?;
    println!("{}", id);
    Ok(())
}

fn cmd_campaign(s: &Store, name: &str, description: &str) -> anyhow::Result<()> {
    let id = create_campaign(s, name, description)?;
    println!("{}", id);
    Ok(())
}

fn cmd_attach(s: &Store, campaign_id: i64, asset_id: i64) -> anyhow::Result<()> {
    attach_asset(s, campaign_id, asset_id)?;
    println!("attached {} to campaign {}", asset_id, campaign_id);
    Ok(())
}

fn cmd_history(s: &Store, prompt_id: i64) -> anyhow::Result<()> {
    for p in history(s, prompt_id)? {
        println!("{}\t{}\t{}", p.id, p.version, p.prompt_text);
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("kei-content-store: {e:#}"); ExitCode::from(1) }
    }
}
