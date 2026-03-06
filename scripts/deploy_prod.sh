#!/usr/bin/env sh
set -eu

STACK_DIR="${STACK_DIR:-/opt/uamappers/prod}"
PROJECT_NAME="${COMPOSE_PROJECT_NAME:-uamappers-prod}"

cd "$STACK_DIR"

if [ ! -f "compose.yml" ]; then
  echo "compose.yml was not found in $STACK_DIR"
  exit 1
fi

if [ ! -f "app.env" ]; then
  echo "app.env was not found in $STACK_DIR"
  exit 1
fi

# shellcheck disable=SC1091
set -a
. ./app.env
set +a

if [ -z "${UAMAPPERS_API_IMAGE:-}" ]; then
  echo "UAMAPPERS_API_IMAGE is required"
  exit 1
fi

if [ -z "${UAMAPPERS_WEB_IMAGE:-}" ]; then
  echo "UAMAPPERS_WEB_IMAGE is required"
  exit 1
fi

export COMPOSE_PROJECT_NAME="$PROJECT_NAME"

# Pull only target runtime images.
docker compose -f compose.yml pull api web

# Keep DB up first.
docker compose -f compose.yml up -d postgres

# Apply DB migrations before promoting runtime services.
docker compose -f compose.yml run --rm migrator up

docker rm -f uamappers-api-candidate uamappers-web-candidate >/dev/null 2>&1 || true

# Preflight candidate API startup before promotion.
docker run -d --name uamappers-api-candidate \
  --network uamappers-prod-internal \
  --env-file app.env \
  "$UAMAPPERS_API_IMAGE" >/dev/null

i=0
until docker run --rm --network uamappers-prod-internal curlimages/curl:8.7.1 \
  -fsS http://uamappers-api-candidate:8080/system/health >/dev/null 2>&1; do
  i=$((i + 1))
  if [ "$i" -ge 30 ]; then
    echo "api candidate failed health check"
    docker rm -f uamappers-api-candidate >/dev/null 2>&1 || true
    exit 1
  fi
  sleep 2
done

# Preflight candidate Web startup before promotion.
docker run -d --name uamappers-web-candidate \
  --network uamappers-prod-internal \
  "$UAMAPPERS_WEB_IMAGE" >/dev/null

i=0
until docker run --rm --network uamappers-prod-internal curlimages/curl:8.7.1 \
  -fsS http://uamappers-web-candidate:80/ >/dev/null 2>&1; do
  i=$((i + 1))
  if [ "$i" -ge 30 ]; then
    echo "web candidate failed health check"
    docker rm -f uamappers-web-candidate uamappers-api-candidate >/dev/null 2>&1 || true
    exit 1
  fi
  sleep 2
done

docker rm -f uamappers-web-candidate uamappers-api-candidate >/dev/null 2>&1 || true

# Promote only after successful preflight startup checks.
docker compose -f compose.yml up -d --wait api web

# Internal smoke checks after promotion.
docker run --rm --network uamappers-prod-internal curlimages/curl:8.7.1 \
  -fsS http://api:8080/system/health >/dev/null

docker run --rm --network uamappers-prod-internal curlimages/curl:8.7.1 \
  -fsS http://web:80/ >/dev/null

echo "prod deploy finished successfully"
