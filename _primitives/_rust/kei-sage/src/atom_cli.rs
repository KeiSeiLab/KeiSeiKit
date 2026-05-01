//! CLI handlers for `atoms-*` subcommands — walks, indexes, queries atoms.
//!
//! Separate from `main.rs` to keep both files under Constructor Pattern
//! 200-LOC limit. `main.rs` wires clap, this module implements the verbs.

use crate::atom_index::index_atoms;
use crate::atoms::{discover_atoms, AtomRecord};
use crate::bfs::bfs;
use crate::facet_query::{discover_primitives_with_roles, matches_all, parse_filters};
use crate::lineage::{discover_lineage, nodes_by_author, trace_lineage};
use crate::pagerank::pagerank;
use crate::rule_index::discover_rules;
use crate::search::fts_search;
use crate::store::Store;
use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn default_atoms_root() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/agents/_primitives/_rust")
}

pub fn default_rules_root() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/rules")
}

pub fn default_capabilities_root() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/_capabilities")
}

pub fn default_manifests_root() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/_manifests")
}

pub fn default_roles_root() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/_roles")
}

pub fn cmd_facet_query(
    cap_root: &Path,
    man_root: &Path,
    roles_root: &Path,
    filters: &[String],
) -> Result<()> {
    let pairs = parse_filters(filters);
    let all = discover_primitives_with_roles(cap_root, man_root, Some(roles_root));
    for p in all.iter().filter(|p| matches_all(p, &pairs)) {
        println!("{}", p.full_id);
    }
    Ok(())
}

pub fn cmd_lineage(cap_root: &Path, man_root: &Path, id: &str, depth: usize) -> Result<()> {
    let nodes = discover_lineage(cap_root, man_root);
    let trace = trace_lineage(&nodes, id, depth);
    if let Some(f) = &trace.focus {
        if let Some(c) = &f.created_at {
            if let Some(by) = &f.created_by {
                println!("created: {} by {}", c, by);
            } else {
                println!("created: {}", c);
            }
        } else if let Some(by) = &f.created_by {
            println!("created-by: {}", by);
        }
    }
    println!("ancestors: {}", format_list(&trace.ancestors));
    println!("descendants: {}", format_list(&trace.descendants));
    Ok(())
}

pub fn cmd_author(cap_root: &Path, man_root: &Path, creator: &str, limit: usize) -> Result<()> {
    let nodes = discover_lineage(cap_root, man_root);
    for n in nodes_by_author(&nodes, creator, limit) {
        let ts = n.created_at.unwrap_or_else(|| "-".into());
        println!("{}\t{}", ts, n.id);
    }
    Ok(())
}

fn format_list(items: &[String]) -> String {
    if items.is_empty() { "(none)".into() } else { items.join(", ") }
}

pub fn cmd_atoms_discover(root: &Path) -> Result<()> {
    let records = discover_atoms(root)?;
    println!("full_id\tkind\tstability\tmd_path");
    for r in &records {
        println!(
            "{}\t{}\t{}\t{}",
            r.full_id,
            r.kind.as_str(),
            r.stability,
            r.md_path.display()
        );
    }
    eprintln!("discovered {} atom(s) under {}", records.len(), root.display());
    Ok(())
}

pub fn cmd_rules_discover(root: &Path) -> Result<()> {
    let records = discover_rules(root)?;
    println!("slug\tname\tpath");
    for r in &records {
        println!("{}\t{}\t{}", r.slug, r.name, r.md_path.display());
    }
    eprintln!("discovered {} rule(s) under {}", records.len(), root.display());
    Ok(())
}

pub fn cmd_atoms_rank(store: &Store, root: &Path, limit: usize) -> Result<()> {
    ingest(store, root)?;
    for (path, score) in pagerank(store)?.into_iter().take(limit) {
        println!("{:.6}\t{}", score, path);
    }
    Ok(())
}

pub fn cmd_atoms_related(store: &Store, root: &Path, atom_id: &str, depth: i64) -> Result<()> {
    ingest(store, root)?;
    for r in bfs(store, atom_id, depth)? {
        println!("{}\t{}\t(depth {})", r.edge_type, r.path, r.depth);
    }
    Ok(())
}

pub fn cmd_atoms_search(store: &Store, root: &Path, query: &str, limit: i64) -> Result<()> {
    ingest(store, root)?;
    for u in fts_search(store, query, limit)? {
        if u.unit_type != "atom" {
            continue;
        }
        println!("{}\t{}\t{}", u.id, u.category, u.vault_path);
    }
    Ok(())
}

fn ingest(store: &Store, root: &Path) -> Result<Vec<AtomRecord>> {
    let records = discover_atoms(root)?;
    let stats = index_atoms(store, &records)?;
    eprintln!(
        "indexed {} atom(s), {} wikilink edge(s) from {}",
        stats.units_indexed,
        stats.edges_indexed,
        root.display()
    );
    Ok(records)
}
