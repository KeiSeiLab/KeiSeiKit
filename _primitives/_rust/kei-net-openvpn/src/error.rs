// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Crate-local error. Bridges into `kei_runtime_core::Error` so
//! `NetworkMode` impl methods can `?` through this and surface a
//! substrate-level error to callers without leaking systemctl /
//! management-socket transport details.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("systemctl failed: {0}")]
    SystemctlFailed(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse: {0}")]
    Parse(String),

    #[error("management socket unavailable")]
    MgmtSocketUnavailable,

    #[error("DNA: {0}")]
    Dna(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<kei_runtime_core::DnaError> for Error {
    fn from(e: kei_runtime_core::DnaError) -> Self {
        Error::Dna(e.to_string())
    }
}

/// Bridge into substrate-level error. OpenVPN-specific failures are
/// funnelled into `Network` / `Provider` / `Config` based on cause; the
/// message string preserves the upstream shape (systemctl stderr / parse
/// detail / IO message).
impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::SystemctlFailed(s) => {
                kei_runtime_core::Error::Provider(format!("openvpn systemctl: {s}"))
            }
            Error::Io(io) => kei_runtime_core::Error::Io(io),
            Error::Parse(s) => {
                kei_runtime_core::Error::Provider(format!("openvpn parse: {s}"))
            }
            Error::MgmtSocketUnavailable => kei_runtime_core::Error::Network(
                "openvpn management socket unavailable".into(),
            ),
            Error::Dna(s) => {
                kei_runtime_core::Error::Provider(format!("openvpn dna: {s}"))
            }
        }
    }
}
