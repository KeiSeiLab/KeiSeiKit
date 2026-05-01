//! kei-changelog — library surface.
//!
//! Public modules, re-exported for the binary and integration tests.
//! Constructor Pattern: one file = one concern; keep this root < 30 LOC.

pub mod commit;
pub mod group;
pub mod parse;
pub mod render;
pub mod walk;

pub use commit::{Commit, CommitKind};
pub use group::Grouped;
pub use parse::parse_subject;
pub use render::{render_markdown, RenderOpts};
pub use walk::{walk_range, WalkRange};
