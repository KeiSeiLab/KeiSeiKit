use clap::{Parser, Subcommand};
use kei_social_store::graph::relationship_graph;
use kei_social_store::interactions::{log_interaction, Interaction};
use kei_social_store::people::{add_org, add_person, Organization, Person};
use kei_social_store::search::search_people;
use kei_social_store::Store;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-social-store", version)]
struct Cli {
    #[arg(long)] db: Option<PathBuf>,
    #[command(subcommand)] cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    SearchPeople { query: String, #[arg(long, default_value_t = 20)] limit: i64 },
    AddPerson { name: String,
                #[arg(long, default_value = "")] email: String,
                #[arg(long, default_value = "")] handle: String,
                #[arg(long, default_value = "manual")] source: String },
    AddOrg { name: String, #[arg(long, default_value = "company")] org_type: String },
    LogInteraction { person_id: i64, interaction_type: String,
                     #[arg(long, default_value = "")] content: String,
                     #[arg(long, default_value = "manual")] channel: String,
                     #[arg(long, default_value_t = 0)] target_id: i64 },
    RelationshipGraph,
}

fn db_path(o: Option<PathBuf>) -> PathBuf {
    if let Some(p) = o { return p; }
    if let Ok(e) = std::env::var("KEI_SOCIAL_DB") { return PathBuf::from(e); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/social/social.sqlite")
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let s = Store::open(&db_path(cli.db))?;
    dispatch(&s, cli.cmd)
}

fn dispatch(s: &Store, cmd: Cmd) -> anyhow::Result<()> {
    match cmd {
        Cmd::SearchPeople { query, limit } => cmd_search(s, &query, limit),
        Cmd::AddPerson { name, email, handle, source } =>
            cmd_add_person(s, name, email, handle, source),
        Cmd::AddOrg { name, org_type } => cmd_add_org(s, name, org_type),
        Cmd::LogInteraction { person_id, interaction_type, content, channel, target_id } =>
            cmd_log(s, person_id, target_id, interaction_type, channel, content),
        Cmd::RelationshipGraph => cmd_graph(s),
    }
}

fn cmd_search(s: &Store, query: &str, limit: i64) -> anyhow::Result<()> {
    for p in search_people(s, query, limit)? {
        println!("{}\t{}\t{}", p.id, p.name, p.email);
    }
    Ok(())
}

fn cmd_add_person(s: &Store, name: String, email: String,
                  handle: String, source: String) -> anyhow::Result<()> {
    let id = add_person(s, &Person { name, email, handle, source, ..Default::default() })?;
    println!("{}", id);
    Ok(())
}

fn cmd_add_org(s: &Store, name: String, org_type: String) -> anyhow::Result<()> {
    let id = add_org(s, &Organization { name, org_type, ..Default::default() })?;
    println!("{}", id);
    Ok(())
}

fn cmd_log(s: &Store, person_id: i64, target_id: i64, interaction_type: String,
           channel: String, content: String) -> anyhow::Result<()> {
    let id = log_interaction(s, &Interaction {
        person_id, target_id, interaction_type, channel, content,
        ..Default::default()
    })?;
    println!("{}", id);
    Ok(())
}

fn cmd_graph(s: &Store) -> anyhow::Result<()> {
    for p in relationship_graph(s)? {
        println!("{}\t-[{}]->\t{}\t({}x)",
            p.person_id, p.channel, p.target_id, p.count);
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("kei-social-store: {e:#}"); ExitCode::from(1) }
    }
}
