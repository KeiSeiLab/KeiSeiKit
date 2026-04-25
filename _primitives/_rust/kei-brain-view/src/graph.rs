//! In-memory taxonomy graph built from the kei-ledger SQLite file.
//!
//! Constructor Pattern: one cube = Node + Graph + adjacency builder.
//! Read-only: we never write to the ledger, only hydrate a view.

use crate::error::{BrainViewError, Result};
use rusqlite::{Connection, OptionalExtension};
use serde::Serialize;
use std::collections::BTreeMap;

/// One hydrated ledger row reduced to the fields brain-view renders.
/// Status is kept as `String` (not enum) to survive schema drift — any
/// new status string from kei-ledger shows up in the stats bucket verbatim.
#[derive(Debug, Clone, Serialize)]
pub struct Node {
    pub id: String,
    pub branch: String,
    pub parent_branch: Option<String>,
    pub status: String,
    pub started_ts: i64,
    pub summary: Option<String>,
    pub dna: Option<String>,
    pub creator_id: Option<String>,
    pub fork_parent_id: Option<String>,
}

/// In-memory adjacency over the `agents` table.
///
/// `by_id` / `by_branch` are ordered maps so downstream rendering is
/// deterministic across runs (BTreeMap iteration order). `children_of`
/// maps parent_branch to a vector of child ids, sorted by `started_ts`
/// so the oldest fork renders first.
#[derive(Debug, Default, Serialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub by_id: BTreeMap<String, usize>,
    pub by_branch: BTreeMap<String, usize>,
    pub children_of: BTreeMap<String, Vec<usize>>,
    pub roots: Vec<usize>,
}

impl Graph {
    /// Look up a node by id; index is the slot in `self.nodes`.
    pub fn node(&self, idx: usize) -> &Node {
        &self.nodes[idx]
    }
}

/// Read all rows from the ledger's `agents` table and build the graph.
/// Works against kei-ledger schema v4+ (dna + creator_id + fork_parent_id
/// present; older rows simply have NULLs in those columns).
pub fn build_graph(conn: &Connection) -> Result<Graph> {
    let rows = fetch_rows(conn)?;
    Ok(assemble_graph(rows))
}

fn fetch_rows(conn: &Connection) -> Result<Vec<Node>> {
    // Tolerate older schemas: if `creator_id` / `fork_parent_id` / `dna`
    // don't exist (v1 schema), COALESCE them via pragma_table_info check.
    let has_v4 = has_column(conn, "creator_id")?;
    let has_v2 = has_column(conn, "dna")?;
    let sql = build_select_sql(has_v4, has_v2);
    let mut stmt = conn.prepare(&sql)?;
    let iter = stmt.query_map([], |r| {
        Ok(Node {
            id: r.get(0)?,
            branch: r.get(1)?,
            parent_branch: r.get(2)?,
            status: r.get(3)?,
            started_ts: r.get(4)?,
            summary: r.get(5)?,
            dna: r.get(6)?,
            creator_id: r.get(7)?,
            fork_parent_id: r.get(8)?,
        })
    })?;
    let mut out = Vec::new();
    for n in iter {
        out.push(n?);
    }
    Ok(out)
}

fn has_column(conn: &Connection, col: &str) -> Result<bool> {
    let found: Option<i64> = conn
        .query_row(
            "SELECT 1 FROM pragma_table_info('agents') WHERE name = ?1",
            [col],
            |r| r.get(0),
        )
        .optional()?;
    Ok(found.is_some())
}

fn build_select_sql(has_v4: bool, has_v2: bool) -> String {
    let dna_expr = if has_v2 { "dna" } else { "NULL" };
    let creator_expr = if has_v4 { "creator_id" } else { "NULL" };
    let fork_expr = if has_v4 { "fork_parent_id" } else { "NULL" };
    format!(
        "SELECT id, branch, parent_branch, status, started_ts, summary, \
         {dna_expr}, {creator_expr}, {fork_expr} \
         FROM agents ORDER BY started_ts ASC, id ASC"
    )
}

fn assemble_graph(rows: Vec<Node>) -> Graph {
    let mut g = Graph::default();
    for (idx, n) in rows.into_iter().enumerate() {
        g.by_id.insert(n.id.clone(), idx);
        g.by_branch.insert(n.branch.clone(), idx);
        g.nodes.push(n);
    }
    for (idx, n) in g.nodes.iter().enumerate() {
        match n.parent_branch.as_deref() {
            Some(p) if g.by_branch.contains_key(p) => {
                g.children_of.entry(p.to_string()).or_default().push(idx);
            }
            _ => g.roots.push(idx),
        }
    }
    g
}

/// Resolve a DNA prefix to a unique node id. Returns `DnaNotFound` if no
/// match, `DnaAmbiguous` if more than one. Exact match always wins over
/// prefix match.
pub fn resolve_dna<'a>(graph: &'a Graph, needle: &str) -> Result<&'a Node> {
    let mut matches: Vec<&Node> = Vec::new();
    for n in &graph.nodes {
        if let Some(dna) = &n.dna {
            if dna == needle {
                return Ok(n);
            }
            if dna.starts_with(needle) {
                matches.push(n);
            }
        }
    }
    match matches.len() {
        0 => Err(BrainViewError::DnaNotFound(needle.to_string())),
        1 => Ok(matches[0]),
        n => Err(BrainViewError::DnaAmbiguous {
            prefix: needle.to_string(),
            count: n,
        }),
    }
}
