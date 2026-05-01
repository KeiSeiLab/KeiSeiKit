//! Model discovery — fixture cache directory with three children.
//!
//! Layout:
//!   models--mlx-community--Llama-3.2-3B-Instruct-4bit  ← MLX, 4 bits
//!   models--mlx-community--phi-3-mini-mlx-q8           ← MLX, 8 bits
//!   models--meta-llama--Llama-3.2-3B-Instruct          ← NOT mlx
//!
//! Expected: 2 entries returned (sorted by hf_id).

use kei_llm_mlx::list_models;
use std::fs;

#[test]
fn lists_only_mlx_quantised() {
    let tmp = tempfile::tempdir().expect("tmp");
    let root = tmp.path();
    for sub in [
        "models--mlx-community--Llama-3.2-3B-Instruct-4bit",
        "models--mlx-community--phi-3-mini-mlx-q8",
        "models--meta-llama--Llama-3.2-3B-Instruct",
    ] {
        fs::create_dir(root.join(sub)).expect("mkdir");
    }
    let out = list_models(root);
    assert_eq!(out.len(), 2, "two MLX-quantised entries expected");
    let ids: Vec<&str> = out.iter().map(|m| m.hf_id.as_str()).collect();
    assert!(ids.contains(&"mlx-community/Llama-3.2-3B-Instruct-4bit"));
    assert!(ids.contains(&"mlx-community/phi-3-mini-mlx-q8"));
    let four = out.iter().find(|m| m.hf_id.ends_with("-4bit")).unwrap();
    assert_eq!(four.quant_bits, Some(4));
    let eight = out.iter().find(|m| m.hf_id.ends_with("-mlx-q8")).unwrap();
    assert_eq!(eight.quant_bits, Some(8));
}
