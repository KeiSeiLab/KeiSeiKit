//! Render `Machine` + `Recommendations` as a markdown / plaintext report.
//!
//! Constructor Pattern: rendering is one cube. The CLI's `report`
//! subcommand calls `render_markdown(&m, &r)` (or `render_plain` when
//! `--markdown` is absent). No formatting logic in `cli.rs`.

use crate::profile::{CpuFamily, GpuInfo, Machine, OsFamily};
use crate::recommendations::Recommendations;

pub fn render_markdown(m: &Machine, r: &Recommendations) -> String {
    let mut s = String::from("# kei-machine-probe report\n\n");
    s.push_str(&md_section_os(&m.os));
    s.push_str(&md_section_arch(&m.arch));
    s.push_str(&md_section_memory(m));
    s.push_str(&md_section_gpu(&m.gpu));
    s.push_str(&md_section_tooling(&m.tooling));
    s.push_str(&md_section_recommendations(r));
    s
}

fn md_section_os(o: &crate::profile::OsInfo) -> String {
    format!(
        "## OS\n- family: `{:?}`\n- version: {}\n- build: {}\n\n",
        o.family,
        or_dash(&o.version),
        or_dash(&o.build)
    )
}

fn md_section_arch(a: &crate::profile::ArchInfo) -> String {
    format!(
        "## Arch\n- family: `{}`\n- brand: {}\n- model_id: {}\n- cores: {}\n\n",
        arch_family_str(&a.family),
        or_dash(&a.brand),
        or_dash(&a.model_id),
        a.cores
    )
}

fn md_section_memory(m: &Machine) -> String {
    format!(
        "## Memory\n- total_gb: {}\n- available_bytes: {}\n- pressure_pct: {}\n\n",
        m.memory.total_gb(),
        m.memory.available_bytes,
        m.memory.pressure_pct
    )
}

fn md_section_gpu(g: &GpuInfo) -> String {
    format!("## GPU\n- {}\n\n", gpu_str(g))
}

fn md_section_tooling(t: &crate::profile::ToolingInfo) -> String {
    format!(
        "## Tooling\n- ollama: {}\n- homebrew: {}\n- llama-server: {}\n\n",
        opt_or_dash(&t.ollama),
        opt_or_dash(&t.homebrew),
        opt_or_dash(&t.llama_cpp)
    )
}

fn md_section_recommendations(r: &Recommendations) -> String {
    let mut s = format!(
        "## Recommendations\n- capability: `{:?}`\n- max_model_b_params: {}\n- viable_backends: {:?}\n- rationale:\n",
        r.capability, r.max_model_b_params, r.viable_backends
    );
    for line in &r.rationale {
        s.push_str(&format!("  - {line}\n"));
    }
    s
}

pub fn render_plain(m: &Machine, r: &Recommendations) -> String {
    let mut s = plain_machine(m);
    s.push_str(&plain_rec(r));
    s
}

fn plain_machine(m: &Machine) -> String {
    let mut s = format!(
        "OS: {:?} {} ({})\n",
        m.os.family, m.os.version, or_dash(&m.os.build)
    );
    s.push_str(&format!(
        "Arch: {} | brand={} | cores={}\n",
        arch_family_str(&m.arch.family),
        or_dash(&m.arch.brand),
        m.arch.cores
    ));
    s.push_str(&format!(
        "Memory: {} GB total | {} bytes free | pressure {}%\n",
        m.memory.total_gb(),
        m.memory.available_bytes,
        m.memory.pressure_pct
    ));
    s.push_str(&format!("GPU: {}\n", gpu_str(&m.gpu)));
    s.push_str(&format!(
        "Tooling: ollama={} brew={} llama-server={}\n",
        opt_or_dash(&m.tooling.ollama),
        opt_or_dash(&m.tooling.homebrew),
        opt_or_dash(&m.tooling.llama_cpp)
    ));
    s
}

fn plain_rec(r: &Recommendations) -> String {
    let mut s = format!(
        "Capability: {:?} | max≈{}B | backends={:?}\n",
        r.capability, r.max_model_b_params, r.viable_backends
    );
    for line in &r.rationale {
        s.push_str(&format!("  - {line}\n"));
    }
    s
}

fn or_dash(s: &str) -> &str {
    if s.is_empty() { "-" } else { s }
}

fn opt_or_dash(s: &Option<String>) -> &str {
    match s {
        Some(v) if !v.is_empty() => v.as_str(),
        _ => "-",
    }
}

fn arch_family_str(f: &CpuFamily) -> String {
    match f {
        CpuFamily::AppleSilicon(v) => format!("AppleSilicon({:?})", v),
        CpuFamily::IntelX86_64 => "IntelX86_64".into(),
        CpuFamily::Other => "Other".into(),
    }
}

fn gpu_str(g: &GpuInfo) -> String {
    match g {
        GpuInfo::AppleIntegrated { cores, name } => {
            format!("AppleIntegrated cores={} name=\"{}\"", cores, name)
        }
        GpuInfo::IntelIntegrated { name } => format!("IntelIntegrated name=\"{}\"", name),
        GpuInfo::Discrete { name } => format!("Discrete name=\"{}\"", name),
        GpuInfo::None => "none".into(),
    }
}

#[allow(dead_code)]
fn _os_family_lint(_f: &OsFamily) {}
