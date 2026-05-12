// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! [`AppleContact`] — normalised contact returned by the CardDAV client.

use kei_social_store::people::Person;
use serde::{Deserialize, Serialize};

/// A single contact entry from iCloud CardDAV, normalised to flat strings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppleContact {
    /// vCard UID property (stable identifier).
    pub uid: String,
    /// Formatted name (FN property).
    pub display_name: String,
    /// Given name (first component of N property).
    pub given_name: String,
    /// Family name (second component of N property).
    pub family_name: String,
    /// All EMAIL values from the vCard.
    pub emails: Vec<String>,
    /// All TEL values from the vCard.
    pub phones: Vec<String>,
    /// ORG property (first component).
    pub organization: String,
    /// NOTE property.
    pub note: String,
    /// Original vCard text (verbatim).
    pub raw_vcard: String,
}

impl AppleContact {
    /// Map to a [`kei_social_store::people::Person`] for store ingestion.
    ///
    /// - `name` — `display_name`, falling back to `"{given} {family}"`.
    /// - `email` — first email or empty string.
    /// - `source` — `"apple:{uid}"`.
    /// - `id` / `created_at` / `updated_at` — zero (assigned by the store).
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
            source: format!("apple:{}", self.uid),
            bio: self.note.clone(),
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
        let c = AppleContact {
            uid: "abc-123".to_string(),
            display_name: "Denis Parfionovich".to_string(),
            given_name: "Denis".to_string(),
            family_name: "Parfionovich".to_string(),
            emails: vec!["denis@example.com".to_string()],
            phones: vec!["+1234567890".to_string()],
            organization: "KeiSei Labs".to_string(),
            note: "founder".to_string(),
            raw_vcard: String::new(),
        };
        let p = c.to_person();
        assert_eq!(p.name, "Denis Parfionovich");
        assert_eq!(p.email, "denis@example.com");
        assert_eq!(p.source, "apple:abc-123");
        assert_eq!(p.organization, "KeiSei Labs");
        assert_eq!(p.bio, "founder");
        assert_eq!(p.id, 0);
    }

    #[test]
    fn to_person_fallback_name() {
        let c = AppleContact {
            uid: "x".to_string(),
            given_name: "Alice".to_string(),
            family_name: "Smith".to_string(),
            ..Default::default()
        };
        let p = c.to_person();
        assert_eq!(p.name, "Alice Smith");
    }
}
