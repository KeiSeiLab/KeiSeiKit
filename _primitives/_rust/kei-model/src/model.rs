//! Core types: `Model`, `Provider`, `Capability`, `Status`.
//!
//! Constructor Pattern: each enum is its own type with a single responsibility
//! (rendering / parsing / matching). Pricing lives in a sibling module so this
//! file stays focused on identity + capability shape.

use serde::{Deserialize, Serialize};

use crate::pricing::Pricing;

/// One row in the model catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    /// Canonical id, e.g. "claude-opus-4-7".
    pub id: String,
    pub provider: Provider,
    pub display_name: String,
    pub context_tokens: u32,
    #[serde(default)]
    pub capabilities: Vec<Capability>,
    pub pricing: Pricing,
    pub status: Status,
    #[serde(default)]
    pub role_tags: Vec<String>,
    /// Next model_id in the fallback chain. Empty string = chain terminates.
    #[serde(default)]
    pub fallback: String,
    #[serde(default)]
    pub notes: Option<String>,
}

impl Model {
    /// True if this model has every capability in `caps`.
    pub fn has_all_caps(&self, caps: &[Capability]) -> bool {
        caps.iter().all(|c| self.capabilities.contains(c))
    }

    /// True if `tag` matches any role tag (case-sensitive, exact match).
    pub fn has_role(&self, tag: &str) -> bool {
        self.role_tags.iter().any(|t| t == tag)
    }

    /// `Some(id)` if a non-empty fallback target is set, else `None`.
    pub fn fallback_target(&self) -> Option<&str> {
        if self.fallback.is_empty() {
            None
        } else {
            Some(&self.fallback)
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Anthropic,
    Openai,
    Kimi,
    Mistral,
    Deepseek,
    Local,
    /// Google: Gemini text-LLM family + image generation (nano-banana CLI
    /// consumes Gemini 3.1 Flash Image / Gemini 3 Pro Image).
    Google,
    /// fal.ai — image / video / 3D generation aggregator. Hosts Flux,
    /// Kling O3, Veo 3, Ideogram, Recraft, etc.
    Fal,
    /// ElevenLabs — text-to-speech and voice cloning.
    Elevenlabs,
}

impl Provider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Provider::Anthropic => "anthropic",
            Provider::Openai => "openai",
            Provider::Kimi => "kimi",
            Provider::Mistral => "mistral",
            Provider::Deepseek => "deepseek",
            Provider::Local => "local",
            Provider::Google => "google",
            Provider::Fal => "fal",
            Provider::Elevenlabs => "elevenlabs",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "anthropic" => Some(Provider::Anthropic),
            "openai" => Some(Provider::Openai),
            "kimi" => Some(Provider::Kimi),
            "mistral" => Some(Provider::Mistral),
            "deepseek" => Some(Provider::Deepseek),
            "local" => Some(Provider::Local),
            "google" => Some(Provider::Google),
            "fal" => Some(Provider::Fal),
            "elevenlabs" => Some(Provider::Elevenlabs),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Capability {
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "vision")]
    Vision,
    #[serde(rename = "streaming")]
    Streaming,
    #[serde(rename = "function-call")]
    FunctionCall,
    #[serde(rename = "long-context-200k")]
    LongContext200k,
    #[serde(rename = "long-context-1m")]
    LongContext1m,
    #[serde(rename = "system-prompt")]
    SystemPrompt,
    // Generation capabilities (image / video / voice). Pricing for these
    // is per-image / per-second / per-1k-chars rather than per-mtok; the
    // existing Pricing struct stores the unit price in
    // `output_per_mtok_micro` and the unit semantics live in `notes`.
    #[serde(rename = "image-gen")]
    ImageGen,
    #[serde(rename = "text-to-image")]
    TextToImage,
    #[serde(rename = "image-edit")]
    ImageEdit,
    #[serde(rename = "video-gen")]
    VideoGen,
    #[serde(rename = "text-to-video")]
    TextToVideo,
    #[serde(rename = "image-to-video")]
    ImageToVideo,
    #[serde(rename = "voice-gen")]
    VoiceGen,
    #[serde(rename = "text-to-speech")]
    TextToSpeech,
    #[serde(rename = "voice-clone")]
    VoiceClone,
}

impl Capability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Capability::Code => "code",
            Capability::Vision => "vision",
            Capability::Streaming => "streaming",
            Capability::FunctionCall => "function-call",
            Capability::LongContext200k => "long-context-200k",
            Capability::LongContext1m => "long-context-1m",
            Capability::SystemPrompt => "system-prompt",
            Capability::ImageGen => "image-gen",
            Capability::TextToImage => "text-to-image",
            Capability::ImageEdit => "image-edit",
            Capability::VideoGen => "video-gen",
            Capability::TextToVideo => "text-to-video",
            Capability::ImageToVideo => "image-to-video",
            Capability::VoiceGen => "voice-gen",
            Capability::TextToSpeech => "text-to-speech",
            Capability::VoiceClone => "voice-clone",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "code" => Some(Capability::Code),
            "vision" => Some(Capability::Vision),
            "streaming" => Some(Capability::Streaming),
            "function-call" => Some(Capability::FunctionCall),
            "long-context-200k" => Some(Capability::LongContext200k),
            "long-context-1m" => Some(Capability::LongContext1m),
            "system-prompt" => Some(Capability::SystemPrompt),
            "image-gen" => Some(Capability::ImageGen),
            "text-to-image" => Some(Capability::TextToImage),
            "image-edit" => Some(Capability::ImageEdit),
            "video-gen" => Some(Capability::VideoGen),
            "text-to-video" => Some(Capability::TextToVideo),
            "image-to-video" => Some(Capability::ImageToVideo),
            "voice-gen" => Some(Capability::VoiceGen),
            "text-to-speech" => Some(Capability::TextToSpeech),
            "voice-clone" => Some(Capability::VoiceClone),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Deprecated,
    Preview,
    Beta,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Active => "active",
            Status::Deprecated => "deprecated",
            Status::Preview => "preview",
            Status::Beta => "beta",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "active" => Some(Status::Active),
            "deprecated" => Some(Status::Deprecated),
            "preview" => Some(Status::Preview),
            "beta" => Some(Status::Beta),
            _ => None,
        }
    }
}
