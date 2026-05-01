//! kei-hibernate — whole-brain export/import for KeiSeiKit state.
//!
//! Wave 14 primitive. Serialises an entire KeiSei installation
//! (sqlite stores + capabilities / roles / blocks / agents /
//! skills / hooks) into a single tar.zst bundle with a SHA-256
//! manifest, then restores it on another machine.
//!
//! Public surface kept deliberately small: `export`, `import`,
//! `inspect`. Each dispatches to a Constructor-Pattern cube.

pub mod error;
pub mod manifest;
pub mod collector;
pub mod sha;
pub mod exporter;
pub mod importer;
pub mod inspector;

pub use error::Error;
pub use exporter::{export, ExportMeta};
pub use importer::{import, ImportReport};
pub use inspector::{inspect, InspectReport};
pub use manifest::{HibernateManifest, ManifestEntry, MANIFEST_FILENAME, MANIFEST_VERSION};
