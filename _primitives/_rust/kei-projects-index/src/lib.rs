//! kei-projects-index — public library surface.
//!
//! Constructor Pattern: each module is one cube with one responsibility.
//! `kei-projects-watcher` (sibling daemon) and `kei-cortex` (HTTP daemon)
//! both depend on this crate's library API to read / write the project
//! state DB at `~/.claude/agents/projects-index.sqlite`.

pub mod docs;
pub mod git_state;
pub mod index;
pub mod query;
pub mod row;
pub mod schema;
pub mod sqlite_scan;
pub mod walk;

pub use docs::{detect_docs, DocsState};
pub use git_state::{detect_git_state, GitState};
pub use index::rebuild_index;
pub use query::{get_one, list_all};
pub use row::ProjectRow;
pub use schema::init;
pub use sqlite_scan::count_sqlite_files;
pub use walk::{walk_projects_root, ProjectEntry};
