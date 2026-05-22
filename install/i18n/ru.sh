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
STR_TR_SUBSCRIPTION="Вход по подписке (Claude Code / ChatGPT — без API-ключа)"

# Сбор ключей
STR_AUTH_INTRO="Аутентификация для"
STR_AUTH_PROMPT="Введите значения (Enter — оставить пустым, заполните позже)."
STR_AUTH_CURRENT_HINT="(текущее: <скрыто>)"

# Завершение
STR_DONE_TITLE="Первичная настройка завершена"
STR_DONE_CONFIG="конфиг:"
STR_DONE_SECRETS="секреты:"

# Меню профилей (lib-menu.sh)
STR_MENU_TITLE="Установщик KeiSeiKit"
STR_MENU_SUBSTRATE="Базовая часть (ставится всегда):"
STR_MENU_PROFILE_PROMPT="Выберите профиль установки:"
STR_MENU_CONFIRM="Подтвердить выбор?"

# Preflight-предупреждения
STR_PREFLIGHT_FAILED="Preflight упал — провайдер может не работать."
STR_PREFLIGHT_CONTINUE="Продолжить всё равно? [y/N]"

# Пояснения мастера + валидация ввода
STR_PICK_INVALID="введите один из показанных номеров"
STR_EXPLAIN_TRANSPORT="Как агенты обращаются к ИИ. subscription = вход по подписке, без API-ключа (Claude Code — вариант 1); direct-api = свой API-ключ. Нажми Enter для варианта по умолчанию."
STR_EXPLAIN_PROVIDER="Какой ИИ-сервис. Вариант 1 — рекомендуемый по умолчанию, нажми Enter."
STR_EXPLAIN_MODEL="Модель, которую используют агенты. Вариант 1 — рекомендуемый по умолчанию, нажми Enter."
