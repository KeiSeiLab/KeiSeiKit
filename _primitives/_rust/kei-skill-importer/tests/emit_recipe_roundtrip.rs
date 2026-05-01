//! Recipe emit + parseable-TOML roundtrip.

use kei_skill_importer::{emit, AtomCall, AtomCallKind, ImportedSkill, Phase, SourceFormat};
use std::path::PathBuf;

fn synthetic_skill() -> ImportedSkill {
    ImportedSkill {
        name: "two-step-recipe".into(),
        description: "Two resolved kei calls in sequence".into(),
        source_format: SourceFormat::ClaudeCode,
        source_path: PathBuf::from("/tmp/synthetic-recipe"),
        language: Some("en".into()),
        tags: vec![],
        phases: vec![Phase {
            name: "main".into(),
            body: String::new(),
            atom_calls: vec![
                AtomCall {
                    raw_command: "kei-task create build-step".into(),
                    atom_id: Some("kei-task::create".into()),
                    kind: AtomCallKind::KeiPrimitive,
                },
                AtomCall {
                    raw_command: "kei-cortex chat plan".into(),
                    atom_id: Some("kei-cortex::chat".into()),
                    kind: AtomCallKind::KeiPrimitive,
                },
            ],
        }],
        tools_required: vec![],
        yaml_frontmatter: None,
        body: String::new(),
    }
}

#[test]
fn render_recipe_yields_parseable_toml() {
    let skill = synthetic_skill();
    let toml_text = emit::as_recipe::render(&skill).expect("render");
    let parsed: toml::Value = toml::from_str(&toml_text).expect("parse TOML");
    let recipe = parsed.get("recipe").expect("[recipe] table");
    assert_eq!(recipe.get("name").and_then(|v| v.as_str()), Some("two-step-recipe"));
    let imported_from = recipe
        .get("imported_from")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(imported_from.starts_with("claude://"),
        "imported_from URI: {}", imported_from);
}

#[test]
fn recipe_steps_have_sequential_depends_on() {
    let skill = synthetic_skill();
    let toml_text = emit::as_recipe::render(&skill).expect("render");
    let parsed: toml::Value = toml::from_str(&toml_text).expect("parse TOML");
    let steps = parsed
        .get("steps")
        .and_then(|v| v.as_array())
        .expect("[[steps]]");
    assert_eq!(steps.len(), 2, "expected 2 steps");
    let step1 = &steps[0];
    let step2 = &steps[1];
    assert_eq!(step1.get("id").and_then(|v| v.as_str()), Some("step-1"));
    assert_eq!(
        step1.get("depends_on").and_then(|v| v.as_array()).unwrap().len(),
        0
    );
    let dep2 = step2.get("depends_on").and_then(|v| v.as_array()).unwrap();
    assert_eq!(dep2.len(), 1);
    assert_eq!(dep2[0].as_str(), Some("step-1"));
}

#[test]
fn recipe_step_atom_id_round_trips() {
    let skill = synthetic_skill();
    let toml_text = emit::as_recipe::render(&skill).expect("render");
    let parsed: toml::Value = toml::from_str(&toml_text).expect("parse");
    let steps = parsed.get("steps").and_then(|v| v.as_array()).unwrap();
    let atoms: Vec<&str> = steps
        .iter()
        .filter_map(|s| s.get("atom").and_then(|v| v.as_str()))
        .collect();
    assert_eq!(atoms, vec!["kei-task::create", "kei-cortex::chat"]);
}

#[test]
fn write_recipe_creates_recipes_subdir_file() {
    let skill = synthetic_skill();
    let tmp = tempfile::tempdir().unwrap();
    let path = emit::as_recipe::write(&skill, tmp.path()).expect("write");
    assert!(path.exists());
    let parent_name = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap();
    assert_eq!(parent_name, "recipes");
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains("[recipe]"));
    assert!(text.contains("[[steps]]"));
}
