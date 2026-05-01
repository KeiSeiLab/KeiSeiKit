// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-git-bitbucket — Bitbucket Cloud impl of [`kei_runtime_core::GitBackend`].
//!
//! Layout:
//! - [`error`]: local `Error`/`Result` mapping into the runtime-core error.
//! - [`client`]: thin async REST 2.0 wrapper (mockable base URL).
//! - [`backend`]: [`BitbucketBackend`] — DNA-bearing trait impl.
//!
//! Auth: HTTP Basic with `BITBUCKET_USERNAME` + `BITBUCKET_APP_PASSWORD`.
//! Base URL defaults to `https://api.bitbucket.org/2.0` and is overridable
//! for `wiremock` tests via `BITBUCKET_URL`.

pub mod backend;
pub mod client;
pub mod error;

pub use backend::BitbucketBackend;
pub use client::{BitbucketClient, BranchRef, Repository};
pub use error::{Error, Result};
