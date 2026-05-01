//! `generate()` translates a non-zero subprocess exit into
//! `Error::NonZeroExit { code, stderr }` with the stderr captured.
//!
//! On non-target builds, `generate()` short-circuits with NotSupported
//! before reaching the runner — so the assertion is wrapped in a
//! cfg-gate. On target builds we exercise the full path with a mock
//! that returns exit=2 + a stderr message.

use kei_llm_mlx::error::Error;
use kei_llm_mlx::generate::{generate, GenerateOpts};
use kei_llm_mlx::runner::RunOutput;
use kei_llm_mlx::{is_supported, MockRunner};

#[test]
fn nonzero_exit_is_typed() {
    let s = is_supported();
    if !s.supported {
        return; // covered by error_not_supported.rs
    }
    // On target: build a runner that returns exit=2 from mlx_lm.generate.
    // We use the absolute bin path the runner sees (we pass it to
    // `generate` directly), so the fixture stem is computed off it.
    let bin = "/opt/mlx/bin/mlx_lm.generate";
    // stem mirrors `fixture_stem(cmd, args)`: every non-`[A-Za-z0-9._-]`
    // byte (including the leading `/`) collapses to `_`.
    let stem = "_opt_mlx_bin_mlx_lm.generate_--model_m_--prompt_p";
    let runner = MockRunner::new()
        .with_run(stem, RunOutput::fail(2, "model not found in cache"));
    let res = generate(&runner, bin, "m", "p", &GenerateOpts::default());
    match res {
        Err(Error::NonZeroExit { code, stderr }) => {
            assert_eq!(code, Some(2));
            assert!(stderr.contains("model not found"));
        }
        other => panic!("expected NonZeroExit, got {other:?}"),
    }
}
