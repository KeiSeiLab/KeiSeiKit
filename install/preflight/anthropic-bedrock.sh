set -e
# shellcheck shell=bash
# preflight/anthropic-bedrock.sh — AWS CLI + Bedrock regional access.

preflight_check_anthropic_bedrock() {
  if ! command -v aws >/dev/null 2>&1; then
    local cmd
    case "$(uname -s)" in
      Darwin) cmd="brew install awscli" ;;
      Linux)  cmd="curl 'https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip' -o /tmp/awscliv2.zip && unzip -q /tmp/awscliv2.zip -d /tmp && sudo /tmp/aws/install" ;;
      *)      cmd="see https://aws.amazon.com/cli/" ;;
    esac
    preflight_offer_install "aws CLI" "$cmd" || return 1
  fi
  # Single call instead of two — saves ~1-3s on the success path.
  local identity_out identity_rc
  identity_out="$(aws sts get-caller-identity --output json 2>&1)"
  identity_rc=$?
  if [ $identity_rc -ne 0 ]; then
    echo "" >&2
    # Distinguish cred error from network/other by message text.
    if echo "$identity_out" | grep -qiE "UnauthorizedAccess|InvalidClientTokenId|ExpiredToken|signature|credential"; then
      echo "  ⚠ AWS credentials invalid." >&2
      echo "  Run: aws configure" >&2
      echo "  Or export AWS_ACCESS_KEY_ID + AWS_SECRET_ACCESS_KEY + AWS_REGION." >&2
    else
      echo "  ⚠ aws sts get-caller-identity failed (not credentials)." >&2
      echo "    raw: $identity_out" >&2
    fi
    echo "" >&2
    return 1
  fi
  echo "  ✓ aws CLI: $(aws --version 2>&1 | head -1)" >&2
  # Do not print the full ARN — may contain account-id (sensitive enum target).
  # Show only identity type (user/role/assumed-role) and user-name.
  local arn_short
  arn_short="$(echo "$identity_out" | sed -n 's/.*"Arn": *"\(arn:aws:[^:]*::\)[0-9]*\(:[^"]*\)".*/\1***\2/p')"
  [ -z "$arn_short" ] && arn_short="<masked>"
  echo "  ✓ identity: $arn_short" >&2
  return 0
}
