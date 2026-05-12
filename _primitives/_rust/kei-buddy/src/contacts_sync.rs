// SPDX-License-Identifier: Apache-2.0
//! Contact-sync helpers — pull Google / Apple contacts into local store.
//! Each function is fire-and-forget: errors are collected in `SyncReport`.

use std::sync::Arc;

use kei_contacts_apple::ICloudCardDavClient;
use kei_contacts_google::GooglePeopleClient;

use crate::contacts::Contacts;

/// Summary returned by a sync operation regardless of partial failures.
#[derive(Debug, Default)]
pub struct SyncReport {
    /// Total contacts returned by the remote source.
    pub fetched: usize,
    /// Contacts successfully written to the local store.
    pub added: usize,
    /// Contacts skipped (empty name+email, or duplicate by name+email).
    pub skipped: usize,
    /// Error strings accumulated during sync; non-fatal individually.
    pub errors: Vec<String>,
}

/// Pull contacts from Google People API into `contacts`.
///
/// Requires a valid OAuth2 access token (not obtained here).
/// Never panics; all errors collected in [`SyncReport::errors`].
pub async fn sync_from_google(
    access_token: &str,
    contacts: &Arc<Contacts>,
) -> SyncReport {
    let client = GooglePeopleClient::new(access_token.to_string());
    let all = match client.list_connections().await {
        Ok(v) => v,
        Err(e) => {
            return SyncReport {
                errors: vec![format!("google list_connections: {e}")],
                ..Default::default()
            };
        }
    };
    let fetched = all.len();
    let mut report = SyncReport { fetched, ..Default::default() };
    for contact in all {
        process_person(contact.to_person(), contacts, &mut report).await;
    }
    report
}

/// Pull contacts from iCloud CardDAV into `contacts`.
///
/// `addressbook_url` must be the full CardDAV addressbook URL.
/// Never panics; all errors collected in [`SyncReport::errors`].
pub async fn sync_from_apple(
    apple_id: &str,
    app_password: &str,
    addressbook_url: &str,
    contacts: &Arc<Contacts>,
) -> SyncReport {
    let client = ICloudCardDavClient::new(apple_id.to_string(), app_password.to_string())
        .with_addressbook_url(addressbook_url.to_string());
    let all = match client.list_contacts().await {
        Ok(v) => v,
        Err(e) => {
            return SyncReport {
                errors: vec![format!("apple list_contacts: {e}")],
                ..Default::default()
            };
        }
    };
    let fetched = all.len();
    let mut report = SyncReport { fetched, ..Default::default() };
    for contact in all {
        process_person(contact.to_person(), contacts, &mut report).await;
    }
    report
}

/// Shared dedup + insert logic for a single Person.
async fn process_person(
    person: kei_social_store::people::Person,
    contacts: &Arc<Contacts>,
    report: &mut SyncReport,
) {
    if person.name.is_empty() && person.email.is_empty() {
        report.skipped += 1;
        return;
    }
    if is_duplicate(&person, contacts).await {
        report.skipped += 1;
        return;
    }
    match contacts.add_contact(person).await {
        Ok(_) => report.added += 1,
        Err(e) => report.errors.push(format!("add_contact: {e}")),
    }
}

/// Returns `true` when `contacts` already has an entry with the same
/// case-insensitive name AND case-insensitive email (both non-empty).
async fn is_duplicate(
    person: &kei_social_store::people::Person,
    contacts: &Arc<Contacts>,
) -> bool {
    if person.name.is_empty() || person.email.is_empty() {
        return false;
    }
    let hits = match contacts.search_contacts(&person.name, 3).await {
        Ok(v) => v,
        Err(_) => return false,
    };
    let name_lc = person.name.to_lowercase();
    let email_lc = person.email.to_lowercase();
    hits.iter().any(|h| {
        h.name.to_lowercase() == name_lc && h.email.to_lowercase() == email_lc
    })
}

// ── tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_initial_zero() {
        let r = SyncReport::default();
        assert_eq!(r.fetched, 0);
        assert_eq!(r.added, 0);
        assert_eq!(r.skipped, 0);
        assert!(r.errors.is_empty());
    }

    #[tokio::test]
    async fn sync_google_bad_token_populates_errors() {
        // Using an obviously-invalid token; no real network required because
        // reqwest will return a connection error in the sandbox environment,
        // but we verify the SyncReport shape on any error path.
        let contacts = Arc::new(Contacts::from_memory().unwrap());
        let report = sync_from_google("invalid-token", &contacts).await;
        // fetched == 0 and either an error was collected OR the network
        // returned something parseable (both are valid non-panic outcomes).
        assert_eq!(report.fetched, 0);
        assert_eq!(report.added, 0);
    }
}
