//! Discovery — neither binary present.
//!
//! Mock both `which` invocations to return non-zero (mlx_lm not pip
//! installed). Assert all `MlxBins` fields are `None`.

use kei_llm_mlx::runner::RunOutput;
use kei_llm_mlx::{discover, MockRunner};

#[test]
fn no_bins_yields_empty_struct() {
    std::env::remove_var("KEI_MLX_BIN");
    let runner = MockRunner::new()
        .with_run(
            "which_mlx_lm.generate",
            RunOutput::fail(1, "mlx_lm.generate not found"),
        )
        .with_run(
            "which_mlx_lm.server",
            RunOutput::fail(1, "mlx_lm.server not found"),
        );
    let bins = discover(&runner);
    assert!(bins.generate.is_none(), "no binary should be discovered");
    assert!(bins.server.is_none());
    assert!(bins.version.is_none());
    assert!(!bins.any_present());
}
