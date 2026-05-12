// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `kei-contacts-google` — thin client for Google People API v1.
//!
//! Expects an OAuth2 access token already acquired (e.g. from `kei-auth-google`).
//! Does NOT perform OAuth itself.
//!
//! # Quick start
//! ```no_run
//! # async fn example() -> Result<(), kei_contacts_google::ContactsError> {
//! let token = std::env::var("GOOGLE_ACCESS_TOKEN").unwrap();
//! let client = kei_contacts_google::GooglePeopleClient::new(token);
//! let contacts = client.list_connections().await?;
//! for c in &contacts {
//!     let person = c.to_person();
//!     println!("{} <{}>", person.name, person.email);
//! }
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod contact;
pub mod error;
pub(crate) mod pagination;

pub use client::GooglePeopleClient;
pub use contact::GoogleContact;
pub use error::ContactsError;
