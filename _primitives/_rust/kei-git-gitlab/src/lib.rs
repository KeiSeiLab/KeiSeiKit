// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! kei-git-gitlab — GitBackend impl for GitLab.com SaaS.
//!
//! REST API v4 + PRIVATE-TOKEN header auth.
//! Project identity: url-encoded `namespace/name` (or numeric project_id).
//! `clone` / `push` shell out to the system `git` CLI; the API is only used
//! for existence + auto-create + branch-SHA lookups.

pub mod backend;
pub mod client;
pub mod error;

pub use backend::GitlabBackend;
pub use client::{GitlabClient, ProjectInfo};
pub use error::{Error, Result};
