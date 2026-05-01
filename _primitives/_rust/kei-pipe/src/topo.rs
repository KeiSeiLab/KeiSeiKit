//! Kahn-style topological sort for the parsed DAG.
//!
//! Split out from `dag.rs` to stay under the Constructor Pattern 200-LOC
//! limit. Stable — ties are broken by declaration order so reports are
//! deterministic across runs.

use std::collections::{BTreeMap, HashMap};

use crate::dag::{DagError, DagSpec, Step};

/// Topologically sort the DAG. Returns `&Step` references in execution
/// order.
pub fn topo_sort(spec: &DagSpec) -> Result<Vec<&Step>, DagError> {
    let idx = index_by_id(spec);
    validate_edges(spec, &idx)?;
    let (in_deg, adj) = build_graph(spec, &idx);
    let ordered = kahn_sort(spec, in_deg, adj)?;
    Ok(ordered.iter().map(|i| &spec.steps[*i]).collect())
}

fn index_by_id(spec: &DagSpec) -> HashMap<&str, usize> {
    let mut m: HashMap<&str, usize> = HashMap::with_capacity(spec.steps.len());
    for (i, s) in spec.steps.iter().enumerate() {
        m.insert(s.id.as_str(), i);
    }
    m
}

fn validate_edges(spec: &DagSpec, idx: &HashMap<&str, usize>) -> Result<(), DagError> {
    for s in &spec.steps {
        for dep in &s.depends_on {
            if !idx.contains_key(dep.as_str()) {
                return Err(DagError::UnknownDep(s.id.clone(), dep.clone()));
            }
        }
    }
    Ok(())
}

fn build_graph(
    spec: &DagSpec,
    idx: &HashMap<&str, usize>,
) -> (Vec<usize>, Vec<Vec<usize>>) {
    let n = spec.steps.len();
    let mut in_deg = vec![0usize; n];
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, s) in spec.steps.iter().enumerate() {
        for dep in &s.depends_on {
            let src = idx[dep.as_str()];
            adj[src].push(i);
            in_deg[i] += 1;
        }
    }
    (in_deg, adj)
}

fn kahn_sort(
    spec: &DagSpec,
    mut in_deg: Vec<usize>,
    adj: Vec<Vec<usize>>,
) -> Result<Vec<usize>, DagError> {
    let n = spec.steps.len();
    let mut ready: BTreeMap<usize, ()> = BTreeMap::new();
    seed_ready(&in_deg, &mut ready);
    let mut out: Vec<usize> = Vec::with_capacity(n);
    while let Some((&i, _)) = ready.iter().next() {
        ready.remove(&i);
        out.push(i);
        for &j in &adj[i] {
            in_deg[j] -= 1;
            if in_deg[j] == 0 {
                ready.insert(j, ());
            }
        }
    }
    if out.len() != n {
        return Err(DagError::Cycle(unresolved_ids(spec, &out)));
    }
    Ok(out)
}

fn seed_ready(in_deg: &[usize], ready: &mut BTreeMap<usize, ()>) {
    for (i, deg) in in_deg.iter().enumerate() {
        if *deg == 0 {
            ready.insert(i, ());
        }
    }
}

fn unresolved_ids(spec: &DagSpec, resolved: &[usize]) -> String {
    spec.steps
        .iter()
        .enumerate()
        .filter(|(i, _)| !resolved.contains(i))
        .map(|(_, s)| s.id.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}
