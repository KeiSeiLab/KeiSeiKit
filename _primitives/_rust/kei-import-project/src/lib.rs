//! kei-import-project — foreign project ingestion runtime.
//!
//! Composes existing primitives (kei-shared, kei-registry, kei-decompose,
//! kei-skill-importer) to clone a foreign codebase, walk its tree,
//! identify language modules, and stage them for downstream atomars
//! (architecture mapping, skill extraction, migration plan generation,
//! phase execution).

pub mod doc_walker;
pub mod execute_cmd;
pub mod executor;
pub mod fragment_writer;
pub mod gap_report;
pub mod identifier;
pub mod map_cmd;
pub mod matcher;
pub mod md_splitter;
pub mod module_source;
pub mod phase_prompt;
pub mod plan_cmd;
pub mod plan_generator;
pub mod plan_parser;
pub mod plan_render;
pub mod registry_writer;
pub mod skeleton;
pub mod skeleton_table;
pub mod skill_extractor;
pub mod trait_kind;
pub mod trait_patterns;
pub mod walker;

pub use identifier::{identify_modules, ModuleKind, ProjectModule};
pub use matcher::{match_module, MatchScore};
pub use module_source::ModuleSource;
pub use registry_writer::{project_slug, register_modules, RegisterResult};
pub use gap_report::{render_gap_report, ModuleAnalysis};
pub use plan_generator::{build_plan, MigrationPhase, MigrationPlan};
pub use plan_render::render_markdown as render_plan_md;
pub use skeleton::render_skeleton;
pub use skill_extractor::{extract_skills, ExtractResult, ExtractedSkill};
pub use trait_patterns::{all_patterns, TraitKind, TraitPattern};
pub use walker::{walk_repo, FileEntry, Language, RepoWalk};
