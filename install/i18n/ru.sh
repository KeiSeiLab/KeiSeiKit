# shellcheck shell=bash
# i18n/ru.sh — русские строки. Source'ится после выбора языка.
# Welcome-баннер всегда EN — на момент его показа выбор ещё не сделан.

STR_WELCOME_TITLE="KeiSeiKit · Exobrain installer"
STR_WELCOME_TAGLINE="Portable Rust agent substrate for AI coding tools"

# Шаги мастера
STR_ONBOARDING_INTRO="Мастер первичной настройки (5 шагов)"
STR_PICK_LANGUAGE="Выберите язык интерфейса:"
STR_PICK_TRANSPORT="Выберите способ подключения:"
STR_PICK_PROVIDER="Выберите провайдера в группе"
STR_PICK_MODEL="Модель по умолчанию:"

# Описание транспортов
STR_TR_DIRECT_API="Прямой API провайдера (ключ)"
STR_TR_AWS_BEDROCK="AWS Bedrock (IAM/role)"
STR_TR_AZURE_OPENAI="Azure OpenAI (deployment+ключ)"
STR_TR_GOOGLE_VERTEX="Google Vertex AI (GCP)"
STR_TR_LOCAL="Локально (Ollama/MLX/LMStudio)"
STR_TR_PROXY="Прокси (LiteLLM/OpenRouter)"
STR_TR_SUBSCRIPTION="OAuth-подписка (ChatGPT)"

# Сбор ключей
STR_AUTH_INTRO="Аутентификация для"
STR_AUTH_PROMPT="Введите значения (Enter — оставить пустым, заполните позже)."
STR_AUTH_CURRENT_HINT="(текущее: <скрыто>)"

# Завершение
STR_DONE_TITLE="Первичная настройка завершена"
STR_DONE_CONFIG="конфиг:"
STR_DONE_SECRETS="секреты:"
STR_DONE_NEXT="Дальше: запустите ./install.sh или перезапустите этот скрипт для установки профиля"
