# shellcheck shell=bash
# i18n/en.sh — English strings. Default before user picks language.

# Welcome banner (always EN, shown before language picker).
STR_WELCOME_TITLE="KeiSeiKit · Exobrain installer"
STR_WELCOME_TAGLINE="Portable Rust agent substrate for AI coding tools"

# Onboarding wizard steps
STR_ONBOARDING_INTRO="Onboarding wizard (5 steps)"
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
STR_TR_SUBSCRIPTION="OAuth subscription (ChatGPT)"

# Auth collection
STR_AUTH_INTRO="Auth for"
STR_AUTH_PROMPT="Enter values (Enter — leave empty, fill later)."
STR_AUTH_CURRENT_HINT="(current: <hidden>)"

# Completion
STR_DONE_TITLE="Onboarding complete"
STR_DONE_CONFIG="config:"
STR_DONE_SECRETS="secrets:"
STR_DONE_NEXT="Next: run ./install.sh or restart this script to apply profile"
