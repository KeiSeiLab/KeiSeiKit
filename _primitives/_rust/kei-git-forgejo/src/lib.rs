// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! kei-git-forgejo — public Forgejo (Codeberg-compatible) [`GitBackend`].
//!
//! Sibling of `kei-git-keigit` (private KeiGit-branded). The two share the
//! /api/v1 surface (Forgejo and Gitea are protocol-compatible) and differ
//! only in branding, DNA caps, body bytes, scope, and env var names.
//!
//! ## Branding axes
//!
//! | axis             | kei-git-keigit                  | kei-git-forgejo                  |
//! |------------------|---------------------------------|----------------------------------|
//! | provider_name    | `keigit`                        | `forgejo`                        |
//! | DNA caps         | `["PR", "AP", "KG"]`            | `["PR", "AP", "FJ"]`             |
//! | DNA scope        | `keiseikit.dev/.../kei-git-keigit` | `keiseikit.dev/.../kei-git-forgejo` |
//! | DNA body         | `b"keigit-priv-v1"`             | `b"forgejo-pub-v1"`              |
//! | env (URL)        | `KEIGIT_URL`                    | `FORGEJO_URL`                    |
//! | env (token)      | `KEIGIT_TOKEN`                  | `FORGEJO_TOKEN`                  |
//! | default URL      | `https://git.keisei.app`        | `https://codeberg.org`           |
//!
//! ## Quick start
//!
//! ```ignore
//! use kei_git_forgejo::{ForgejoBackend, ForgejoClient};
//! use kei_runtime_core::traits::git::{GitBackend, GitRemote, GitAuthKind};
//!
//! # async fn ex() -> kei_runtime_core::Result<()> {
//! let client = ForgejoClient::from_env()?;
//! let backend = ForgejoBackend::new(client, None)?;
//! let remote = GitRemote {
//!     url: "https://codeberg.org/me/demo.git".into(),
//!     branch: "main".into(),
//!     auth_kind: GitAuthKind::Pat,
//! };
//! backend.ensure_repo(&remote).await?;
//! # Ok(())
//! # }
//! ```

pub mod backend;
pub mod client;
pub mod error;

pub use backend::ForgejoBackend;
pub use client::{Branch, ForgejoClient, RepoInfo};
pub use error::{Error, Result};
