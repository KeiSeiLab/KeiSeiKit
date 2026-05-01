//! Phase 3 core: extract skills from a repo's documentation and register them.
//!
//! Algorithm:
//! 1. Walk doc paths via doc_walker.
//! 2. For each file: split on H2 headings via md_splitter.
//! 3. Filter fragments with meaningful heading (≥3 chars) + non-empty body.
//! 4. Write SKILL.md to fragments_dir via fragment_writer.
//! 5. Register as BlockType::Skill in kei-registry.

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::doc_walker::collect_doc_paths;
use crate::fragment_writer::{fragment_path, render_skill_md, sanitize, write_fragment, WriteOutcome};
use crate::md_splitter::{first_sentences, split_by_h2};

/// One extracted skill fragment.
#[derive(Debug, Clone)]
pub struct ExtractedSkill {
    pub source_doc: PathBuf,
    pub fragment_slug: String,
    pub frontmatter_name: String,
    pub frontmatter_description: String,
    pub body: String,
}

/// Aggregate outcome from a single extract_skills call.
#[derive(Debug, Default)]
pub struct ExtractResult {
    pub extracted: Vec<ExtractedSkill>,
    pub written_files: Vec<PathBuf>,
    pub registered: usize,
    pub superseded: usize,
    pub unchanged: usize,
}

/// Extract skills from `repo_root` docs, writing to `fragments_dir`.
/// If `registry_db` is Some, registers rows in kei-registry.
/// Pass `registry_db = None` for library-level operation without a DB.
pub fn extract_skills(
    repo_root: &Path,
    project_slug: &str,
    fragments_dir: &Path,
    registry_db: Option<&Path>,
) -> Result<ExtractResult> {
    let doc_paths = collect_doc_paths(repo_root);
    let mut result = ExtractResult::default();
    let conn = registry_db
        .map(kei_registry::store::open_db)
        .transpose()?;

    for doc_path in &doc_paths {
        let source_stem = stem_of(doc_path);
        process_doc(doc_path, &source_stem, project_slug, fragments_dir, &conn, &mut result)?;
    }
    Ok(result)
}

fn process_doc(
    doc_path: &Path,
    source_stem: &str,
    project_slug: &str,
    fragments_dir: &Path,
    conn: &Option<rusqlite::Connection>,
    result: &mut ExtractResult,
) -> Result<()> {
    let text = std::fs::read_to_string(doc_path)
        .map_err(|e| anyhow::anyhow!("read {}: {}", doc_path.display(), e))?;
    for (section_slug, heading, body) in split_by_h2(&text) {
        if heading.len() < 3 || body.trim().is_empty() {
            continue;
        }
        let skill = build_skill(project_slug, source_stem, &section_slug, &body, doc_path);
        let fpath = fragment_path(fragments_dir, project_slug, source_stem, &section_slug);
        let content = render_skill_md(&skill.frontmatter_name, &skill.frontmatter_description, &skill.body);
        let outcome = write_fragment(&fpath, &content)?;
        match outcome {
            WriteOutcome::Written => {
                register_if_conn(conn, &skill, &fpath, &content, result)?;
                result.written_files.push(fpath);
            }
            WriteOutcome::Unchanged => {
                result.unchanged += 1;
            }
        }
        result.extracted.push(skill);
    }
    Ok(())
}

fn build_skill(
    project_slug: &str,
    source_stem: &str,
    section_slug: &str,
    body: &str,
    doc_path: &Path,
) -> ExtractedSkill {
    let fragment_slug = format!("{source_stem}::{section_slug}");
    let frontmatter_name = format!("{project_slug}::{fragment_slug}");
    let frontmatter_description = first_sentences(body.trim(), 200);
    ExtractedSkill {
        source_doc: doc_path.to_path_buf(),
        fragment_slug,
        frontmatter_name,
        frontmatter_description,
        body: body.to_string(),
    }
}

fn register_if_conn(
    conn: &Option<rusqlite::Connection>,
    skill: &ExtractedSkill,
    fpath: &Path,
    content: &str,
    result: &mut ExtractResult,
) -> Result<()> {
    let Some(c) = conn else { return Ok(()); };
    let path_str = fpath.to_string_lossy();
    let existing = kei_registry::find_by_path(c, &path_str)?;
    kei_registry::register(
        c,
        kei_registry::BlockType::Skill,
        &skill.frontmatter_name,
        &path_str,
        content.as_bytes(),
        "skill",
    )?;
    match existing {
        None => result.registered += 1,
        Some(old) => {
            let still_active = row_still_active(c, old.id)?;
            if !still_active {
                result.superseded += 1;
            }
        }
    }
    Ok(())
}

fn row_still_active(conn: &rusqlite::Connection, id: i64) -> Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT superseded_by FROM blocks WHERE id = ?1",
    )?;
    let superseded_by: Option<String> =
        stmt.query_row(rusqlite::params![id], |r| r.get(0))?;
    Ok(superseded_by.is_none())
}

/// Return the file stem (filename without extension) as a sanitized slug.
fn stem_of(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(sanitize)
        .unwrap_or_else(|| "doc".to_string())
}
