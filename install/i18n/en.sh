# shellcheck shell=bash
# i18n/en.sh — English strings. Default before user picks language.

# Welcome banner (always EN, shown before language picker).
STR_WELCOME_TITLE="KeiSeiKit · Exobrain installer"
STR_WELCOME_TAGLINE="Portable Rust agent substrate for AI coding tools"

# Onboarding wizard steps
STR_ONBOARDING_INTRO="Onboarding wizard (6 steps)"
STR_PICK_LANGUAGE="Choose interface language:"
STR_PICK_TRANSPORT="Choose connection transport:"
STR_PICK_PROVIDER="Choose provider within"
STR_PICK_MODEL="Default model:"

# Transport descriptions
STR_TR_DIRECT_API="Direct provider API (key)"
STR_TR_AWS_BEDROCK="AWS Bedrock (IAM/role)"
STR_TR_AZURE_OPENAI="Azure OpenAI (deployment+key)"
STR_TR_GOOGLE_VERTEX="Google Vertex AI (GCP)"
STR_TR_LOCAL="Local (Ollama/MLX/LMStudio)"
STR_TR_PROXY="Proxy (LiteLLM/OpenRouter)"
STR_TR_SUBSCRIPTION="Subscription login (Claude Code / ChatGPT — no API key)"

# Auth collection
STR_AUTH_INTRO="Auth for"
STR_AUTH_PROMPT="Enter values (Enter — leave empty, fill later)."
STR_AUTH_CURRENT_HINT="(current: <hidden>)"

# Completion
STR_DONE_TITLE="Onboarding complete"
STR_DONE_CONFIG="config:"
STR_DONE_SECRETS="secrets:"

# Profile menu (lib-menu.sh strings)
STR_MENU_TITLE="KeiSeiKit Installer"
STR_MENU_SUBSTRATE="Substrate baseline (always installed):"
STR_MENU_PROFILE_PROMPT="Choose install profile:"
STR_MENU_CONFIRM="Confirm selection?"

# Preflight warnings
STR_PREFLIGHT_FAILED="Preflight failed — provider may not work."
STR_PREFLIGHT_CONTINUE="Continue anyway? [y/N]"

# Wizard explanations + input validation
STR_PICK_INVALID="please type one of the numbers shown"
STR_EXPLAIN_TRANSPORT="How the agents reach the AI. subscription = log in with your plan, no API key (Claude Code is option 1); direct-api = your own API key. Press Enter for the default."
STR_EXPLAIN_PROVIDER="Which AI service. Option 1 is the recommended default — press Enter."
STR_EXPLAIN_MODEL="Default model the agents use. Option 1 is the recommended default — press Enter."

# Stack profile + hook-pack picker (step 6)
STR_PICK_STACK="Pick your stack profile (selects which hooks + agents install):"
STR_PICK_STACK_PROMPT="[1-5, default 1=minimal]: "
STR_STACK_MINIMAL="safety hooks + core agents only"
STR_STACK_WEB="TS/frontend agents + evidence, observability"
STR_STACK_ML="ML/data agents + evidence, observability, epistemic"
STR_STACK_SYSTEMS="Rust/Go agents + Rust-first + evidence, observability"
STR_STACK_MOBILE="Swift/Flutter agents + evidence, observability"
STR_PACK_INTRO="Optional discipline packs (safety is always on):"
STR_PACK_EVIDENCE="force evidence markers on numeric/cost claims"
STR_PACK_OBS="task timing, session dumps, agent telemetry"
STR_PACK_EPI="no-downgrade + alignment + recurrence reminders"
STR_PACK_ORCH="multi-agent fork logging + orchestrator git checks"
STR_PACK_GIT="block git push to github (for private-remote teams)"
STR_PACK_ENABLE="enable? [y/N]: "
