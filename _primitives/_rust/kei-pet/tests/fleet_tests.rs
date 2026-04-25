//! Hermetic tests for the multi-pet fleet module.
//!
//! Every test uses a fresh `tempfile::TempDir` as the fleet_root, so no
//! test touches real user state and no test depends on another's side
//! effects.

use std::path::PathBuf;

use kei_pet::fleet::{
    add_pet, load_fleet, per_pet_memory_key, shared_memory_key, switch_active, PetHandle,
};

fn mk_handle(name: &str, role: &str) -> PetHandle {
    PetHandle {
        pet_name: name.to_string(),
        role: role.to_string(),
        manifest_path: PathBuf::from(format!("/tmp/{name}.toml")),
        last_active: 0,
    }
}

#[test]
fn load_fleet_empty_returns_zero_pets() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let fleet = load_fleet("user-alpha", dir.path()).expect("load empty");
    assert_eq!(fleet.user_id, "user-alpha");
    assert!(fleet.pets.is_empty());
    assert!(fleet.active_pet.is_none());
}

#[test]
fn add_pet_persists_to_disk() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let handle = mk_handle("mira", "friend");
    add_pet("user-alpha", handle, dir.path()).expect("add");

    let fleet = load_fleet("user-alpha", dir.path()).expect("reload");
    assert_eq!(fleet.pets.len(), 1);
    assert_eq!(fleet.pets[0].pet_name, "mira");
    assert_eq!(fleet.pets[0].role, "friend");
    // First add should seed active_pet.
    assert_eq!(fleet.active_pet.as_deref(), Some("mira"));
}

#[test]
fn switch_active_updates_file() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    add_pet("user-alpha", mk_handle("mira", "friend"), dir.path()).expect("add 1");
    add_pet("user-alpha", mk_handle("nova", "tutor"), dir.path()).expect("add 2");

    switch_active("user-alpha", "nova", dir.path()).expect("switch");

    let fleet = load_fleet("user-alpha", dir.path()).expect("reload");
    assert_eq!(fleet.pets.len(), 2);
    assert_eq!(fleet.active_pet.as_deref(), Some("nova"));
}

#[test]
fn memory_keys_differ_per_pet_same_user() {
    let a = per_pet_memory_key("user-alpha", "mira");
    let b = per_pet_memory_key("user-alpha", "nova");
    assert_ne!(a, b);
    assert!(a.contains("user-alpha"));
    assert!(a.contains("mira"));
    assert!(b.contains("nova"));
}

#[test]
fn shared_memory_key_stable() {
    let k1 = shared_memory_key("user-alpha");
    let k2 = shared_memory_key("user-alpha");
    assert_eq!(k1, k2);
    assert_ne!(k1, shared_memory_key("user-beta"));
}

#[test]
fn switch_active_errors_when_pet_absent() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    add_pet("user-alpha", mk_handle("mira", "friend"), dir.path()).expect("add");

    let err = switch_active("user-alpha", "ghost", dir.path()).unwrap_err();
    assert!(matches!(err, kei_pet::fleet::FleetError::PetNotInFleet(_)));
}
