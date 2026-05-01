//! Dispatch — map a parsed `Cmd` to its concrete behaviour.
//!
//! Each handler is ≤30 LOC. They share a tiny set of helpers
//! (json print, error print) so the main.rs entry stays trivial.

use kei_llm_llamacpp::cli::Cmd;
use kei_llm_llamacpp::{
    discover, generate, generate_stream, list_models, models::default_dirs, start_server,
    BinPaths, Error, GenerateOpts, RealRunner, ServerInfo, ServerOpts, KEI_WRAPPER_VERSION,
};
use serde::Serialize;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Serialize)]
struct VersionInfo<'a> {
    llama_cli_version: Option<String>,
    llama_server_version: Option<String>,
    kei_wrapper_version: &'a str,
}

pub async fn run(cmd: Cmd) -> ExitCode {
    match cmd {
        Cmd::Probe => run_probe().await,
        Cmd::Models { dir } => run_models(dir).await,
        Cmd::Generate { model, prompt, max_tokens, temperature, stream } => {
            run_generate(model, prompt, max_tokens, temperature, stream).await
        }
        Cmd::Server { model, host, port } => run_server(model, host, port).await,
        Cmd::Version => run_version().await,
    }
}

async fn run_probe() -> ExitCode {
    let runner = RealRunner;
    match discover(&runner).await {
        Ok(paths) => {
            emit_json(&paths);
            if paths.any_found() { ExitCode::SUCCESS } else { ExitCode::from(2) }
        }
        Err(e) => err_exit(&e),
    }
}

async fn run_models(dir: Option<PathBuf>) -> ExitCode {
    let dirs: Vec<PathBuf> = match dir {
        Some(d) => vec![d],
        None => default_dirs(),
    };
    let mut all = Vec::new();
    for d in dirs {
        match list_models(&d) {
            Ok(mut v) => all.append(&mut v),
            Err(e) => return err_exit(&e),
        }
    }
    emit_json(&all);
    ExitCode::SUCCESS
}

async fn run_generate(
    model: PathBuf,
    prompt: String,
    max_tokens: u32,
    temperature: Option<f32>,
    stream: bool,
) -> ExitCode {
    let runner = RealRunner;
    let opts = GenerateOpts { max_tokens, temperature };
    if stream {
        run_generate_stream(&runner, &model, &prompt, &opts).await
    } else {
        run_generate_oneshot(&runner, &model, &prompt, &opts).await
    }
}

async fn run_generate_stream(
    runner: &RealRunner,
    model: &PathBuf,
    prompt: &str,
    opts: &GenerateOpts,
) -> ExitCode {
    match generate_stream(runner, "llama-cli", model, prompt, opts).await {
        Ok(chunks) => {
            for c in chunks {
                emit_ndjson(&c);
            }
            ExitCode::SUCCESS
        }
        Err(e) => err_exit(&e),
    }
}

async fn run_generate_oneshot(
    runner: &RealRunner,
    model: &PathBuf,
    prompt: &str,
    opts: &GenerateOpts,
) -> ExitCode {
    match generate(runner, "llama-cli", model, prompt, opts).await {
        Ok(r) => {
            emit_json(&r);
            ExitCode::SUCCESS
        }
        Err(e) => err_exit(&e),
    }
}

async fn run_server(model: PathBuf, host: String, port: u16) -> ExitCode {
    let runner = RealRunner;
    let opts = ServerOpts { host: host.clone(), port };
    let handle = match start_server(&runner, "llama-server", &model, &opts).await {
        Ok(h) => h,
        Err(e) => return err_exit(&e),
    };
    let info = ServerInfo {
        pid: handle.pid,
        port: handle.port,
        host: opts.host.clone(),
        openai_compat_url: format!("http://{}:{}/v1", opts.host, opts.port),
    };
    emit_json(&info);
    // Hold the handle until SIGINT; Drop kills the child.
    let _ = tokio::signal::ctrl_c().await;
    drop(handle);
    ExitCode::SUCCESS
}

async fn run_version() -> ExitCode {
    let runner = RealRunner;
    let bp: BinPaths = discover(&runner).await.unwrap_or_default();
    let info = VersionInfo {
        llama_cli_version: bp.llama_cli.as_ref().and(bp.version.clone()),
        llama_server_version: bp.llama_server.as_ref().and(bp.version),
        kei_wrapper_version: KEI_WRAPPER_VERSION,
    };
    emit_json(&info);
    ExitCode::SUCCESS
}

fn emit_json<T: Serialize>(v: &T) {
    match serde_json::to_string_pretty(v) {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("kei-llm-llamacpp: serialize: {e}"),
    }
}

fn emit_ndjson<T: Serialize>(v: &T) {
    match serde_json::to_string(v) {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("kei-llm-llamacpp: serialize: {e}"),
    }
}

fn err_exit(e: &Error) -> ExitCode {
    eprintln!("kei-llm-llamacpp: {e}");
    ExitCode::from(e.exit_code())
}
