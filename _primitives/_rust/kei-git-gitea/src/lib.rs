// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! kei-git-gitea — GitBackend impl for Gitea (gitea.com or self-hosted).
//!
//! Gitea exposes a REST API at `/api/v1` that is near-identical to the
//! Forgejo fork (Forgejo was originally a Gitea hard-fork). The same
//! endpoint surface used by `kei-git-keigit` works against Gitea with
//! zero protocol-level deviation. Differences are limited to defaults
//! (base URL, env-var name).
//!
//! Constructor Pattern: 4 source files, each <200 LOC, one responsibility:
//! * `error.rs` — error type + From impls
//! * `client.rs` — typed HTTP client over `/api/v1`
//! * `backend.rs` — `GitBackend` trait impl that wraps the client +
//!   shells out to `git` for clone/push/mirror
//!
//! Auth: bearer token from `GITEA_TOKEN`. Base URL from `GITEA_URL`
//! (default `https://gitea.com`). Constructor accepts both explicitly
//! so wiremock tests can point at a localhost mock.

pub mod backend;
pub mod client;
pub mod error;

pub use backend::GiteaBackend;
pub use client::{CreateRepoRequest, GiteaClient, RepoInfo};
pub use error::{Error, Result};
