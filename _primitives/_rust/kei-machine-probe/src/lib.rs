//! kei-machine-probe — public library surface (Wave 56).
//!
//! Foundation for the local-LLM stack: every Wave 57-60 primitive
//! (ollama / llamacpp / mlx / router) calls `probe()` to know what the
//! current machine can run before choosing a backend.
//!
//! Constructor Pattern: each detector lives in its own module and
//! accepts a `&dyn Runner` so unit tests can substitute a fixture-backed
//! mock. NO direct `std::process::Command::new` outside `runner.rs`.

pub mod arch;
pub mod cli;
pub mod gpu;
pub mod markdown;
pub mod memory;
pub mod os;
pub mod profile;
pub mod recommendations;
pub mod runner;
pub mod tooling;

pub use arch::detect_arch;
pub use gpu::detect_gpu;
pub use markdown::{render_markdown, render_plain};
pub use memory::detect_memory;
pub use os::detect_os;
pub use profile::{
    AppleVariant, ArchInfo, CpuFamily, GpuInfo, Machine, MemoryInfo, OsFamily, OsInfo,
    ToolingInfo,
};
pub use recommendations::{recommend, BackendTier, Capability, Recommendations};
pub use runner::{fixture_stem, MockRunner, Runner, SystemRunner};
pub use tooling::detect_tooling;

/// One-shot probe — runs every detector and returns a fully populated
/// `Machine`. The optional `skip_tooling` flag avoids the four `which`
/// + `--version` shell-outs (CI / fast-path mode).
pub fn probe(runner: &dyn Runner, skip_tooling: bool) -> Machine {
    let os = detect_os(runner);
    let arch = detect_arch(runner);
    let memory = detect_memory(runner);
    let gpu = detect_gpu(runner);
    let tooling = if skip_tooling {
        ToolingInfo::default()
    } else {
        detect_tooling(runner)
    };
    Machine {
        os,
        arch,
        memory,
        gpu,
        tooling,
        source_commands: source_commands(skip_tooling),
    }
}

fn source_commands(skip_tooling: bool) -> Vec<String> {
    let mut v: Vec<String> = vec![
        "sw_vers -productVersion".into(),
        "sw_vers -buildVersion".into(),
        "uname -sr".into(),
        "sysctl -n hw.model".into(),
        "sysctl -n machdep.cpu.brand_string".into(),
        "sysctl -n hw.optional.arm64".into(),
        "sysctl -n hw.ncpu".into(),
        "sysctl -n hw.memsize".into(),
        "vm_stat".into(),
        "system_profiler SPDisplaysDataType -json".into(),
    ];
    if !skip_tooling {
        v.extend([
            "which ollama".into(),
            "ollama --version".into(),
            "which brew".into(),
            "brew --version".into(),
            "which llama-server".into(),
            "llama-server --version".into(),
        ]);
    }
    v
}
