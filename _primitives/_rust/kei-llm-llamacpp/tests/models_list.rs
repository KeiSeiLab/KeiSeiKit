//! Tempdir with three files (two .gguf, one unrelated .bin):
//! list_models returns 2 ModelEntry, quants parsed correctly.

use kei_llm_llamacpp::list_models;

#[test]
fn models_list_returns_only_gguf_with_quant_parsed() {
    let td = tempfile::tempdir().unwrap();
    let q4 = td.path().join("llama-7b-Q4_K_M.gguf");
    let q8 = td.path().join("mistral-Q8_0.gguf");
    let bin = td.path().join("readme.bin");
    std::fs::write(&q4, b"placeholder Q4 weights").unwrap();
    std::fs::write(&q8, b"placeholder Q8 weights").unwrap();
    std::fs::write(&bin, b"unrelated binary blob").unwrap();

    let mut models = list_models(td.path()).unwrap();
    models.sort_by(|a, b| a.name.cmp(&b.name));

    assert_eq!(models.len(), 2, "should pick up only .gguf, got {models:?}");
    let q4_entry = models.iter().find(|m| m.name.contains("Q4_K_M")).expect("Q4 entry");
    let q8_entry = models.iter().find(|m| m.name.contains("Q8_0")).expect("Q8 entry");
    assert_eq!(q4_entry.quant.as_deref(), Some("Q4_K_M"));
    assert_eq!(q8_entry.quant.as_deref(), Some("Q8_0"));
    assert!(q4_entry.size_bytes > 0);
    assert!(q8_entry.size_bytes > 0);
}

#[test]
fn models_list_unknown_quant_is_none() {
    let td = tempfile::tempdir().unwrap();
    let path = td.path().join("custom-model-uuid.gguf");
    std::fs::write(&path, b"x").unwrap();
    let models = list_models(td.path()).unwrap();
    assert_eq!(models.len(), 1);
    assert!(models[0].quant.is_none(), "unknown filename → quant=None");
}

#[test]
fn models_list_empty_dir_returns_empty_vec() {
    let td = tempfile::tempdir().unwrap();
    let models = list_models(td.path()).unwrap();
    assert!(models.is_empty());
}

#[test]
fn models_list_nonexistent_dir_returns_empty_vec() {
    let bogus = std::path::PathBuf::from("/tmp/this-dir-must-not-exist-kei-llm-test");
    let models = list_models(&bogus).unwrap();
    assert!(models.is_empty(), "nonexistent dir is not an error");
}
