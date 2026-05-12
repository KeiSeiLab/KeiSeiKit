// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! [`GoogleContact`] — normalised contact returned by the People API.

use kei_social_store::people::Person;
use serde::{Deserialize, Serialize};

/// A single contact entry from the Google People API, normalised to flat strings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GoogleContact {
    /// E.g. `"people/c123456"` — stable identifier from the People API.
    pub resource_name: String,
    pub display_name: String,
    pub given_name: String,
    pub family_name: String,
    /// All email addresses reported by the API.
    pub emails: Vec<String>,
    /// All phone numbers reported by the API.
    pub phones: Vec<String>,
    /// Primary organization name (first entry).
    pub organization: String,
    /// First biography/note.
    pub bio: String,
}

impl GoogleContact {
    /// Map to a [`kei_social_store::people::Person`] ready for store ingestion.
    ///
    /// - `name` — `display_name`, falling back to `"{given} {family}"`.
    /// - `email` — first email or empty string.
    /// - `source` — `"google:{resource_name}"`.
    /// - `id` / `created_at` / `updated_at` — zero (assigned by the store on insert).
    pub fn to_person(&self) -> Person {
        let name = if !self.display_name.is_empty() {
            self.display_name.clone()
        } else {
            format!("{} {}", self.given_name, self.family_name)
        };
        let email = self.emails.first().cloned().unwrap_or_default();
        Person {
            id: 0,
            name,
            email,
            handle: String::new(),
            role: String::new(),
            organization: self.organization.clone(),
            source: format!("google:{}", self.resource_name),
            bio: self.bio.clone(),
            created_at: 0,
            updated_at: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_person_maps_correctly() {
        let c = GoogleContact {
            resource_name: "people/c99".to_string(),
            display_name: "Alice Smith".to_string(),
            given_name: "Alice".to_string(),
            family_name: "Smith".to_string(),
            emails: vec!["alice@example.com".to_string()],
            phones: vec!["+1-555-0100".to_string()],
            organization: "ACME Corp".to_string(),
            bio: "Engineer".to_string(),
        };
        let p = c.to_person();
        assert_eq!(p.name, "Alice Smith");
        assert_eq!(p.email, "alice@example.com");
        assert_eq!(p.source, "google:people/c99");
        assert_eq!(p.organization, "ACME Corp");
        assert_eq!(p.bio, "Engineer");
        assert_eq!(p.id, 0);
    }

    #[test]
    fn to_person_fallback_name() {
        let c = GoogleContact {
            resource_name: "people/c1".to_string(),
            display_name: String::new(),
            given_name: "Bob".to_string(),
            family_name: "Jones".to_string(),
            ..Default::default()
        };
        let p = c.to_person();
        assert_eq!(p.name, "Bob Jones");
    }
}
