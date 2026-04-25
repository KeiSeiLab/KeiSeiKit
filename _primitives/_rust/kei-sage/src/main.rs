//! kei-sage CLI — import / search / related / rank / add / edit.

use clap::{Parser, Subcommand};
use kei_sage::atom_cli::{
    cmd_atoms_discover, cmd_atoms_rank, cmd_atoms_related, cmd_atoms_search, cmd_author,
    cmd_facet_query, cmd_lineage, cmd_rules_discover, default_atoms_root,
    default_capabilities_root, default_manifests_root, default_roles_root, default_rules_root,
};
use kei_sage::bfs::bfs;
use kei_sage::edges::add_edge;
use kei_sage::import::import_vault;
use kei_sage::pagerank::pagerank;
use kei_sage::search::fts_search;
use kei_sage::{Store, Unit};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-sage", version, about = "Obsidian-style knowledge vault")]
struct Cli {
    /// Database path (default: $KEI_VAULT_DB or ~/.claude/sage/vault.sqlite)
    #[arg(long)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Import { vault: PathBuf },
    Search { query: String, #[arg(long, default_value_t = 20)] limit: i64 },
    Related { key: String, #[arg(long, default_value_t = 2)] depth: i64 },
    Rank { #[arg(long, default_value_t = 20)] limit: usize },
    Add {
        #[arg(long)] title: String,
        #[arg(long, default_value = "")] content: String,
        #[arg(long, default_value = "")] vault_path: String,
        #[arg(long, default_value = "E4")] grade: String,
    },
    Edit {
        id: i64,
        #[arg(long)] title: Option<String>,
        #[arg(long)] content: Option<String>,
        #[arg(long)] grade: Option<String>,
    },
    Link { src: String, dst: String, #[arg(long, default_value = "related")] edge_type: String },
    AtomsDiscover {
        #[arg(long)] root: Option<PathBuf>,
    },
    AtomsRank {
        #[arg(long)] root: Option<PathBuf>,
        #[arg(long, default_value_t = 20)] limit: usize,
    },
    AtomsRelated {
        atom_id: String,
        #[arg(long)] root: Option<PathBuf>,
        #[arg(long, default_value_t = 2)] depth: i64,
    },
    AtomsSearch {
        query: String,
        #[arg(long)] root: Option<PathBuf>,
        #[arg(long, default_value_t = 20)] limit: i64,
    },
    AtomsRulesDiscover {
        #[arg(long)] rules_root: Option<PathBuf>,
    },
    FacetQuery {
        filters: Vec<String>,
        #[arg(long)] capabilities_root: Option<PathBuf>,
        #[arg(long)] manifests_root: Option<PathBuf>, #[arg(long)] roles_root: Option<PathBuf>,
    },
    Lineage {
        id: String,
        #[arg(long, default_value_t = 3)] depth: usize,
        #[arg(long)] capabilities_root: Option<PathBuf>,
        #[arg(long)] manifests_root: Option<PathBuf>,
    },
    Author {
        creator: String,
        #[arg(long, default_value_t = 50)] limit: usize,
        #[arg(long)] capabilities_root: Option<PathBuf>,
        #[arg(long)] manifests_root: Option<PathBuf>,
    },
}

fn db_path(cli_db: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_db { return p; }
    if let Ok(e) = std::env::var("KEI_VAULT_DB") { return PathBuf::from(e); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/sage/vault.sqlite")
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let store = Store::open(&db_path(cli.db))?;
    dispatch(&store, cli.cmd)
}

fn dispatch(store: &Store, cmd: Cmd) -> anyhow::Result<()> {
    match cmd {
        Cmd::Import { vault } => cmd_import(store, &vault),
        Cmd::Search { query, limit } => cmd_search(store, &query, limit),
        Cmd::Related { key, depth } => cmd_related(store, &key, depth),
        Cmd::Rank { limit } => cmd_rank(store, limit),
        Cmd::Add { title, content, vault_path, grade } =>
            cmd_add(store, title, content, vault_path, grade),
        Cmd::Edit { id, title, content, grade } =>
            cmd_edit(store, id, title, content, grade),
        Cmd::Link { src, dst, edge_type } => cmd_link(store, &src, &dst, &edge_type),
        Cmd::AtomsDiscover { root } =>
            cmd_atoms_discover(&root.unwrap_or_else(default_atoms_root)),
        Cmd::AtomsRank { root, limit } =>
            cmd_atoms_rank(store, &root.unwrap_or_else(default_atoms_root), limit),
        Cmd::AtomsRelated { atom_id, root, depth } =>
            cmd_atoms_related(store, &root.unwrap_or_else(default_atoms_root), &atom_id, depth),
        Cmd::AtomsSearch { query, root, limit } =>
            cmd_atoms_search(store, &root.unwrap_or_else(default_atoms_root), &query, limit),
        Cmd::AtomsRulesDiscover { rules_root } =>
            cmd_rules_discover(&rules_root.unwrap_or_else(default_rules_root)),
        Cmd::FacetQuery { filters, capabilities_root, manifests_root, roles_root } => {
            let (c, m) = prim_roots(capabilities_root, manifests_root);
            cmd_facet_query(&c, &m, &roles_root.unwrap_or_else(default_roles_root), &filters)
        }
        Cmd::Lineage { id, depth, capabilities_root, manifests_root } => {
            let (c, m) = prim_roots(capabilities_root, manifests_root);
            cmd_lineage(&c, &m, &id, depth)
        }
        Cmd::Author { creator, limit, capabilities_root, manifests_root } => {
            let (c, m) = prim_roots(capabilities_root, manifests_root);
            cmd_author(&c, &m, &creator, limit)
        }
    }
}

fn prim_roots(c: Option<PathBuf>, m: Option<PathBuf>) -> (PathBuf, PathBuf) {
    (c.unwrap_or_else(default_capabilities_root),
     m.unwrap_or_else(default_manifests_root))
}

fn cmd_import(store: &Store, vault: &std::path::Path) -> anyhow::Result<()> {
    let s = import_vault(store, vault)?;
    println!("imported={} skipped={}", s.imported, s.skipped);
    Ok(())
}

fn cmd_search(store: &Store, query: &str, limit: i64) -> anyhow::Result<()> {
    for u in fts_search(store, query, limit)? {
        println!("{}\t{}\t{}", u.id, u.evidence_grade, u.title);
    }
    Ok(())
}

fn cmd_related(store: &Store, key: &str, depth: i64) -> anyhow::Result<()> {
    for r in bfs(store, key, depth)? {
        println!("{}\t{}\t(depth {})", r.edge_type, r.path, r.depth);
    }
    Ok(())
}

fn cmd_rank(store: &Store, limit: usize) -> anyhow::Result<()> {
    for (p, s) in pagerank(store)?.into_iter().take(limit) {
        println!("{:.6}\t{}", s, p);
    }
    Ok(())
}

fn cmd_add(store: &Store, title: String, content: String,
           vault_path: String, grade: String) -> anyhow::Result<()> {
    let id = store.add_unit(&Unit {
        title, content, vault_path, evidence_grade: grade,
        unit_type: "note".into(), ..Default::default()
    })?;
    println!("{}", id);
    Ok(())
}

fn cmd_edit(store: &Store, id: i64, title: Option<String>,
            content: Option<String>, grade: Option<String>) -> anyhow::Result<()> {
    let mut u = store.get_unit(id)?
        .ok_or_else(|| anyhow::anyhow!("id {id} not found"))?;
    if let Some(t) = title { u.title = t; }
    if let Some(c) = content { u.content = c; }
    if let Some(g) = grade { u.evidence_grade = g; }
    store.update_unit(&u)?;
    println!("updated {}", id);
    Ok(())
}

fn cmd_link(store: &Store, src: &str, dst: &str, edge_type: &str) -> anyhow::Result<()> {
    add_edge(store, src, dst, edge_type, 1.0)?;
    println!("linked {} -> {}", src, dst);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("kei-sage: {e:#}"); ExitCode::from(1) }
    }
}
