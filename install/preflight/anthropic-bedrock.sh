# shellcheck shell=bash
# preflight/anthropic-bedrock.sh — AWS CLI + Bedrock региональный доступ.

preflight_check_anthropic_bedrock() {
  if ! command -v aws >/dev/null 2>&1; then
    local cmd
    case "$(uname -s)" in
      Darwin) cmd="brew install awscli" ;;
      Linux)  cmd="curl 'https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip' -o /tmp/awscliv2.zip && unzip -q /tmp/awscliv2.zip -d /tmp && sudo /tmp/aws/install" ;;
      *)      cmd="см. https://aws.amazon.com/cli/" ;;
    esac
    preflight_offer_install "aws CLI" "$cmd" || return 1
  fi
  # Проверяем что credentials хоть как-то настроены (env, ~/.aws/credentials, IAM role).
  if ! aws sts get-caller-identity >/dev/null 2>&1; then
    echo "" >&2
    echo "  ⚠ AWS credentials не настроены." >&2
    echo "  Запустите: aws configure" >&2
    echo "  Или экспортируйте AWS_ACCESS_KEY_ID + AWS_SECRET_ACCESS_KEY + AWS_REGION." >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ aws CLI: $(aws --version 2>&1 | head -1)" >&2
  echo "  ✓ identity: $(aws sts get-caller-identity --query Arn --output text 2>&1)" >&2
  return 0
}
