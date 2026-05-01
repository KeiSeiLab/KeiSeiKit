// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("mlx: {0}")]
    Mlx(String),
    #[error("DNA: {0}")]
    Dna(#[from] kei_runtime_core::DnaError),
    #[error("wrong platform: MLX requires macOS Apple Silicon")]
    WrongPlatform,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<kei_llm_mlx::Error> for Error {
    fn from(e: kei_llm_mlx::Error) -> Self {
        Error::Mlx(e.to_string())
    }
}

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Dna(e) => kei_runtime_core::Error::Dna(e),
            Error::WrongPlatform => kei_runtime_core::Error::Provider(
                "MLX requires macOS Apple Silicon".into()
            ),
            Error::Mlx(s) => kei_runtime_core::Error::Provider(format!("mlx: {s}")),
        }
    }
}
