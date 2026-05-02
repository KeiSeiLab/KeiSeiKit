// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use thiserror::Error;

/// Runtime substrate errors. Trait impls map their backend-specific
/// errors into these variants.
#[derive(Debug, Error)]
pub enum Error {
    #[error("DNA: {0}")]
    Dna(#[from] kei_shared::dna::DnaError),

    #[error("registry: {0}")]
    Registry(String),

    #[error("config: {0}")]
    Config(String),

    #[error("network: {0}")]
    Network(String),

    #[error("auth: {0}")]
    Auth(String),

    #[error("CSRF state mismatch")]
    CsrfStateMismatch,

    #[error("provider: {0}")]
    Provider(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("budget exhausted: {0}")]
    Budget(String),

    #[error("permission denied: {0}")]
    Permission(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("other: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
