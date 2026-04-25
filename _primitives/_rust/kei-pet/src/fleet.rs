//! Multi-pet fleet per user.
//!
//! One user_id owns N pet personas. All pets under that user share one
//! user-level memory scope (shared_memory_key), but each pet keeps its own
//! conversation stream (per_pet_memory_key). Fleet state is serialized to
//! `<fleet_root>/<user_id>/fleet.toml`; per-pet manifests are written by
//! the caller at paths recorded in `PetHandle::manifest_path`.
//!
//! Scope boundary: this module owns only the fleet index file. It never
//! reads or writes individual pet manifests — those are the caller's
//! responsibility, referenced here by `PathBuf` only.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Fleet = ordered list of pet handles plus the currently active pet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetFleet {
    pub user_id: String,
    pub pets: Vec<PetHandle>,
    pub active_pet: Option<String>,
}

/// Pointer to one pet persona + its role + manifest location on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetHandle {
    pub pet_name: String,
    pub role: String,
    pub manifest_path: PathBuf,
    pub last_active: i64,
}

/// Errors surfaced by fleet operations.
#[derive(Debug, thiserror::Error)]
pub enum FleetError {
    #[error("fleet not found for user {0}")]
    NotFound(String),
    #[error("pet {0} not in fleet")]
    PetNotInFleet(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
}

/// Canonical on-disk path for a user's fleet index file.
pub fn fleet_path(user_id: &str, fleet_root: &Path) -> PathBuf {
    fleet_root.join(user_id).join("fleet.toml")
}

/// Load fleet for `user_id`. If the index file does not yet exist, return
/// an empty fleet (no pets, no active). Parse errors propagate.
pub fn load_fleet(user_id: &str, fleet_root: &Path) -> Result<PetFleet, FleetError> {
    let path = fleet_path(user_id, fleet_root);
    if !path.exists() {
        return Ok(PetFleet {
            user_id: user_id.to_string(),
            pets: Vec::new(),
            active_pet: None,
        });
    }
    let text = std::fs::read_to_string(&path)?;
    let fleet: PetFleet = toml::from_str(&text)?;
    Ok(fleet)
}

/// Serialize fleet to `<fleet_root>/<user_id>/fleet.toml`, creating the
/// parent directory if needed. Overwrites existing file atomically enough
/// for single-writer use; concurrent writers must layer their own locking.
pub fn save_fleet(fleet: &PetFleet, fleet_root: &Path) -> Result<(), FleetError> {
    let path = fleet_path(&fleet.user_id, fleet_root);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let text = toml::to_string_pretty(fleet)?;
    std::fs::write(&path, text)?;
    Ok(())
}

/// Append `handle` to the user's fleet. If this is the first pet added,
/// it also becomes `active_pet`. Creates the fleet file if absent.
pub fn add_pet(
    user_id: &str,
    handle: PetHandle,
    fleet_root: &Path,
) -> Result<(), FleetError> {
    let mut fleet = load_fleet(user_id, fleet_root)?;
    if fleet.active_pet.is_none() {
        fleet.active_pet = Some(handle.pet_name.clone());
    }
    fleet.pets.push(handle);
    save_fleet(&fleet, fleet_root)
}

/// Set `active_pet` to `pet_name`. Errors if the fleet is absent or the
/// pet name is not present in the fleet.
pub fn switch_active(
    user_id: &str,
    pet_name: &str,
    fleet_root: &Path,
) -> Result<(), FleetError> {
    let path = fleet_path(user_id, fleet_root);
    if !path.exists() {
        return Err(FleetError::NotFound(user_id.to_string()));
    }
    let mut fleet = load_fleet(user_id, fleet_root)?;
    if !fleet.pets.iter().any(|p| p.pet_name == pet_name) {
        return Err(FleetError::PetNotInFleet(pet_name.to_string()));
    }
    fleet.active_pet = Some(pet_name.to_string());
    save_fleet(&fleet, fleet_root)
}

/// Shared memory key (all pets under this user share this scope).
pub fn shared_memory_key(user_id: &str) -> String {
    format!("shared::{user_id}")
}

/// Per-pet memory key (one conversation stream per (user, pet) pair).
pub fn per_pet_memory_key(user_id: &str, pet_name: &str) -> String {
    format!("pet::{user_id}::{pet_name}")
}
