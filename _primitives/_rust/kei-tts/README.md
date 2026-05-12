# kei-tts

Text-to-speech abstraction crate with 4 backends selected at runtime via
`KEI_TTS_BACKEND`. Default backend is **piper** (local, free, zero latency).

## Backend matrix

| Backend     | Feature flag  | Cost        | Latency    | Quality   | Language coverage |
|-------------|---------------|-------------|------------|-----------|-------------------|
| `piper`     | `piper`       | Free        | ~50–200 ms | Good      | 20+ language packs |
| `elevenlabs`| `elevenlabs`  | ~$0.30/1k ch| 300–600 ms | Excellent | 30+ languages     |
| `openai`    | `openai`      | ~$0.015/1k ch| 200–500 ms| Very good | 50+ languages     |
| `google`    | `google`      | ~$4/1M ch   | 200–400 ms | Very good | 40+ languages     |

## Environment variables

| Variable                | Backend     | Required | Description                        |
|-------------------------|-------------|----------|------------------------------------|
| `KEI_TTS_BACKEND`       | all         | No       | `piper` (default) / `elevenlabs` / `openai` / `google` |
| `ELEVENLABS_API_KEY`    | elevenlabs  | Yes      | ElevenLabs API key                 |
| `OPENAI_API_KEY`        | openai      | Yes      | OpenAI API key                     |
| `KEI_TTS_OPENAI_MODEL`  | openai      | No       | `tts-1` (default) or `tts-1-hd`   |
| `GOOGLE_TTS_API_KEY`    | google      | Yes      | Google Cloud API key               |
| `KEI_TTS_PIPER_MODEL`   | piper       | Yes      | Path to `.onnx` piper model file   |
| `KEI_TTS_PIPER_BINARY`  | piper       | No       | Path to `piper-tts` (default: PATH)|

## Usage

```toml
[dependencies]
kei-tts = { path = "../kei-tts", features = ["piper"] }
```

```rust
#[tokio::main]
async fn main() -> Result<(), kei_tts::TtsError> {
    let backend = kei_tts::from_env()?;
    let req = kei_tts::TtsRequest::new("Hello, world!");
    let resp = backend.synth(&req).await?;
    std::fs::write("out.mp3", &resp.audio_bytes).ok();
    println!("synthesised {} bytes via {}", resp.audio_bytes.len(), backend.name());
    Ok(())
}
```

## Compile-time features

```toml
# All backends:
kei-tts = { features = ["all-backends"] }
# Cloud only, no piper:
kei-tts = { features = ["elevenlabs", "openai", "google"], default-features = false }
```
