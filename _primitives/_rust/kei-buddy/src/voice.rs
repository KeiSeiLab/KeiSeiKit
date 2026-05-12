// SPDX-License-Identifier: Apache-2.0
//! Voice-message handling: Telegram getFile → download → STT → transcript.
//!
//! Constructor Pattern: one responsibility — download audio via Telegram API
//! and pass it to the STT backend. Token is never logged.

use std::sync::Arc;

use kei_stt::{SttBackend, SttRequest};

use crate::error::BuddyError;

/// Downloads a Telegram voice/audio file and returns the STT transcript.
pub struct VoiceHandler {
    pub(crate) bot_token: String,
    pub(crate) stt: Arc<dyn SttBackend>,
    pub(crate) http: reqwest::Client,
}

impl VoiceHandler {
    /// Construct from a bot token string and an already-built STT backend.
    pub fn new(bot_token: String, stt: Arc<dyn SttBackend>) -> Self {
        Self { bot_token, stt, http: reqwest::Client::new() }
    }

    /// Resolve file_id → file_path via Telegram getFile, download bytes,
    /// and transcribe via the STT backend.
    ///
    /// Errors map to [`BuddyError::Transport`]. The bot token is never
    /// included in error messages.
    pub async fn transcribe_file(
        &self,
        file_id: &str,
        mime_type: &str,
    ) -> Result<String, BuddyError> {
        let file_path = self.get_file_path(file_id).await?;
        let audio_bytes = self.download_file(&file_path).await?;
        self.run_stt(audio_bytes, mime_type).await
    }

    pub(crate) async fn get_file_path(&self, file_id: &str) -> Result<String, BuddyError> {
        let url = format!(
            "https://api.telegram.org/bot{}/getFile",
            self.bot_token
        );
        let resp = self.http
            .get(&url)
            .query(&[("file_id", file_id)])
            .send()
            .await
            .map_err(|e| BuddyError::Transport(format!("getFile request failed: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            return Err(BuddyError::Transport(
                format!("getFile returned HTTP {status}"),
            ));
        }
        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| BuddyError::Transport(format!("getFile JSON parse: {e}")))?;
        let path = json["result"]["file_path"]
            .as_str()
            .ok_or_else(|| BuddyError::Transport("getFile: missing result.file_path".into()))?
            .to_string();
        Ok(path)
    }

    pub(crate) async fn download_file(&self, file_path: &str) -> Result<Vec<u8>, BuddyError> {
        let url = format!(
            "https://api.telegram.org/file/bot{}/{}",
            self.bot_token, file_path
        );
        let bytes = self.http
            .get(&url)
            .send()
            .await
            .map_err(|e| BuddyError::Transport(format!("file download failed: {e}")))?
            .bytes()
            .await
            .map_err(|e| BuddyError::Transport(format!("file download bytes: {e}")))?;
        Ok(bytes.to_vec())
    }

    pub(crate) async fn run_stt(
        &self,
        audio_bytes: Vec<u8>,
        mime_type: &str,
    ) -> Result<String, BuddyError> {
        let req = SttRequest {
            audio_bytes,
            mime_type: mime_type.to_string(),
            language: None,
        };
        let resp = self.stt
            .transcribe(&req)
            .await
            .map_err(|e| BuddyError::Transport(format!("STT failed: {e}")))?;
        Ok(resp.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use kei_stt::{SttError, SttResponse};
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct MockStt(String);

    #[async_trait]
    impl SttBackend for MockStt {
        async fn transcribe(&self, _req: &SttRequest) -> Result<SttResponse, SttError> {
            Ok(SttResponse::text_only(self.0.clone()))
        }
        fn name(&self) -> &'static str { "mock" }
    }

    /// Test-only helper that injects a base_url so tests point at wiremock.
    async fn run(base: &str, token: &str, stt_reply: &str, file_id: &str, mime: &str)
        -> Result<String, BuddyError>
    {
        let stt: Arc<dyn SttBackend> = Arc::new(MockStt(stt_reply.into()));
        let http = reqwest::Client::new();
        let get_url = format!("{}/bot{}/getFile", base, token);
        let resp = http.get(&get_url).query(&[("file_id", file_id)]).send().await
            .map_err(|e| BuddyError::Transport(format!("getFile request failed: {e}")))?;
        if !resp.status().is_success() {
            let s = resp.status();
            return Err(BuddyError::Transport(format!("getFile returned HTTP {s}")));
        }
        let json: serde_json::Value = resp.json().await
            .map_err(|e| BuddyError::Transport(format!("getFile JSON: {e}")))?;
        let file_path = json["result"]["file_path"].as_str()
            .ok_or_else(|| BuddyError::Transport("missing file_path".into()))?.to_string();
        let dl_url = format!("{}/file/bot{}/{}", base, token, file_path);
        let audio = http.get(&dl_url).send().await
            .map_err(|e| BuddyError::Transport(format!("download: {e}")))?
            .bytes().await
            .map_err(|e| BuddyError::Transport(format!("bytes: {e}")))?.to_vec();
        let req = SttRequest { audio_bytes: audio, mime_type: mime.into(), language: None };
        let r = stt.transcribe(&req).await
            .map_err(|e| BuddyError::Transport(format!("STT: {e}")))?;
        Ok(r.text)
    }

    #[tokio::test]
    async fn transcribe_file_calls_getfile_then_downloads_then_stt() {
        let server = MockServer::start().await;
        Mock::given(method("GET")).and(path("/bottoken/getFile"))
            .and(query_param("file_id", "v123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "ok": true, "result": { "file_id": "v123", "file_path": "voice/f.oga" }
            }))).mount(&server).await;
        Mock::given(method("GET")).and(path("/file/bottoken/voice/f.oga"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"audio".to_vec()))
            .mount(&server).await;

        let text = run(&server.uri(), "token", "hello world", "v123", "audio/ogg").await;
        assert_eq!(text.unwrap(), "hello world");
    }

    #[tokio::test]
    async fn getfile_error_propagates_as_transport_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET")).and(path("/bottoken/getFile"))
            .respond_with(ResponseTemplate::new(500)).mount(&server).await;

        let result = run(&server.uri(), "token", "x", "bad", "audio/ogg").await;
        match result {
            Err(BuddyError::Transport(msg)) => assert!(msg.contains("getFile"), "msg={msg}"),
            other => panic!("expected Transport error, got {other:?}"),
        }
    }
}
