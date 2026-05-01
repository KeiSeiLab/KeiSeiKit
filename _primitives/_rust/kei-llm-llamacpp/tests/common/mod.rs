//! Shared MockRunner for integration tests.
//!
//! Each test pushes canned `Behaviour` entries onto a queue; the next
//! `Runner::run*` call pops the head entry. Tests assert side-effects
//! (e.g. `last_args`) plus return values.
//!
//! `#![allow(dead_code)]` because each integration-test binary compiles
//! `common/mod.rs` independently and may use only a subset of the
//! exposed surface.

#![allow(dead_code)]

use kei_llm_llamacpp::error::{Error, Result};
use kei_llm_llamacpp::runner::{RunOutput, Runner, ServerHandle};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

/// One canned response from the mock.
#[derive(Debug, Clone)]
pub enum Behaviour {
    /// `run` returns this RunOutput.
    Run(RunOutput),
    /// `run` returns Err(BinaryNotFound { name }) — simulates "not found".
    BinaryMissing { name: String },
    /// `run_stream` returns these lines.
    Stream(Vec<String>),
    /// `spawn_server` succeeds; the test holds the kill_flag handle.
    ServerOk { pid: u32, kill_flag: Arc<Mutex<bool>> },
}

/// Canned-response runner.
pub struct MockRunner {
    queue: Mutex<Vec<Behaviour>>,
    pub last_bin: Mutex<Option<String>>,
    pub last_args: Mutex<Option<Vec<String>>>,
}

impl MockRunner {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(Vec::new()),
            last_bin: Mutex::new(None),
            last_args: Mutex::new(None),
        }
    }

    pub fn push(&self, b: Behaviour) {
        self.queue.lock().unwrap().push(b);
    }

    fn pop(&self) -> Option<Behaviour> {
        let mut q = self.queue.lock().unwrap();
        if q.is_empty() { None } else { Some(q.remove(0)) }
    }

    fn record(&self, bin: &str, args: &[String]) {
        *self.last_bin.lock().unwrap() = Some(bin.to_string());
        *self.last_args.lock().unwrap() = Some(args.to_vec());
    }
}

type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

impl Runner for MockRunner {
    fn run<'a>(&'a self, bin: &'a str, args: &'a [String]) -> BoxFut<'a, Result<RunOutput>> {
        Box::pin(async move {
            self.record(bin, args);
            match self.pop() {
                Some(Behaviour::Run(r)) => Ok(r),
                Some(Behaviour::BinaryMissing { name }) => Err(Error::BinaryNotFound { name }),
                Some(other) => Err(Error::ParseFailed {
                    reason: format!("mock got non-Run behaviour: {other:?}"),
                }),
                None => Err(Error::ParseFailed { reason: "mock queue empty".into() }),
            }
        })
    }

    fn run_stream<'a>(
        &'a self,
        bin: &'a str,
        args: &'a [String],
    ) -> BoxFut<'a, Result<Vec<String>>> {
        Box::pin(async move {
            self.record(bin, args);
            match self.pop() {
                Some(Behaviour::Stream(lines)) => Ok(lines),
                _ => Err(Error::ParseFailed { reason: "mock queue empty/wrong".into() }),
            }
        })
    }

    fn spawn_server<'a>(
        &'a self,
        bin: &'a str,
        args: &'a [String],
        port: u16,
    ) -> BoxFut<'a, Result<ServerHandle>> {
        Box::pin(async move {
            self.record(bin, args);
            match self.pop() {
                Some(Behaviour::ServerOk { pid, kill_flag }) => {
                    Ok(ServerHandle::mock(pid, port, kill_flag))
                }
                _ => Err(Error::ParseFailed { reason: "mock queue empty/wrong".into() }),
            }
        })
    }
}
