//! Discovery — both binaries present.
//!
//! Mock the two `which` invocations and a `--help` call for the version
//! probe; assert `MlxBins` is fully populated.

use kei_llm_mlx::{discover, MockRunner};

#[test]
fn both_bins_and_version_populated() {
    // Defensive: ensure the env-override branch is not taken in CI.
    std::env::remove_var("KEI_MLX_BIN");
    let runner = MockRunner::new()
        .with_ok("which_mlx_lm.generate", "/opt/mlx/bin/mlx_lm.generate\n")
        .with_ok("which_mlx_lm.server", "/opt/mlx/bin/mlx_lm.server\n")
        .with_ok(
            "_opt_mlx_bin_mlx_lm.generate_--help",
            "usage: mlx_lm.generate [-h] ...\nMLX-LM 0.20.4 \u{2014} local LLM inference for Apple Silicon\n",
        );
    let bins = discover(&runner);
    assert_eq!(
        bins.generate.as_deref().and_then(|p| p.to_str()),
        Some("/opt/mlx/bin/mlx_lm.generate"),
    );
    assert_eq!(
        bins.server.as_deref().and_then(|p| p.to_str()),
        Some("/opt/mlx/bin/mlx_lm.server"),
    );
    assert_eq!(bins.version.as_deref(), Some("0.20.4"));
    assert!(bins.any_present());
}
