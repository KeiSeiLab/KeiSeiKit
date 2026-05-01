//! Integration tests for plan_parser.
//! All fixtures are synthetic strings — no real plan files touched.

use kei_import_project::plan_parser::{parse_plan, parse_plan_file};
use tempfile::NamedTempFile;
use std::io::Write as _;

const SYNTHETIC_PLAN: &str = r#"# alpha-project — Migration Plan

> Generated: 2026-01-01T00:00:00Z
> Source: file:///tmp/alpha
> Average confidence: 0.75

## STATUS BANNER

> AUTO-GENERATED initial plan.

| Phase | Trait family | Modules | Priority | Initial status |
|---|---|---:|---:|---|
| P0.1 | MemoryBackend | 2 | 0 | scaffolding |
| P1.1 | ComputeProvider | 1 | 1 | scaffolding |
| Pwip.1 | LlmBackend | 1 | 99 | blocked-needs-review |

## Per-phase detail

### P0.1 — MemoryBackend

Modules to port:
- mem-sled (confidence 0.85)
- mem-pg (confidence 0.78)

Verification gate (RULE 0.13 + RULE 0.16):
- `cargo check --workspace` PASS

### P1.1 — ComputeProvider

Modules to port:
- compute-vultr (confidence 0.75)

Verification gate (RULE 0.13 + RULE 0.16):
- `cargo check --workspace` PASS

### Pwip.1 — LlmBackend

Modules to port:
- llm-partial (confidence 0.40)

## Unmatched modules

These do not match any trait.

- glue-code
- legacy-adapter

## Follow-up

- Apply skeletons
"#;

#[test]
fn parses_project_name() {
    let plan = parse_plan(SYNTHETIC_PLAN).unwrap();
    assert_eq!(plan.project_name, "alpha-project");
}

#[test]
fn parses_source_repo() {
    let plan = parse_plan(SYNTHETIC_PLAN).unwrap();
    assert_eq!(plan.source_repo, "file:///tmp/alpha");
}

#[test]
fn parses_three_phases_correctly() {
    let plan = parse_plan(SYNTHETIC_PLAN).unwrap();
    assert_eq!(plan.phases.len(), 3, "expected 3 phases");

    let p0 = &plan.phases[0];
    assert_eq!(p0.id, "P0.1");
    assert_eq!(p0.trait_family, "MemoryBackend");
    assert_eq!(p0.priority, 0);
    assert_eq!(p0.status, "scaffolding");
    assert_eq!(p0.modules.len(), 2);
    assert_eq!(p0.modules[0].name, "mem-sled");
    assert!((p0.modules[0].confidence - 0.85).abs() < 1e-6);

    let pwip = &plan.phases[2];
    assert_eq!(pwip.id, "Pwip.1");
    assert_eq!(pwip.status, "blocked-needs-review");
}

#[test]
fn parses_unmatched_modules() {
    let plan = parse_plan(SYNTHETIC_PLAN).unwrap();
    assert_eq!(plan.unmatched, vec!["glue-code", "legacy-adapter"]);
}

#[test]
fn empty_content_produces_empty_plan() {
    let plan = parse_plan("# empty-proj — Migration Plan\n").unwrap();
    assert_eq!(plan.project_name, "empty-proj");
    assert!(plan.phases.is_empty());
    assert!(plan.unmatched.is_empty());
}

#[test]
fn parse_plan_file_round_trips_through_tempfile() {
    let mut f = NamedTempFile::new().unwrap();
    write!(f, "{}", SYNTHETIC_PLAN).unwrap();
    let plan = parse_plan_file(f.path()).unwrap();
    assert_eq!(plan.project_name, "alpha-project");
    assert_eq!(plan.phases.len(), 3);
}
