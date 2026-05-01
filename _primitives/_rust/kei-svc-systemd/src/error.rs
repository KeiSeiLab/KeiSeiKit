// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("systemctl not found in PATH (not on a systemd host?)")]
    SystemctlNotFound,
    #[error("systemctl {cmd} failed: {stderr}")]
    SystemctlFailed { cmd: String, stderr: String },
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("DNA: {0}")]
    Dna(#[from] kei_runtime_core::DnaError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::SystemctlNotFound => kei_runtime_core::Error::Provider(e.to_string()),
            Error::SystemctlFailed { .. } => kei_runtime_core::Error::Provider(e.to_string()),
            Error::Dna(e) => kei_runtime_core::Error::Dna(e),
            other => kei_runtime_core::Error::Other(other.to_string()),
        }
    }
}
