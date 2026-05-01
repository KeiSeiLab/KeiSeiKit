//! Bridge between a validated `PetManifest` and an agent-spawn prompt.
//!
//! Used by the `/spawn-agent` skill's pet-overlay phase: compose the final
//! system prompt as `base_prompt` ++ (optional persona overlay) ++ `task_body`.
//! No I/O here — pure string composition. Deterministic.
//!
//! Scope boundary (see crate root): this module renders prompts for any
//! agent runtime. It imports nothing from sibling research-grade projects.

use crate::overlay::system_prompt;
use crate::schema::PetManifest;

/// Everything the bridge needs to compose one spawn prompt.
///
/// `base_prompt` is the composed capabilities string from the agent runtime
/// (e.g. `kei-agent-runtime`). `pet_manifest` is `None` when the user opted
/// out of a persona overlay during the spawn wizard. `task_body` is the
/// verbatim task description the orchestrator wants the agent to execute.
#[derive(Debug, Clone)]
pub struct AgentSpawnRequest {
    pub role: String,
    pub pet_manifest: Option<PetManifest>,
    pub task_body: String,
    pub base_prompt: String,
}

/// Compose the full prompt: base + persona overlay (if any) + task body.
///
/// Layout:
///   <base_prompt>
///   \n\n---\n\n
///   [## Persona overlay\n\n<overlay>\n\n---\n\n]   (only when manifest set)
///   <task_body>
pub fn compose_prompt_with_pet(req: &AgentSpawnRequest) -> String {
    let mut out = String::with_capacity(
        req.base_prompt.len() + req.task_body.len() + 1024,
    );
    out.push_str(&req.base_prompt);
    out.push_str("\n\n---\n\n");
    if let Some(m) = &req.pet_manifest {
        out.push_str("## Persona overlay\n\n");
        out.push_str(&system_prompt(m));
        out.push_str("\n\n---\n\n");
    }
    out.push_str(&req.task_body);
    out
}
