//! Machine-facing `run-atom <verb>` dispatcher.
//!
//! Reads JSON input (stdin or literal), dispatches to `atoms::<verb>::run`,
//! serializes the typed Output back to stdout. Exit codes mapped by caller.

use crate::atoms::{self, DispatchError};
use crate::Store;
use serde_json::Value;
use std::io::Read;

/// Read JSON input from an optional arg. `None` → read from stdin.
/// `Some("@path")` → read the file at `path`.
/// `Some(literal)` → parse the literal as JSON.
pub fn read_input(arg: Option<String>) -> Result<String, String> {
    match arg {
        Some(s) if s.starts_with('@') => {
            let path = &s[1..];
            std::fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))
        }
        Some(s) => Ok(s),
        None => read_stdin(),
    }
}

fn read_stdin() -> Result<String, String> {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .map_err(|e| format!("stdin: {e}"))?;
    Ok(buf)
}

/// Dispatch a verb to its atom. Returns serialized JSON on success.
pub fn dispatch(store: &Store, verb: &str, input_json: &str) -> Result<String, DispatchError> {
    let input: Value = serde_json::from_str(input_json)
        .map_err(|e| DispatchError::InvalidInput(e.to_string()))?;
    match verb {
        "create" => run_create(store, input),
        "add-dependency" => run_add_dep(store, input),
        "search" => run_search(store, input),
        other => Err(DispatchError::UnknownVerb(other.to_string())),
    }
}

fn run_create(store: &Store, input: Value) -> Result<String, DispatchError> {
    let parsed: atoms::create::Input = serde_json::from_value(input)
        .map_err(|e| DispatchError::InvalidInput(e.to_string()))?;
    let out = atoms::create::run(store, parsed).map_err(DispatchError::Create)?;
    serde_json::to_string(&out).map_err(|e| DispatchError::InvalidInput(e.to_string()))
}

fn run_add_dep(store: &Store, input: Value) -> Result<String, DispatchError> {
    let parsed: atoms::add_dependency::Input = serde_json::from_value(input)
        .map_err(|e| DispatchError::InvalidInput(e.to_string()))?;
    let out = atoms::add_dependency::run(store, parsed).map_err(DispatchError::AddDep)?;
    serde_json::to_string(&out).map_err(|e| DispatchError::InvalidInput(e.to_string()))
}

fn run_search(store: &Store, input: Value) -> Result<String, DispatchError> {
    let parsed: atoms::search::Input = serde_json::from_value(input)
        .map_err(|e| DispatchError::InvalidInput(e.to_string()))?;
    let out = atoms::search::run(store, parsed).map_err(DispatchError::Search)?;
    serde_json::to_string(&out).map_err(|e| DispatchError::InvalidInput(e.to_string()))
}

/// Map a `DispatchError` to the §Runtime exit-code contract.
/// Returns `(exit_code, stderr_msg)`.
pub fn exit_for_error(e: &DispatchError) -> u8 {
    match e {
        DispatchError::UnknownVerb(_) | DispatchError::InvalidInput(_) => 2,
        DispatchError::Create(err) => match err {
            atoms::create::Error::InvalidTitle | atoms::create::Error::InvalidPriority(_) => 2,
            atoms::create::Error::StoreError(_) => 1,
        },
        DispatchError::AddDep(err) => match err {
            atoms::add_dependency::Error::SelfDependency
            | atoms::add_dependency::Error::InvalidDepType(_)
            | atoms::add_dependency::Error::CycleDetected => 2,
            atoms::add_dependency::Error::StoreError(_) => 1,
        },
        DispatchError::Search(err) => match err {
            atoms::search::Error::InvalidQuery => 2,
            atoms::search::Error::StoreError(_) => 1,
        },
    }
}
