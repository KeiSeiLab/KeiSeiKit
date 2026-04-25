//! Schema types for pet.toml.
//!
//! Enums use `#[serde(rename_all = "kebab-case")]` to match the TOML wire
//! format (e.g. `"mirror-user"`). Optional fields use `Option<T>` and are
//! omitted on serialize when `None`. Arrays default to `Vec::new()`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PetManifest {
    /// Schema version. Must be `1` for this crate.
    pub schema: u32,

    pub identity: Identity,
    pub voice: Voice,
    pub edge: Edge,

    #[serde(default)]
    pub appearance: Option<Appearance>,

    #[serde(default)]
    pub room: Option<Room>,

    #[serde(default)]
    pub privacy: Option<Privacy>,

    #[serde(default, rename = "interests")]
    pub interests: Vec<Interest>,

    #[serde(default, rename = "routines")]
    pub routines: Vec<Routine>,

    pub forbidden: Forbidden,

    pub meta: Meta,
}

// ─────────────────────────────── identity ────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Identity {
    pub pet_name: String,
    pub user_name: String,
    pub addressing: Addressing,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Addressing {
    ByName,
    Nickname,
    Formal,
    NoAddress,
}

// ───────────────────────────────── voice ─────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Voice {
    pub tone_primary: Tone,
    #[serde(default)]
    pub tone_secondary: Vec<Tone>,
    pub humor_style: HumorStyle,
    pub humor_frequency: HumorFrequency,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Tone {
    Warm,
    Dry,
    Sarcastic,
    Neutral,
    Supportive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HumorStyle {
    None,
    Puns,
    Dark,
    Absurd,
    #[serde(rename = "engineering-meta")]
    EngineeringMeta,
    #[serde(rename = "dark+meta")]
    DarkMeta,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HumorFrequency {
    Rare,
    Medium,
    Often,
}

// ────────────────────────────────── edge ─────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Edge {
    pub profanity: Profanity,
    #[serde(default)]
    pub profanity_languages: Vec<String>,
    pub directness: Directness,
    pub initiative: Initiative,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Profanity {
    Never,
    Accent,
    Casual,
    MirrorUser,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Directness {
    Soft,
    Balanced,
    Hard,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Initiative {
    Wait,
    Suggest,
    TapOnShoulder,
}

// ─────────────────────────────── appearance ──────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Appearance {
    pub base_shape: BaseShape,
    pub size: Size,
    pub color_primary: String,
    pub color_secondary: String,
    pub eyes: Eyes,
    pub expression: Expression,
    #[serde(default)]
    pub accessories: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BaseShape {
    Cat,
    Dog,
    Blob,
    Owl,
    Bot,
    Capybara,
    Dragon,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Size {
    Tiny,
    Small,
    Medium,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Eyes {
    Round,
    Sleepy,
    Sharp,
    Dots,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Expression {
    Focused,
    Curious,
    Grumpy,
    Happy,
    Neutral,
}

// ────────────────────────────────── room ─────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Room {
    pub theme: RoomTheme,
    pub lighting: Lighting,
    #[serde(default)]
    pub decor: Vec<String>,
    #[serde(default = "default_true")]
    pub time_sync: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RoomTheme {
    Study,
    Nature,
    Cyberpunk,
    Minimalist,
    Cozy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Lighting {
    Warm,
    Cool,
    Natural,
    Moody,
}

// ──────────────────────────────── privacy ────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Privacy {
    #[serde(default = "default_true")]
    pub public_profile: bool,
    #[serde(default = "default_true")]
    pub publish_allowed: bool,
    #[serde(default)]
    pub share_dreams: bool,
    #[serde(default = "default_summary")]
    pub share_garden: GardenVisibility,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GardenVisibility {
    Full,
    Summary,
    None,
}

// ─────────────────────────────── interests ───────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Interest {
    pub topic: String,
    pub depth: Depth,
    pub freshness: Freshness,
    #[serde(default)]
    pub vault_path: String,
    #[serde(default)]
    pub last_refresh: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Depth {
    Shallow,
    Intermediate,
    Expert,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Freshness {
    Daily,
    Weekly,
    Monthly,
    OnDemand,
}

// ──────────────────────────────── routines ───────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Routine {
    pub kind: RoutineKind,
    pub schedule: String,
    pub template: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RoutineKind {
    MorningDigest,
    EveningRecap,
    WeeklyDeepdive,
    IdleCheck,
    ErrorSpike,
    Custom,
}

// ─────────────────────────────── forbidden ───────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Forbidden {
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub tone_patterns: Vec<String>,
}

// ────────────────────────────────── meta ─────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Meta {
    pub schema_version_written_by: String,
    pub created_at: String,
    pub last_tuned: String,
    #[serde(default)]
    pub tune_count: u32,
}

// ──────────────────────────────── helpers ────────────────────────────────

fn default_true() -> bool { true }
fn default_summary() -> GardenVisibility { GardenVisibility::Summary }
