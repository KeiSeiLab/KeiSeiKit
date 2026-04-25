//! ASCII tree rendering with optional ANSI color.
//!
//! Constructor Pattern: one cube = render entrypoints + color helpers.
//! Colors obey `NO_COLOR` convention (https://no-color.org) — present
//! env var of any value disables ANSI escape codes at runtime.

use crate::error::{BrainViewError, Result, MAX_TREE_DEPTH};
use crate::graph::{Graph, Node};
use std::collections::HashSet;

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_CYAN: &str = "\x1b[36m";
const ANSI_DIM: &str = "\x1b[2m";

/// Render the full graph as an indented text tree (roots first, BFS
/// children). Colors status labels when stdout is not redirected and
/// `NO_COLOR` is unset.
pub fn render_ascii(graph: &Graph) -> String {
    let colored = color_enabled();
    render_ascii_with_color(graph, colored)
}

/// Explicit-color variant — used by tests to assert color-free output.
pub fn render_ascii_with_color(graph: &Graph, colored: bool) -> String {
    let mut out = String::new();
    for &root_idx in &graph.roots {
        render_subtree(graph, root_idx, 0, &mut out, colored);
    }
    out
}

/// Render the lineage (ancestors + focus + descendants) for a single DNA.
pub fn render_lineage(graph: &Graph, focus: &Node, colored: bool) -> Result<String> {
    let mut out = String::new();
    let ancestors = walk_ancestors(graph, focus)?;
    out.push_str("ANCESTORS:\n");
    for (depth, n) in ancestors.iter().enumerate() {
        out.push_str(&format_line(n, depth, colored));
    }
    out.push_str(&format!("FOCUS ({} matches):\n", 1));
    out.push_str(&format_line(focus, 0, colored));
    out.push_str("DESCENDANTS:\n");
    render_subtree_by_branch(graph, &focus.branch, 0, &mut out, colored);
    Ok(out)
}

fn render_subtree(graph: &Graph, idx: usize, depth: usize, out: &mut String, colored: bool) {
    let n = graph.node(idx);
    out.push_str(&format_line(n, depth, colored));
    if depth + 1 > MAX_TREE_DEPTH {
        return;
    }
    if let Some(kids) = graph.children_of.get(&n.branch) {
        for &k in kids {
            render_subtree(graph, k, depth + 1, out, colored);
        }
    }
}

fn render_subtree_by_branch(
    graph: &Graph,
    parent_branch: &str,
    depth: usize,
    out: &mut String,
    colored: bool,
) {
    if depth > MAX_TREE_DEPTH {
        return;
    }
    if let Some(kids) = graph.children_of.get(parent_branch) {
        for &k in kids {
            let n = graph.node(k);
            out.push_str(&format_line(n, depth + 1, colored));
            render_subtree_by_branch(graph, &n.branch, depth + 1, out, colored);
        }
    }
}

fn walk_ancestors<'a>(graph: &'a Graph, focus: &'a Node) -> Result<Vec<&'a Node>> {
    let mut out = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut cur_parent = focus.parent_branch.clone();
    let mut steps = 0usize;
    while let Some(pb) = cur_parent {
        steps += 1;
        if steps > MAX_TREE_DEPTH {
            return Err(BrainViewError::MaxDepthExceeded(MAX_TREE_DEPTH));
        }
        if !seen.insert(pb.clone()) {
            break;
        }
        let Some(&idx) = graph.by_branch.get(&pb) else {
            break;
        };
        let n = graph.node(idx);
        out.push(n);
        cur_parent = n.parent_branch.clone();
    }
    out.reverse();
    Ok(out)
}

fn format_line(n: &Node, depth: usize, colored: bool) -> String {
    let indent = "  ".repeat(depth);
    let status = colorize_status(&n.status, colored);
    let dna = n.dna.as_deref().unwrap_or("-");
    let dna_short = dna.chars().take(20).collect::<String>();
    let dna_fmt = if colored {
        format!("{ANSI_DIM}{dna_short}{ANSI_RESET}")
    } else {
        dna_short
    };
    format!(
        "{indent}- [{status}] {id}  branch={branch}  dna={dna_fmt}\n",
        id = n.id,
        branch = n.branch,
    )
}

fn colorize_status(status: &str, colored: bool) -> String {
    if !colored {
        return status.to_string();
    }
    let code = match status {
        "done" | "merged" => ANSI_GREEN,
        "failed" | "rejected" => ANSI_RED,
        "running" => ANSI_YELLOW,
        _ => ANSI_CYAN,
    };
    format!("{code}{status}{ANSI_RESET}")
}

fn color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none()
}
