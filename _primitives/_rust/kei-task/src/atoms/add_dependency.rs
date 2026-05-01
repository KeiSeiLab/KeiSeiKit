//! kei-task::add-dependency atom — see atoms/add-dependency.md for contract.

use crate::deps::add_dependency as add_dep_impl;
use crate::store::Store;
use crate::types::{is_valid_dep, VALID_DEP_TYPES};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub from: i64,
    pub to: i64,
    #[serde(default)]
    pub dep_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub ok: bool,
}

#[derive(Debug)]
pub enum Error {
    SelfDependency,
    InvalidDepType(String),
    CycleDetected,
    StoreError(anyhow::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SelfDependency => write!(f, "SelfDependency: task cannot depend on itself"),
            Error::InvalidDepType(t) => write!(
                f, "InvalidDepType: {t} (allowed: {})", VALID_DEP_TYPES.join(", ")
            ),
            Error::CycleDetected => write!(f, "CycleDetected: edge would close a cycle"),
            Error::StoreError(e) => write!(f, "StoreError: {e:#}"),
        }
    }
}

impl std::error::Error for Error {}

pub fn run(store: &Store, input: Input) -> Result<Output, Error> {
    validate(&input)?;
    let dep = normalize_dep(&input.dep_type);
    add_dep_impl(store, input.from, input.to, &dep).map_err(classify_error)?;
    Ok(Output { ok: true })
}

fn validate(input: &Input) -> Result<(), Error> {
    if input.from == input.to {
        return Err(Error::SelfDependency);
    }
    if !input.dep_type.is_empty() && !is_valid_dep(&input.dep_type) {
        return Err(Error::InvalidDepType(input.dep_type.clone()));
    }
    Ok(())
}

fn normalize_dep(raw: &str) -> String {
    if raw.is_empty() { "blocks".into() } else { raw.to_string() }
}

fn classify_error(e: anyhow::Error) -> Error {
    let msg = format!("{e:#}");
    if msg.contains("cycle") {
        Error::CycleDetected
    } else if msg.contains("self-dependency") {
        Error::SelfDependency
    } else if msg.contains("invalid dep type") {
        Error::InvalidDepType(msg)
    } else {
        Error::StoreError(e)
    }
}
