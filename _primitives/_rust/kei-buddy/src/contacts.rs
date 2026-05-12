// SPDX-License-Identifier: Apache-2.0
//! `Contacts` — async address-book adapter over `kei-social-store`.
//! Arc<Mutex<Store>> + spawn_blocking pattern (same as chat_log.rs).
//! Never `await` while holding the std::sync::Mutex guard.

use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex};

use kei_social_store::{
    graph::Pair,
    interactions::{self, Interaction},
    people::{self, Person},
    search::search_people,
    Store,
};

use crate::error::BuddyError;

/// Async contact-book backed by `kei-social-store`.
pub struct Contacts {
    store: Arc<Mutex<Store>>,
}

impl Contacts {
    /// Open a file-backed contact store at `path`.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, BuddyError> {
        let store = Store::open(path.as_ref())
            .map_err(|e| BuddyError::Memory(format!("{e}")))?;
        Ok(Self { store: Arc::new(Mutex::new(store)) })
    }

    /// Open an in-memory store (useful for tests).
    pub fn from_memory() -> Result<Self, BuddyError> {
        let store = Store::open_memory()
            .map_err(|e| BuddyError::Memory(format!("{e}")))?;
        Ok(Self { store: Arc::new(Mutex::new(store)) })
    }

    /// Add a contact; returns the new row id.
    pub async fn add_contact(&self, p: Person) -> Result<i64, BuddyError> {
        let store = Arc::clone(&self.store);
        tokio::task::spawn_blocking(move || {
            let locked = store.lock().expect("store mutex poisoned");
            people::add_person(&locked, &p).map_err(|e| BuddyError::Memory(format!("{e}")))
        })
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
    }

    /// Retrieve a contact by id; `None` if not found.
    pub async fn get_contact(&self, id: i64) -> Result<Option<Person>, BuddyError> {
        let store = Arc::clone(&self.store);
        tokio::task::spawn_blocking(move || {
            let locked = store.lock().expect("store mutex poisoned");
            people::get_person(&locked, id).map_err(|e| BuddyError::Memory(format!("{e}")))
        })
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
    }

    /// Full-text search over contacts.
    pub async fn search_contacts(&self, q: &str, limit: i64) -> Result<Vec<Person>, BuddyError> {
        let store = Arc::clone(&self.store);
        let q = q.to_string();
        tokio::task::spawn_blocking(move || {
            let locked = store.lock().expect("store mutex poisoned");
            search_people(&locked, &q, limit).map_err(|e| BuddyError::Memory(format!("{e}")))
        })
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
    }

    /// Log a meeting between `person_id` (source) and `target_id` on `channel`.
    /// Returns the new interaction row id.
    pub async fn log_meet(
        &self,
        person_id: i64,
        target_id: i64,
        channel: &str,
        note: &str,
    ) -> Result<i64, BuddyError> {
        let store = Arc::clone(&self.store);
        let interaction = Interaction {
            id: 0,
            person_id,
            target_id,
            interaction_type: "meet".to_string(),
            channel: channel.to_string(),
            content: note.to_string(),
            timestamp: 0,
        };
        tokio::task::spawn_blocking(move || {
            let locked = store.lock().expect("store mutex poisoned");
            interactions::log_interaction(&locked, &interaction)
                .map_err(|e| BuddyError::Memory(format!("{e}")))
        })
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
    }

    /// List all interactions where `person_id` is the source.
    pub async fn interactions_for(&self, person_id: i64) -> Result<Vec<Interaction>, BuddyError> {
        let store = Arc::clone(&self.store);
        tokio::task::spawn_blocking(move || {
            let locked = store.lock().expect("store mutex poisoned");
            interactions::interactions_for(&locked, person_id)
                .map_err(|e| BuddyError::Memory(format!("{e}")))
        })
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
    }

    /// Returns the full relationship graph as `Vec<Pair>`.
    pub async fn relationship_graph(&self) -> Result<Vec<Pair>, BuddyError> {
        let store = Arc::clone(&self.store);
        tokio::task::spawn_blocking(move || {
            let locked = store.lock().expect("store mutex poisoned");
            kei_social_store::graph::relationship_graph(&locked)
                .map_err(|e| BuddyError::Memory(format!("{e}")))
        })
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
    }

    /// People who interacted with BOTH `person_a` AND `person_b`
    /// (appear as `target_id` in pairs for both sources).
    pub async fn common_connections(
        &self,
        person_a: i64,
        person_b: i64,
    ) -> Result<Vec<i64>, BuddyError> {
        let pairs = self.relationship_graph().await?;
        let targets_a: HashSet<i64> = pairs
            .iter()
            .filter(|p| p.person_id == person_a)
            .map(|p| p.target_id)
            .collect();
        let targets_b: HashSet<i64> = pairs
            .iter()
            .filter(|p| p.person_id == person_b)
            .map(|p| p.target_id)
            .collect();
        Ok(targets_a.intersection(&targets_b).copied().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kei_social_store::Person;

    fn alice() -> Person { Person { name: "Denis".to_string(), email: "d@test.com".to_string(), ..Default::default() } }
    fn bob() -> Person { Person { name: "Bob".to_string(), email: "b@test.com".to_string(), ..Default::default() } }
    fn charlie() -> Person { Person { name: "Charlie".to_string(), email: "c@test.com".to_string(), ..Default::default() } }

    #[tokio::test]
    async fn add_and_get_contact_roundtrip() {
        let contacts = Contacts::from_memory().unwrap();
        let id = contacts.add_contact(alice()).await.unwrap();
        assert!(id > 0);
        let found = contacts.get_contact(id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Denis");
    }

    #[tokio::test]
    async fn search_contacts_finds_by_name() {
        let contacts = Contacts::from_memory().unwrap();
        contacts.add_contact(alice()).await.unwrap();
        contacts.add_contact(bob()).await.unwrap();
        let results = contacts.search_contacts("Denis", 10).await.unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|p| p.name == "Denis"));
    }

    #[tokio::test]
    async fn log_meet_and_list_interactions() {
        let contacts = Contacts::from_memory().unwrap();
        let a = contacts.add_contact(alice()).await.unwrap();
        let b = contacts.add_contact(bob()).await.unwrap();
        let iid = contacts.log_meet(a, b, "telegram", "hi").await.unwrap();
        assert!(iid > 0);
        let list = contacts.interactions_for(a).await.unwrap();
        assert!(!list.is_empty());
        assert_eq!(list[0].channel, "telegram");
    }

    #[tokio::test]
    async fn common_connections_finds_shared_target() {
        let contacts = Contacts::from_memory().unwrap();
        let a = contacts.add_contact(alice()).await.unwrap();
        let b = contacts.add_contact(bob()).await.unwrap();
        let c = contacts.add_contact(charlie()).await.unwrap();
        contacts.log_meet(a, c, "telegram", "met charlie").await.unwrap();
        contacts.log_meet(b, c, "telegram", "also met charlie").await.unwrap();
        let common = contacts.common_connections(a, b).await.unwrap();
        assert!(common.contains(&c));
    }
}
