// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `kei-contacts-apple` — CardDAV client for iCloud Contacts.
//!
//! Authenticates with **Apple ID + app-specific password** (HTTP Basic Auth).
//! Does NOT use Sign in with Apple / OAuth — that scope does not cover contacts.
//!
//! # Quick start
//! ```no_run
//! # async fn example() -> Result<(), kei_contacts_apple::ContactsError> {
//! let client = kei_contacts_apple::ICloudCardDavClient::new(
//!     "user@icloud.com".to_string(),
//!     std::env::var("APPLE_APP_SPECIFIC_PASSWORD").unwrap(),
//! )
//! .with_addressbook_url(
//!     "https://p01-contacts.icloud.com/123456789/carddavhome/card/".to_string(),
//! );
//! let contacts = client.list_contacts().await?;
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
pub mod vcard;
pub(crate) mod xml;

pub use client::ICloudCardDavClient;
pub use contact::AppleContact;
pub use error::ContactsError;
